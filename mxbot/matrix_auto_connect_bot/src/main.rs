use anyhow::{Context, Result};
use matrix_sdk::ruma::{OwnedRoomId, OwnedUserId};
use matrix_sdk::Client;
use reqwest::{header, StatusCode};
use serde::Deserialize;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::env;
use std::time::Duration;
use tracing::{error, info, warn};

#[derive(Debug, Deserialize)]
struct UserInfo {
    #[serde(rename = "name")]
    user_id: String,
    #[serde(default)]
    #[allow(dead_code)]
    user_type: Option<String>,
    #[serde(default)]
    is_guest: bool,
    #[allow(dead_code)]
    admin: bool,
    deactivated: bool,
}

#[derive(Debug, Deserialize)]
struct UsersResponse {
    users: Vec<UserInfo>,
    #[serde(default)]
    next_token: Option<String>,
    #[allow(dead_code)]
    #[serde(default)]
    total: u32,
}

#[derive(Debug, Deserialize)]
struct JoinedRoomsResponse {
    joined_rooms: Vec<String>,
    #[allow(dead_code)]
    #[serde(default)]
    total: u64,
}

#[derive(Debug, Deserialize)]
struct RoomMembersResponse {
    members: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RoomStateResponse {
    state: Vec<Value>,
}

#[derive(Debug, Deserialize)]
struct AdminLoginResponse {
    access_token: String,
    #[allow(dead_code)]
    #[serde(default)]
    device_id: Option<String>,
    #[allow(dead_code)]
    #[serde(default)]
    user_id: Option<String>,
}

struct AppState {
    client: Client,
    http: reqwest::Client,
    homeserver_url: String,
    admin_token: String,
    #[allow(dead_code)]
    homeserver_domain: String,
    support_bots: Vec<OwnedUserId>,
    admin_user_id: OwnedUserId,
    user_whitelist: HashSet<OwnedUserId>,
    bot_display_names: HashMap<OwnedUserId, String>,
}

fn admin_headers(state: &AppState) -> Result<header::HeaderMap> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", state.admin_token))?,
    );
    Ok(headers)
}

fn parse_user_ids_csv(raw: &str, homeserver_domain: &str) -> Result<Vec<OwnedUserId>> {
    raw.split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|token| normalize_user_id_token(token, homeserver_domain))
        .collect::<Result<Vec<_>, _>>()
        .map_err(Into::into)
}

fn normalize_user_id_token(token: &str, homeserver_domain: &str) -> Result<OwnedUserId> {
    // 支持三种格式：
    // 1) 简写：alice -> @alice:<homeserver_domain>
    // 2) Matrix 标准：@alice:example.com
    // 3) 兼容格式：!alice@example.com -> @alice:example.com
    //
    // 注意：`homeserver_domain` 来自 HOMESERVER URL 的 host 部分（不含 scheme/port 以外的内容）。
    if token.starts_with('@') {
        return Ok(token.parse()?);
    }

    if let Some(rest) = token.strip_prefix('!') {
        // !<username>@<homeserver>
        if let Some((u, hs)) = rest.split_once('@') {
            let u = u.trim().trim_start_matches('@');
            let hs = hs.trim();
            anyhow::ensure!(!u.is_empty() && !hs.is_empty(), "无效用户格式: {}", token);
            return Ok(format!("@{}:{}", u, hs).parse()?);
        }
        anyhow::bail!("无效用户格式(缺少 @ 分隔): {}", token);
    }

    Ok(format!("@{}:{}", token, homeserver_domain).parse()?)
}

async fn get_user_list(state: &AppState) -> Result<HashSet<OwnedUserId>> {
    let http = &state.http;
    let mut all = HashSet::new();
    let mut from_token: Option<String> = None;
    loop {
        let mut url = format!(
            "{}/_synapse/admin/v2/users?guests=false&deactivated=false&limit=100",
            state.homeserver_url.trim_end_matches('/')
        );
        if let Some(ref t) = from_token {
            url.push_str("&from=");
            url.push_str(&url::form_urlencoded::byte_serialize(t.as_bytes()).collect::<String>());
        }

        let response = http
            .get(&url)
            .headers(admin_headers(state)?)
            .send()
            .await?;

        if response.status() != StatusCode::OK {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("获取用户列表失败. Status: {}, Error: {}", status, error_text);
        }

        let page: UsersResponse = response.json().await?;
        for u in page.users {
            if !u.deactivated && !u.is_guest {
                if let Ok(id) = u.user_id.parse::<OwnedUserId>() {
                    all.insert(id);
                }
            }
        }

        from_token = page.next_token.filter(|s| !s.is_empty());
        if from_token.is_none() {
            break;
        }
    }

    Ok(all)
}

