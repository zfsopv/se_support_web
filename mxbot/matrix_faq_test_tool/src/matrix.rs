use crate::models::{ReplyCapture, RoomSummary};
use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, Utc};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::Deserialize;
use serde_json::json;
use std::cmp::min;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use urlencoding::encode;
use uuid::Uuid;

const INTERIM_REPLY_MAX_CHARS: usize = 120;
const FINAL_REPLY_IDLE_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Clone)]
pub struct MatrixClient {
    homeserver: String,
    user_id: String,
    http: reqwest::Client,
    access_token: String,
}

#[derive(Debug, Deserialize)]
struct LoginResponse {
    access_token: String,
    user_id: String,
}

#[derive(Debug, Deserialize)]
struct JoinedRoomsResponse {
    joined_rooms: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RoomNameContent {
    name: String,
}

#[derive(Debug, Deserialize)]
struct SendResponse {
    event_id: String,
}

#[derive(Debug, Deserialize)]
struct SyncResponse {
    next_batch: String,
    #[serde(default)]
    rooms: RoomsSection,
}

#[derive(Debug, Default, Deserialize)]
struct RoomsSection {
    #[serde(default)]
    join: std::collections::HashMap<String, JoinedRoom>,
}

#[derive(Debug, Default, Deserialize)]
struct JoinedRoom {
    #[serde(default)]
    timeline: Timeline,
}

#[derive(Debug, Default, Deserialize)]
struct Timeline {
    #[serde(default)]
    events: Vec<TimelineEvent>,
}

#[derive(Debug, Deserialize)]
struct TimelineEvent {
    #[serde(default)]
    event_id: Option<String>,
    #[serde(default)]
    sender: Option<String>,
    #[serde(default)]
    origin_server_ts: Option<i64>,
    #[serde(default)]
    content: MessageContent,
    #[serde(rename = "type")]
    event_type: String,
}

#[derive(Debug, Default, Deserialize)]
struct MessageContent {
    #[serde(default)]
    body: Option<String>,
    #[serde(default)]
    msgtype: Option<String>,
}

impl MatrixClient {
    pub async fn login(homeserver: &str, username: &str, password: &str) -> Result<Self> {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .context("Failed to build HTTP client")?;

        let url = format!(
            "{}/_matrix/client/v3/login",
            homeserver.trim_end_matches('/')
        );
        let body = json!({
            "type": "m.login.password",
            "identifier": {
                "type": "m.id.user",
                "user": username
            },
            "password": password,
        });

        let response = http
            .post(url)
            .header(CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await
            .context("Matrix login request failed")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Matrix login failed with status {status}: {text}"));
        }

        let login: LoginResponse = response.json().await.context("Invalid login response")?;

        Ok(Self {
            homeserver: homeserver.trim_end_matches('/').to_string(),
            user_id: login.user_id,
            http,
            access_token: login.access_token,
        })
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub async fn list_joined_rooms(&self) -> Result<Vec<RoomSummary>> {
        let url = format!("{}/_matrix/client/v3/joined_rooms", self.homeserver);
        let response = self
            .http
            .get(url)
            .headers(self.auth_headers()?)
            .send()
            .await
            .context("Failed to query joined rooms")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Failed to list joined rooms: {status} {text}"));
        }

        let joined: JoinedRoomsResponse = response
            .json()
            .await
            .context("Invalid joined rooms response")?;
        let mut rooms = Vec::new();
        for room_id in joined.joined_rooms {
            let display_name = self
                .get_room_name(&room_id)
                .await
                .unwrap_or_else(|_| room_id.clone());
            rooms.push(RoomSummary {
                room_id,
                display_name,
            });
        }
        rooms.sort_by(|left, right| left.display_name.cmp(&right.display_name));
        Ok(rooms)
    }

    pub async fn current_sync_token(&self) -> Result<String> {
        let response = self.sync(None, 0).await?;
        Ok(response.next_batch)
    }

    pub async fn send_text_message(&self, room_id: &str, body: &str) -> Result<String> {
        let txn_id = Uuid::new_v4().to_string();
        let encoded_room = encode(room_id);
        let url = format!(
            "{}/_matrix/client/v3/rooms/{}/send/m.room.message/{}",
            self.homeserver, encoded_room, txn_id
        );
        let response = self
            .http
            .put(url)
            .headers(self.auth_headers()?)
            .json(&json!({
                "msgtype": "m.text",
                "body": body,
            }))
            .send()
            .await
            .context("Failed to send Matrix message")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Failed to send Matrix message: {status} {text}"));
        }

        let sent: SendResponse = response.json().await.context("Invalid send response")?;
        Ok(sent.event_id)
    }

