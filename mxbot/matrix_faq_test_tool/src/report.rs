use crate::models::{CaseResult, CaseStatus, RunSummary};
use anyhow::{Context, Result};
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};

pub fn write_reports(output_dir: &Path, summary: &RunSummary) -> Result<(PathBuf, PathBuf)> {
    fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create output directory {}", output_dir.display()))?;

    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let json_path = output_dir.join(format!("matrix_faq_report_{timestamp}.json"));
    let csv_path = output_dir.join(format!("matrix_faq_report_{timestamp}.csv"));

    let json = serde_json::to_string_pretty(summary).context("Failed to serialize JSON report")?;
    fs::write(&json_path, json)
        .with_context(|| format!("Failed to write JSON report {}", json_path.display()))?;

    let mut writer = csv::Writer::from_path(&csv_path)
        .with_context(|| format!("Failed to open CSV report {}", csv_path.display()))?;
    writer.write_record([
        "source_file",
        "room_id",
        "question_index",
        "section",
        "subsection",
        "question",
        "expected_answer",
        "actual_reply",
        "reply_sender",
        "status",
        "score",
        "reason",
        "detailed_reason",
        "matched_points",
        "missing_points",
        "latency_ms",
        "started_at",
        "completed_at",
    ])?;

    for case in &summary.cases {
        writer.write_record(case_to_csv_row(case))?;
    }
    writer.flush().context("Failed to flush CSV report")?;

    Ok((json_path, csv_path))
}

pub fn print_summary(summary: &RunSummary) {
    println!("FAQ file: {}", summary.source_file);
    println!("Room: {}", summary.room_id);
    println!("Total: {}", summary.total);
    println!("Passed: {}", summary.passed);
    println!("Partial: {}", summary.partial);
    println!("Failed: {}", summary.failed);
    println!("Timeout: {}", summary.timeout);
    println!("Send error: {}", summary.send_error);
    println!("Judge error: {}", summary.judge_error);
    println!("Accuracy: {:.2}%", summary.accuracy * 100.0);
    println!();

    for case in summary
        .cases
        .iter()
        .filter(|case| {
            matches!(
                case.status,
                CaseStatus::Failed
                    | CaseStatus::Partial
                    | CaseStatus::Timeout
                    | CaseStatus::SendError
                    | CaseStatus::JudgeError
            )
        })
        .take(5)
    {
        println!("[{}] {}", case.question_index, case.question);
        println!("Status: {:?}", case.status);
        println!("Reason: {}", case.reason);
        if let Some(reply) = &case.actual_reply {
            println!("Reply: {}", reply);
        }
        println!();
    }
}

fn case_to_csv_row(case: &CaseResult) -> [String; 18] {
    [
        case.source_file.clone(),
        case.room_id.clone(),
        case.question_index.to_string(),
        case.section.clone(),
        case.subsection.clone(),
        case.question.clone(),
        case.expected_answer.clone(),
        case.actual_reply.clone().unwrap_or_default(),
        case.reply_sender.clone().unwrap_or_default(),
        format!("{:?}", case.status),
        case.judge
            .as_ref()
            .map(|judge| judge.score.to_string())
            .unwrap_or_default(),
        case.reason.clone(),
        case.judge
            .as_ref()
            .map(|judge| judge.detailed_reason.clone())
            .unwrap_or_default(),
        case.judge
            .as_ref()
            .map(|judge| judge.matched_points.join("\n"))
            .unwrap_or_default(),
        case.judge
            .as_ref()
            .map(|judge| judge.missing_points.join("\n"))
            .unwrap_or_default(),
        case.latency_ms
            .map(|value| value.to_string())
            .unwrap_or_default(),
        case.started_at.to_rfc3339(),
        case.completed_at.to_rfc3339(),
    ]
}