async fn get_joined_rooms_for_user(state: &AppState, user_id: &OwnedUserId) -> Result<HashSet<String>> {
    let uid_enc = url::form_urlencoded::byte_serialize(user_id.as_str().as_bytes()).collect::<String>();
    let url = format!(
        "{}/_synapse/admin/v1/users/{}/joined_rooms",
        state.homeserver_url.trim_end_matches('/'),
        uid_enc
    );

    let response = state
        .http
        .get(&url)
        .headers(admin_headers(state)?)
        .send()
        .await?;

    if response.status() != StatusCode::OK {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        anyhow::bail!(
            "获取用户 {} 的 joined_rooms 失败. Status: {}, Error: {}",
            user_id,
            status,
            error_text
        );
    }

    let body: JoinedRoomsResponse = response.json().await?;
    Ok(body.joined_rooms.into_iter().collect())
}

async fn get_room_members(state: &AppState, room_id: &str) -> Result<Vec<String>> {
    let rid_enc = url::form_urlencoded::byte_serialize(room_id.as_bytes()).collect::<String>();
    let url = format!(
        "{}/_synapse/admin/v1/rooms/{}/members",
        state.homeserver_url.trim_end_matches('/'),
        rid_enc
    );

    let response = state
        .http
        .get(&url)
        .headers(admin_headers(state)?)
        .send()
        .await?;

    if response.status() != StatusCode::OK {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        anyhow::bail!(
            "获取房间 {} 成员失败. Status: {}, Error: {}",
            room_id,
            status,
            error_text
        );
    }

    let body: RoomMembersResponse = response.json().await?;
    Ok(body.members)
}

/// 若 m.room.create 中显式 `is_direct: false` 则返回 true（应排除该房作为 DM）。
async fn room_create_is_explicitly_non_direct(state: &AppState, room_id: &str) -> Result<bool> {
    let rid_enc = url::form_urlencoded::byte_serialize(room_id.as_bytes()).collect::<String>();
    let url = format!(
        "{}/_synapse/admin/v1/rooms/{}/state",
        state.homeserver_url.trim_end_matches('/'),
        rid_enc
    );

    let response = state
        .http
        .get(&url)
        .headers(admin_headers(state)?)
        .send()
        .await?;

    if response.status() != StatusCode::OK {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        anyhow::bail!(
            "获取房间 {} 状态失败. Status: {}, Error: {}",
            room_id,
            status,
            error_text
        );
    }

    let body: RoomStateResponse = response.json().await?;
    for ev in body.state {
        if ev.get("type").and_then(|t| t.as_str()) == Some("m.room.create")
            && ev.get("state_key").and_then(|k| k.as_str()) == Some("")
        {
            if let Some(content) = ev.get("content") {
                if let Some(false) = content.get("is_direct").and_then(|v| v.as_bool()) {
                    return Ok(true);
                }
            }
            break;
        }
    }
    Ok(false)
}

#[derive(Debug, Deserialize)]
struct ProfileDisplayNameResponse {
    #[serde(default)]
    displayname: Option<String>,
}

async fn get_display_name_for_user(
    http: &reqwest::Client,
    homeserver_url: &str,
    admin_token: &str,
    user_id: &OwnedUserId,
) -> Result<Option<String>> {
    let uid_enc =
        url::form_urlencoded::byte_serialize(user_id.as_str().as_bytes()).collect::<String>();
    let url = format!(
        "{}/_matrix/client/v3/profile/{}/displayname",
        homeserver_url.trim_end_matches('/'),
        uid_enc
    );

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", admin_token))?,
    );

    let response = http.get(&url).headers(headers).send().await?;
    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        warn!(
            "获取 {} 显示名称失败，使用 localpart. Status: {}, Error: {}",
            user_id, status, text
        );
        return Ok(None);
    }

    let body: ProfileDisplayNameResponse = response.json().await?;
    Ok(body.displayname.filter(|s| !s.is_empty()))
}

async fn is_twin_dm_room(
    state: &AppState,
    room_id: &str,
    user_id: &OwnedUserId,
    bot_id: &OwnedUserId,
) -> Result<bool> {
    let members = get_room_members(state, room_id).await?;
    if members.len() != 2 {
        return Ok(false);
    }
    let u = user_id.to_string();
    let b = bot_id.to_string();
    let mut ok_user = false;
    let mut ok_bot = false;
    for m in &members {
        if m == &u {
            ok_user = true;
        }
        if m == &b {
            ok_bot = true;
        }
    }
    if !ok_user || !ok_bot {
        return Ok(false);
    }
    if room_create_is_explicitly_non_direct(state, room_id).await? {
        return Ok(false);
    }
    Ok(true)
}