    pub async fn wait_for_room_reply(
        &self,
        room_id: &str,
        since_token: String,
        not_before: DateTime<Utc>,
        reply_timeout: Duration,
        sync_timeout: Duration,
    ) -> Result<Option<ReplyCapture>> {
        let deadline = Instant::now() + reply_timeout;
        let mut since = since_token;
        let mut aggregated_reply: Option<ReplyCapture> = None;
        let mut idle_deadline: Option<Instant> = None;

        while Instant::now() < deadline {
            if let (Some(reply), Some(idle_until)) = (&aggregated_reply, idle_deadline) {
                if Instant::now() >= idle_until {
                    return Ok(Some(reply.clone()));
                }
            }

            let remaining_total = deadline.saturating_duration_since(Instant::now());
            let wait_budget = match idle_deadline {
                Some(idle_until) => min(remaining_total, idle_until.saturating_duration_since(Instant::now())),
                None => remaining_total,
            };
            let request_timeout = min(sync_timeout, wait_budget);

            let response = self
                .sync(Some(&since), request_timeout.as_millis() as u64)
                .await?;
            since = response.next_batch.clone();

            if let Some(room) = response.rooms.join.get(room_id) {
                for event in &room.timeline.events {
                    if event.event_type != "m.room.message" {
                        continue;
                    }
                    if event.sender.as_deref() == Some(self.user_id()) {
                        continue;
                    }
                    if event.content.msgtype.as_deref() != Some("m.text") {
                        continue;
                    }
                    let body = match event.content.body.clone() {
                        Some(body) if !body.trim().is_empty() => body,
                        _ => continue,
                    };
                    if aggregated_reply.is_none() && is_interim_reply(&body) {
                        continue;
                    }
                    let received_at = event
                        .origin_server_ts
                        .and_then(DateTime::<Utc>::from_timestamp_millis)
                        .unwrap_or_else(Utc::now);
                    if received_at < not_before {
                        continue;
                    }
                    let latency_ms = (received_at - not_before).num_milliseconds();
                    let sender = event.sender.clone().unwrap_or_default();

                    if let Some(existing_reply) = aggregated_reply.as_mut() {
                        append_reply_body(&mut existing_reply.body, &body);
                        existing_reply.sender = sender;
                        existing_reply.event_id = event.event_id.clone();
                        existing_reply.received_at = received_at;
                        existing_reply.latency_ms = latency_ms;
                    } else {
                        aggregated_reply = Some(ReplyCapture {
                            sender,
                            body,
                            event_id: event.event_id.clone(),
                            received_at,
                            latency_ms,
                        });
                    }

                    idle_deadline = Some(Instant::now() + FINAL_REPLY_IDLE_TIMEOUT);
                }
            }

            sleep(Duration::from_millis(250)).await;
        }

        Ok(aggregated_reply)
    }

    async fn get_room_name(&self, room_id: &str) -> Result<String> {
        let encoded_room = encode(room_id);
        let url = format!(
            "{}/_matrix/client/v3/rooms/{}/state/m.room.name",
            self.homeserver, encoded_room
        );
        let response = self
            .http
            .get(url)
            .headers(self.auth_headers()?)
            .send()
            .await
            .context("Failed to query room name")?;

        if !response.status().is_success() {
            return Err(anyhow!("No room name available"));
        }

        let content: RoomNameContent = response
            .json()
            .await
            .context("Invalid room name response")?;
        Ok(content.name)
    }

    async fn sync(&self, since: Option<&str>, timeout_ms: u64) -> Result<SyncResponse> {
        let mut url = format!(
            "{}/_matrix/client/v3/sync?timeout={timeout_ms}",
            self.homeserver
        );
        if let Some(since) = since {
            url.push_str("&since=");
            url.push_str(&encode(since));
        }

        let response = self
            .http
            .get(url)
            .headers(self.auth_headers()?)
            .send()
            .await
            .context("Matrix sync request failed")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Matrix sync failed: {status} {text}"));
        }

        response.json().await.context("Invalid sync response")
    }

    fn auth_headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.access_token))?,
        );
        Ok(headers)
    }
}

fn is_interim_reply(body: &str) -> bool {
    body.trim().chars().count() < INTERIM_REPLY_MAX_CHARS
}

fn append_reply_body(existing_body: &mut String, next_body: &str) {
    let trimmed = next_body.trim();
    if trimmed.is_empty() || existing_body.contains(trimmed) {
        return;
    }

    if !existing_body.ends_with("\n\n") {
        existing_body.push_str("\n\n");
    }
    existing_body.push_str(trimmed);
}

#[cfg(test)]
mod tests {
    use super::{append_reply_body, is_interim_reply};

    #[test]
    fn short_reply_is_treated_as_interim() {
        let body = "我来帮您排查视频花屏问题。先让我在知识库中查找相关资料。";
        assert!(is_interim_reply(body));
    }

    #[test]
    fn long_reply_is_treated_as_final_answer() {
        let body = "问题定位：视频长时间花屏通常与 socket recv buffer 溢出有关。排查步骤：第一步先增大 rmem_max；第二步执行 netstat -na 观察 Recv-Q 和 Send-Q；第三步确认 RTSP 码流是否持续堆积；第四步根据结果调整 buffer queue，并复测网络和应用处理抖动。";
        assert!(!is_interim_reply(body));
    }

    #[test]
    fn appends_follow_up_reply_once() {
        let mut body = "问题定位：这是第一段正式回复。".to_string();
        append_reply_body(&mut body, "补充说明：这是第二段回复。");
        append_reply_body(&mut body, "补充说明：这是第二段回复。");
        assert_eq!(body, "问题定位：这是第一段正式回复。\n\n补充说明：这是第二段回复。");
    }
}
