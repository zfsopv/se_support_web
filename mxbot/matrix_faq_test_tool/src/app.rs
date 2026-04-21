use crate::config::{CliArgs, EnvConfig};
use crate::faq_parser::parse_faq_file;
use crate::judge::JudgeClient;
use crate::matrix::MatrixClient;
use crate::models::{CaseResult, CaseStatus, JudgeVerdict, RoomSummary, RunSummary};
use crate::report::{print_summary, write_reports};
use anyhow::{Context, Result, anyhow};
use chrono::Utc;
use dialoguer::{Select, theme::ColorfulTheme};
use std::path::PathBuf;
use std::time::Duration;
use tokio::time::sleep;

const RESET_COMMAND: &str = "/reset";
const RESET_SETTLE_MS: u64 = 1000;

pub async fn run(cli_args: CliArgs, env_config: EnvConfig) -> Result<()> {
    let faq_items = parse_faq_file(&cli_args.faq_file)?;
    if faq_items.is_empty() {
        return Err(anyhow!(
            "No FAQ questions were extracted from {}",
            cli_args.faq_file.display()
        ));
    }

    let output_dir = cli_args
        .output_dir
        .clone()
        .unwrap_or_else(|| env_config.output_dir.clone());

    let total_questions = faq_items.len();
    let run_limit = cli_args.limit.unwrap_or(total_questions).min(total_questions);

    println!(
        "Loaded {} FAQ questions from {}",
        total_questions,
        cli_args.faq_file.display()
    );
    if run_limit != total_questions {
        println!("Running a limited subset: {run_limit}/{total_questions} questions");
    }
    println!("Logging into Matrix homeserver {}", env_config.homeserver);

    let matrix_client = MatrixClient::login(
        &env_config.homeserver,
        &env_config.test_user,
        &env_config.test_password,
    )
    .await?;
    println!("Logged in as {}", matrix_client.user_id());

    let rooms = matrix_client.list_joined_rooms().await?;
    if rooms.is_empty() {
        return Err(anyhow!("The test account is not joined to any rooms"));
    }

    let selected_room = select_room(&rooms)?;
    println!(
        "Selected room: {} ({})",
        selected_room.display_name, selected_room.room_id
    );

    let judge_client = JudgeClient::new(
        env_config.openai_base_url.clone(),
        env_config.openai_api_key.clone(),
        env_config.openai_model.clone(),
    );

    let mut cases = Vec::with_capacity(run_limit);
    for (run_index, item) in faq_items.into_iter().take(run_limit).enumerate() {
        let started_at = Utc::now();
        println!("[{}/{}] Asking: {}", run_index + 1, run_limit, item.question);

        let reset_result = matrix_client
            .send_text_message(&selected_room.room_id, RESET_COMMAND)
            .await;

        let case = match reset_result {
            Ok(_) => {
                sleep(Duration::from_millis(RESET_SETTLE_MS)).await;

                let sync_token = matrix_client.current_sync_token().await?;
                let question_sent_at = Utc::now();
                let send_result = matrix_client
                    .send_text_message(&selected_room.room_id, &item.question)
                    .await;

                match send_result {
                    Ok(_) => {
                let reply = matrix_client
                    .wait_for_room_reply(
                        &selected_room.room_id,
                        sync_token,
                        question_sent_at,
                        Duration::from_secs(env_config.reply_timeout_seconds),
                        Duration::from_millis(env_config.sync_timeout_ms),
                    )
                    .await?;

                match reply {
                    Some(reply) => match judge_client
                        .judge_answer(&item.question, &item.answer_markdown, &reply.body)
                        .await
                    {
                        Ok(judge) => {
                            let (status, reason) = match judge.verdict {
                                JudgeVerdict::Pass => (CaseStatus::Passed, judge.reason.clone()),
                                JudgeVerdict::Partial => {
                                    (CaseStatus::Partial, judge.reason.clone())
                                }
                                JudgeVerdict::Fail => (CaseStatus::Failed, judge.reason.clone()),
                            };
                            CaseResult {
                                source_file: cli_args.faq_file.display().to_string(),
                                room_id: selected_room.room_id.clone(),
                                question_index: item.index,
                                section: item.section.clone(),
                                subsection: item.subsection.clone(),
                                question: item.question.clone(),
                                expected_answer: item.answer_markdown.clone(),
                                actual_reply: Some(reply.body.clone()),
                                reply_sender: Some(reply.sender.clone()),
                                status,
                                judge: Some(judge),
                                reason,
                                started_at,
                                completed_at: Utc::now(),
                                latency_ms: Some(reply.latency_ms),
                            }
                        }
                        Err(error) => CaseResult {
                            source_file: cli_args.faq_file.display().to_string(),
                            room_id: selected_room.room_id.clone(),
                            question_index: item.index,
                            section: item.section.clone(),
                            subsection: item.subsection.clone(),
                            question: item.question.clone(),
                            expected_answer: item.answer_markdown.clone(),
                            actual_reply: Some(reply.body.clone()),
                            reply_sender: Some(reply.sender.clone()),
                            status: CaseStatus::JudgeError,
                            judge: None,
                            reason: error.to_string(),
                            started_at,
                            completed_at: Utc::now(),
                            latency_ms: Some(reply.latency_ms),
                        },
                    },
                    None => CaseResult {
                        source_file: cli_args.faq_file.display().to_string(),
                        room_id: selected_room.room_id.clone(),
                        question_index: item.index,
                        section: item.section.clone(),
                        subsection: item.subsection.clone(),
                        question: item.question.clone(),
                        expected_answer: item.answer_markdown.clone(),
                        actual_reply: None,
                        reply_sender: None,
                        status: CaseStatus::Timeout,
                        judge: None,
                        reason: format!(
                            "No room reply received within {} seconds",
                            env_config.reply_timeout_seconds
                        ),
                        started_at,
                        completed_at: Utc::now(),
                        latency_ms: None,
                    },
                }
            }
                    Err(error) => CaseResult {
                        source_file: cli_args.faq_file.display().to_string(),
                        room_id: selected_room.room_id.clone(),
                        question_index: item.index,
                        section: item.section.clone(),
                        subsection: item.subsection.clone(),
                        question: item.question.clone(),
                        expected_answer: item.answer_markdown.clone(),
                        actual_reply: None,
                        reply_sender: None,
                        status: CaseStatus::SendError,
                        judge: None,
                        reason: error.to_string(),
                        started_at,
                        completed_at: Utc::now(),
                        latency_ms: None,
                    },
                }
            }
            Err(error) => CaseResult {
                source_file: cli_args.faq_file.display().to_string(),
                room_id: selected_room.room_id.clone(),
                question_index: item.index,
                section: item.section.clone(),
                subsection: item.subsection.clone(),
                question: item.question.clone(),
                expected_answer: item.answer_markdown.clone(),
                actual_reply: None,
                reply_sender: None,
                status: CaseStatus::SendError,
                judge: None,
                reason: format!("Failed to send reset command before asking the question: {error}"),
                started_at,
                completed_at: Utc::now(),
                latency_ms: None,
            },
        };

        println!("[{}] {:?}", case.question_index, case.status);
        cases.push(case);
    }

    let summary = summarize(
        cli_args.faq_file.clone(),
        selected_room.room_id.clone(),
        cases,
    );
    print_summary(&summary);
    let (json_path, csv_path) = write_reports(&output_dir, &summary)?;
    println!("JSON report: {}", json_path.display());
    println!("CSV report: {}", csv_path.display());

    Ok(())
}