async fn force_join_room(state: &AppState, room_id: &str, user_id: &OwnedUserId) -> Result<()> {
    let rid_enc = url::form_urlencoded::byte_serialize(room_id.as_bytes()).collect::<String>();
    let url = format!(
        "{}/_synapse/admin/v1/join/{}",
        state.homeserver_url.trim_end_matches('/'),
        rid_enc
    );

    let mut headers = admin_headers(state)?;
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    let body = serde_json::json!({ "user_id": user_id.to_string() });

    let response = state.http.post(&url).headers(headers).json(&body).send().await?;

    if response.status().is_success() {
        info!("已强制用户 {} 加入房间 {}", user_id, room_id);
        Ok(())
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        anyhow::bail!("强制加入失败. Status: {}, Error: {}", status, error_text)
    }
}

async fn admin_login_as_user(state: &AppState, user_id: &OwnedUserId) -> Result<String> {
    let uid_enc =
        url::form_urlencoded::byte_serialize(user_id.as_str().as_bytes()).collect::<String>();
    let url = format!(
        "{}/_synapse/admin/v1/users/{}/login",
        state.homeserver_url.trim_end_matches('/'),
        uid_enc
    );

    let response = state
        .http
        .post(&url)
        .headers(admin_headers(state)?)
        .send()
        .await?;

    if response.status() != StatusCode::OK {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        anyhow::bail!(
            "管理员代登录 {} 失败. Status: {}, Error: {}",
            user_id,
            status,
            error_text
        );
    }

    let body: AdminLoginResponse = response.json().await?;
    Ok(body.access_token)
}

async fn leave_room_as_user(
    state: &AppState,
    room_id: &str,
    user_id: &OwnedUserId,
) -> Result<()> {
    let token = admin_login_as_user(state, user_id).await?;
    let rid_enc = url::form_urlencoded::byte_serialize(room_id.as_bytes()).collect::<String>();
    let url = format!(
        "{}/_matrix/client/v3/rooms/{}/leave",
        state.homeserver_url.trim_end_matches('/'),
        rid_enc
    );

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", token))?,
    );

    let response = state.http.post(&url).headers(headers).send().await?;
    if response.status().is_success() {
        info!("已让用户 {} 离开房间 {}", user_id, room_id);
        Ok(())
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        anyhow::bail!(
            "让用户 {} 离开房间 {} 失败. Status: {}, Error: {}",
            user_id,
            room_id,
            status,
            error_text
        )
    }
}

async fn dedupe_twin_rooms(
    state: &AppState,
    user_id: &OwnedUserId,
    bot_id: &OwnedUserId,
    mut twin_room_ids: Vec<String>,
    bot_room_cache: &mut HashSet<String>,
) -> Result<()> {
    twin_room_ids.sort();
    let keep = twin_room_ids[0].clone();
    let remove: Vec<String> = twin_room_ids[1..].to_vec();
    warn!(
        "用户 {} 与 bot {} 存在 {} 间双人私聊候选房，保留 {}，正在从其余房间移除 bot",
        user_id,
        bot_id,
        twin_room_ids.len(),
        keep
    );
    for rid in remove {
        match leave_room_as_user(state, &rid, bot_id).await {
            Ok(()) => {
                bot_room_cache.remove(&rid);
            }
            Err(e) => error!(
                "去重失败 user={} bot={} room={}: {}",
                user_id, bot_id, rid, e
            ),
        }
    }
    Ok(())
}

