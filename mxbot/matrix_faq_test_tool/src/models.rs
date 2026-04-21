use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaqItem {
    pub index: usize,
    pub section: String,
    pub subsection: String,
    pub question: String,
    pub answer_markdown: String,
    pub answer_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomSummary {
    pub room_id: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyCapture {
    pub sender: String,
    pub body: String,
    pub event_id: Option<String>,
    pub received_at: DateTime<Utc>,
    pub latency_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JudgeVerdict {
    Pass,
    Partial,
    Fail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgeResult {
    pub verdict: JudgeVerdict,
    pub score: f64,
    pub reason: String,
    pub detailed_reason: String,
    pub matched_points: Vec<String>,
    pub missing_points: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaseStatus {
    Passed,
    Partial,
    Failed,
    Timeout,
    SendError,
    JudgeError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseResult {
    pub source_file: String,
    pub room_id: String,
    pub question_index: usize,
    pub section: String,
    pub subsection: String,
    pub question: String,
    pub expected_answer: String,
    pub actual_reply: Option<String>,
    pub reply_sender: Option<String>,
    pub status: CaseStatus,
    pub judge: Option<JudgeResult>,
    pub reason: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    pub latency_ms: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunSummary {
    pub source_file: String,
    pub room_id: String,
    pub total: usize,
    pub passed: usize,
    pub partial: usize,
    pub failed: usize,
    pub timeout: usize,
    pub send_error: usize,
    pub judge_error: usize,
    pub accuracy: f64,
    pub generated_at: DateTime<Utc>,
    pub cases: Vec<CaseResult>,
}