fn select_room(rooms: &[RoomSummary]) -> Result<RoomSummary> {
    let display_items: Vec<String> = rooms
        .iter()
        .map(|room| format!("{} ({})", room.display_name, room.room_id))
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the Matrix room to test")
        .items(&display_items)
        .default(0)
        .interact()
        .context("Failed to select a room")?;

    rooms
        .get(selection)
        .cloned()
        .ok_or_else(|| anyhow!("Invalid room selection index"))
}

fn summarize(source_file: PathBuf, room_id: String, cases: Vec<CaseResult>) -> RunSummary {
    let total = cases.len();
    let passed = cases
        .iter()
        .filter(|case| matches!(case.status, CaseStatus::Passed))
        .count();
    let partial = cases
        .iter()
        .filter(|case| matches!(case.status, CaseStatus::Partial))
        .count();
    let failed = cases
        .iter()
        .filter(|case| matches!(case.status, CaseStatus::Failed))
        .count();
    let timeout = cases
        .iter()
        .filter(|case| matches!(case.status, CaseStatus::Timeout))
        .count();
    let send_error = cases
        .iter()
        .filter(|case| matches!(case.status, CaseStatus::SendError))
        .count();
    let judge_error = cases
        .iter()
        .filter(|case| matches!(case.status, CaseStatus::JudgeError))
        .count();
    let accuracy = if total == 0 {
        0.0
    } else {
        passed as f64 / total as f64
    };

    RunSummary {
        source_file: source_file.display().to_string(),
        room_id,
        total,
        passed,
        partial,
        failed,
        timeout,
        send_error,
        judge_error,
        accuracy,
        generated_at: Utc::now(),
        cases,
    }
}