async fn create_private_room_with_bot(
    client: &Client,
    new_user_id: &OwnedUserId,
    bot_user_id: &OwnedUserId,
    state: &AppState,
) -> Result<OwnedRoomId> {
    info!("创建私聊房间：{} 和 {}", new_user_id, bot_user_id);

    use matrix_sdk::ruma::api::client::room::create_room::v3::{Request, RoomPreset};

    let mut request = Request::new();
    request.is_direct = true;
    request.preset = Some(RoomPreset::PrivateChat);
    let bot_display = state
        .bot_display_names
        .get(bot_user_id)
        .cloned()
        .unwrap_or_else(|| bot_user_id.localpart().to_string());
    request.name = Some(format!("{} 与 {}", bot_display, new_user_id.localpart()));

    let room = client
        .create_room(request)
        .await
        .context("创建私聊失败")?;

    let room_id: OwnedRoomId = room.room_id().to_owned();
    let room_id_str = room_id.as_str();

    info!("已创建私聊房间 {}，正在拉入用户...", room_id);

    force_join_room(state, room_id_str, new_user_id).await?;
    force_join_room(state, room_id_str, bot_user_id).await?;

    info!("所有用户已拉入，正在设置权限...");

    let admin_user_id = client
        .user_id()
        .ok_or_else(|| anyhow::anyhow!("未登录"))?
        .to_string();

    let power_levels = serde_json::json!({
        "users": {
            admin_user_id: 100,
            new_user_id.to_string(): 0,
            bot_user_id.to_string(): 0
        },
        "users_default": 0,
        "events": {
            "m.room.name": 50,
            "m.room.power_levels": 100,
            "m.room.history_visibility": 100,
            "m.room.canonical_alias": 50,
            "m.room.avatar": 50,
            "m.room.topic": 50,
            "m.room.encryption": 100
        },
        "events_default": 0,
        "state_default": 50,
        "ban": 50,
        "kick": 50,
        "redact": 50,
        "invite": 50,
        "notifications": {
            "room": 50
        }
    });

    let rid_enc = url::form_urlencoded::byte_serialize(room_id_str.as_bytes()).collect::<String>();
    let url = format!(
        "{}/_matrix/client/v3/rooms/{}/state/m.room.power_levels",
        state.homeserver_url.trim_end_matches('/'),
        rid_enc
    );

    let mut headers = admin_headers(state)?;
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    let response = state
        .http
        .put(&url)
        .headers(headers)
        .json(&power_levels)
        .send()
        .await?;

    if response.status().is_success() {
        info!("已设置房间 {} 的权限，禁止E2EE", room_id);
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        error!(
            "设置房间权限失败 {}: Status: {}, Error: {}",
            room_id, status, error_text
        );
    }

    tokio::time::sleep(Duration::from_millis(500)).await;

    if let Err(e) = room.leave().await {
        error!("管理员离开房间失败 {}: {}", room_id, e);
    } else {
        info!("管理员已离开房间 {}", room_id);
    }

    info!(
        "私聊房间 {} 创建完成，参与者：{} 和 {}",
        room_id, new_user_id, bot_user_id
    );

    Ok(room_id)
}

async fn reconcile_private_rooms(state: &AppState) -> Result<()> {
    let all_users = get_user_list(state).await.context("获取用户列表失败")?;

    let mut skip: HashSet<OwnedUserId> = state.user_whitelist.clone();
    skip.insert(state.admin_user_id.clone());
    for b in &state.support_bots {
        skip.insert(b.clone());
    }

    let mut target_users: Vec<OwnedUserId> = all_users.difference(&skip).cloned().collect();
    target_users.sort_by(|a, b| a.as_str().cmp(b.as_str()));

    info!(
        "私聊巡检：总用户 {}，目标用户 {}（已排除白名单 / 管理员 / 支持 bot）",
        all_users.len(),
        target_users.len()
    );

    let mut bot_room_cache: HashMap<OwnedUserId, HashSet<String>> = HashMap::new();
    for bot in &state.support_bots {
        let rooms = get_joined_rooms_for_user(state, bot).await?;
        bot_room_cache.insert(bot.clone(), rooms);
    }

    for user_id in &target_users {
        let mut user_rooms = match get_joined_rooms_for_user(state, user_id).await {
            Ok(r) => r,
            Err(e) => {
                error!("获取用户 {} 的 joined_rooms 失败: {}", user_id, e);
                continue;
            }
        };

        for bot_id in &state.support_bots {
            let bot_rooms = match bot_room_cache.get(bot_id) {
                Some(r) => r.clone(),
                None => HashSet::new(),
            };

            let candidates: Vec<String> = user_rooms.intersection(&bot_rooms).cloned().collect();

            let mut twin_rooms: Vec<String> = Vec::new();
            for room_id in candidates {
                match is_twin_dm_room(state, &room_id, user_id, bot_id).await {
                    Ok(true) => twin_rooms.push(room_id),
                    Ok(false) => {}
                    Err(e) => warn!("检查房间 {} 是否双人 DM 失败: {}", room_id, e),
                }
            }

            twin_rooms.sort();

            match twin_rooms.len() {
                0 => {
                    match create_private_room_with_bot(&state.client, user_id, bot_id, state).await {
                        Ok(rid) => {
                            let rid_str = rid.as_str().to_string();
                            user_rooms.insert(rid_str.clone());
                            if let Some(cache) = bot_room_cache.get_mut(bot_id) {
                                cache.insert(rid_str);
                            }
                            info!("已为用户 {} 创建与 {} 的私聊房间", user_id, bot_id);
                        }
                        Err(e) => error!(
                            "创建与 {} 的私聊房间失败，用户 {}: {}",
                            bot_id, user_id, e
                        ),
                    }
                }
                1 => {}
                _ => {
                    if let Some(cache) = bot_room_cache.get_mut(bot_id) {
                        if let Err(e) =
                            dedupe_twin_rooms(state, user_id, bot_id, twin_rooms, cache).await
                        {
                            error!("去重 user={} bot={}: {}", user_id, bot_id, e);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

async fn run(state: AppState, poll_interval: Duration) -> Result<()> {
    info!("Matrix 自动连接机器人启动中（全量私聊巡检模式）...");

    loop {
        match reconcile_private_rooms(&state).await {
            Ok(()) => {}
            Err(e) => {
                error!("私聊巡检出错: {}", e);
            }
        }

        tokio::time::sleep(poll_interval).await;
    }
}

fn extract_homeserver(homeserver_url: &str) -> Result<String> {
    let url = url::Url::parse(homeserver_url)?;
    let host = url
        .host_str()
        .context("Failed to extract host from homeserver URL")?;
    Ok(host.to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        tracing_subscriber::EnvFilter::new(
            "warn,matrix_sdk_base=error,matrix_auto_connect_bot=info",
        )
    });

    tracing_subscriber::fmt().with_env_filter(filter).init();

    let homeserver =
        env::var("HOMESERVER").unwrap_or_else(|_| "http://localhost:8008".to_string());
    let homeserver_domain = extract_homeserver(&homeserver)?;

    let admin_user_env = env::var("ADMIN_USER").unwrap_or_else(|_| "admin".to_string());
    let admin_user_str = if admin_user_env.starts_with('@') {
        admin_user_env
    } else {
        format!("@{}:{}", admin_user_env, homeserver_domain)
    };
    let admin_user_id: OwnedUserId = admin_user_str.parse().context("Invalid admin user ID")?;
    let admin_password =
        env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD environment variable must be set");

    let support_bots_env = env::var("SUPPORT_BOTS").unwrap_or_else(|_| {
        "se7-support-bot,se9-support-bot".to_string()
    });
    let support_bots: Vec<OwnedUserId> = parse_user_ids_csv(&support_bots_env, &homeserver_domain)
        .context("Failed to parse support bot user IDs")?;

    if support_bots.is_empty() {
        anyhow::bail!("SUPPORT_BOTS must be configured with at least one bot user ID");
    }

    let user_whitelist: HashSet<OwnedUserId> = env::var("USER_WHITELIST")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(|s| parse_user_ids_csv(&s, &homeserver_domain))
        .transpose()?
        .unwrap_or_default()
        .into_iter()
        .collect();

    info!("配置的支持机器人: {:?}", support_bots);
    if !user_whitelist.is_empty() {
        info!("用户白名单（不巡检）: {:?}", user_whitelist);
    }

    let http = reqwest::Client::builder()
        .timeout(Duration::from_secs(120))
        .build()
        .context("构建 HTTP 客户端")?;

    let client = Client::builder()
        .homeserver_url(&homeserver)
        .build()
        .await?;

    info!("正在以管理员身份登录: {}", admin_user_id);
    let login_response = client
        .matrix_auth()
        .login_username(&admin_user_str, &admin_password)
        .initial_device_display_name("matrix-auto-connect-bot")
        .send()
        .await?;

    let admin_token = login_response.access_token;

    info!("管理员登录成功");
    info!("已连接到服务器: {}", homeserver);

    // 预先获取每个 bot 的显示名称，用于房间命名
    let mut bot_display_names: HashMap<OwnedUserId, String> = HashMap::new();
    for bot in &support_bots {
        let display = get_display_name_for_user(&http, &homeserver, &admin_token, bot).await?;
        let final_name = display.unwrap_or_else(|| bot.localpart().to_string());
        bot_display_names.insert(bot.clone(), final_name);
    }

    let state = AppState {
        client,
        http,
        homeserver_url: homeserver,
        admin_token,
        homeserver_domain,
        support_bots,
        admin_user_id,
        user_whitelist,
        bot_display_names,
    };

    let poll_secs: u64 = env::var("POLL_INTERVAL_SECONDS")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(5);
    info!("巡检间隔: {} 秒", poll_secs);

    run(state, Duration::from_secs(poll_secs)).await?;

    Ok(())
}
