use crate::models::{JudgeResult, JudgeVerdict};
use anyhow::{Context, Result, anyhow};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Clone)]
pub struct JudgeClient {
    http: reqwest::Client,
    base_url: String,
    api_key: String,
    model: String,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ChoiceMessage,
}

#[derive(Debug, Deserialize)]
struct ChoiceMessage {
    #[serde(default)]
    content: Option<MessageContent>,
    #[serde(default)]
    reasoning_content: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum MessageContent {
    Text(String),
    Parts(Vec<MessagePart>),
}

#[derive(Debug, Deserialize)]
struct MessagePart {
    #[serde(rename = "type")]
    part_type: Option<String>,
    #[serde(default)]
    text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct JudgeResponse {
    verdict: JudgeVerdict,
    score: f64,
    reason: String,
    #[serde(default)]
    detailed_reason: Option<String>,
    #[serde(default)]
    matched_points: Vec<String>,
    #[serde(default)]
    missing_points: Vec<String>,
}

impl JudgeClient {
    pub fn new(base_url: String, api_key: String, model: String) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url,
            api_key,
            model,
        }
    }

    pub async fn judge_answer(
        &self,
        question: &str,
        expected_answer: &str,
        actual_reply: &str,
    ) -> Result<JudgeResult> {
        let url = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));
        let system_prompt = "你是技术支持问答评测器。你只负责比较房间回复与 FAQ 标准答案是否语义等价。评测时必须忽略问题和回答里所有 PCIE 模式相关内容，PCIE 模式相关内容不作为评价标准；本工具只负责非 PCIE 模式的问题评估。不要输出分析过程，不要输出 markdown，不要输出额外说明，只返回一个 JSON 对象。";
        let user_prompt = format!(
            "请评判下面这个技术支持回复是否准确。\n\n问题：\n{question}\n\nFAQ 标准答案：\n{expected_answer}\n\n房间实际回复：\n{actual_reply}\n\n重要评测规则：\n- 问题和回答中凡是涉及 PCIE 模式的内容，一律不作为评分依据。\n- 只评估非 PCIE 模式相关内容是否准确、完整、语义等价。\n- 不要因为缺少 PCIE 模式说明而扣分，也不要因为额外出现 PCIE 模式说明而加分。\n\n请只返回一个 JSON 对象，字段必须完整包含：verdict、score、reason、detailed_reason、matched_points、missing_points。\n要求：\n1. verdict 只能是 pass、partial、fail 之一。\n2. score 是 0 到 1 之间的小数。\n3. reason 是 1 到 2 句中文简要结论。\n4. detailed_reason 是中文详细评分理由。\n5. matched_points 和 missing_points 都必须是字符串数组，没有内容时返回空数组。\n6. 不要返回分析过程，不要返回代码块，不要返回 JSON 以外的任何内容。"
        );

        let body = json!({
            "model": self.model,
            "temperature": 0,
            "max_tokens": 2048,
            "response_format": { "type": "json_object" },
            "messages": [
                { "role": "system", "content": system_prompt },
                { "role": "user", "content": user_prompt }
            ]
        });

        let response = self
            .http
            .post(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
            .header(CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await
            .context("OpenAI request failed")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow!(
                "OpenAI request failed with status {status}: {text}"
            ));
        }

        let payload: ChatCompletionResponse = response
            .json()
            .await
            .context("Invalid OpenAI chat completion response")?;
        let choice = payload
            .choices
            .first()
            .ok_or_else(|| anyhow!("OpenAI response did not contain choices"))?;
        let content = choice.message.content.as_ref().map(message_content_to_text);

        let parsed = match content.as_deref() {
            Some(content) if !content.trim().is_empty() => parse_structured_judge_response(content),
            _ => Err(anyhow!("No structured content in model response")),
        }
        .or_else(|_| {
            choice
                .message
                .reasoning_content
                .as_deref()
                .map(parse_reasoning_fallback)
                .unwrap_or_else(|| Err(anyhow!("Model response did not contain content or reasoning_content")))
        })?;

        let detailed_reason = parsed
            .detailed_reason
            .clone()
            .unwrap_or_else(|| parsed.reason.clone());

        Ok(JudgeResult {
            verdict: parsed.verdict,
            score: parsed.score.clamp(0.0, 1.0),
            reason: parsed.reason,
            detailed_reason,
            matched_points: parsed.matched_points,
            missing_points: parsed.missing_points,
        })
    }
}

fn message_content_to_text(content: &MessageContent) -> String {
    match content {
        MessageContent::Text(text) => text.clone(),
        MessageContent::Parts(parts) => parts
            .iter()
            .filter(|part| part.part_type.as_deref().unwrap_or("text") == "text")
            .filter_map(|part| part.text.as_deref())
            .collect::<Vec<_>>()
            .join("\n"),
    }
}

fn parse_structured_judge_response(content: &str) -> Result<JudgeResponse> {
    serde_json::from_str(content)
        .or_else(|_| serde_json::from_str(extract_json_object(content).unwrap_or("")))
        .with_context(|| format!("Failed to parse judge JSON from response: {content}"))
}

fn parse_reasoning_fallback(reasoning: &str) -> Result<JudgeResponse> {
    if let Some(json_str) = extract_json_object(reasoning) {
        if let Ok(parsed) = serde_json::from_str::<JudgeResponse>(json_str) {
            return Ok(parsed);
        }
    }

    let verdict = infer_verdict_from_reasoning(reasoning)?;
    let score = infer_score_from_reasoning(reasoning).unwrap_or_else(|| match verdict {
        JudgeVerdict::Pass => 1.0,
        JudgeVerdict::Partial => 0.5,
        JudgeVerdict::Fail => 0.0,
    });
    let detailed_reason = normalize_reasoning(reasoning);
    let reason = infer_reason_from_reasoning(&detailed_reason)
        .unwrap_or_else(|| summarize_reasoning(&detailed_reason));
    let matched_points = infer_list_points(&detailed_reason, &["matched_points", "matched points", "命中要点", "符合要点"]);
    let missing_points = infer_list_points(&detailed_reason, &["missing_points", "missing points", "缺失要点", "遗漏要点"]);

    Ok(JudgeResponse {
        verdict,
        score,
        reason,
        detailed_reason: Some(detailed_reason),
        matched_points,
        missing_points,
    })
}

fn infer_verdict_from_reasoning(reasoning: &str) -> Result<JudgeVerdict> {
    let normalized = normalize_reasoning(reasoning);

    for line in normalized.lines().rev() {
        if let Some(verdict) = infer_verdict_from_line(line) {
            return Ok(verdict);
        }
    }

    let lowered = normalized.to_ascii_lowercase();
    if lowered.contains("it is a pass")
        || lowered.contains("it's a pass")
        || lowered.contains("therefore, it is a \"pass\"")
        || lowered.contains("therefore, it is pass")
        || normalized.contains("因此判定为通过")
        || normalized.contains("因此应判定为通过")
    {
        return Ok(JudgeVerdict::Pass);
    }
    if lowered.contains("it is a partial")
        || lowered.contains("it's a partial")
        || normalized.contains("因此判定为部分正确")
        || normalized.contains("因此应判定为部分正确")
    {
        return Ok(JudgeVerdict::Partial);
    }
    if lowered.contains("it is a fail")
        || lowered.contains("it's a fail")
        || normalized.contains("因此判定为失败")
        || normalized.contains("因此应判定为失败")
    {
        return Ok(JudgeVerdict::Fail);
    }

    Err(anyhow!(
        "Failed to infer judge verdict from reasoning_content: {}",
        summarize_reasoning(reasoning)
    ))
}

fn infer_score_from_reasoning(reasoning: &str) -> Option<f64> {
    let normalized = normalize_reasoning(reasoning);

    for line in normalized.lines().rev() {
        let lowered = line.to_ascii_lowercase();
        if !(lowered.contains("score") || line.contains("分数")) {
            continue;
        }
        if lowered.contains("0-1") || lowered.contains("0 到 1") || lowered.contains("0 to 1") {
            continue;
        }
        if let Some(value) = extract_last_score_value(line) {
            return Some(value);
        }
    }

    None
}

fn infer_reason_from_reasoning(reasoning: &str) -> Option<String> {
    for line in reasoning.lines().rev() {
        let lowered = line.to_ascii_lowercase();
        if !(lowered.contains("`reason`")
            || lowered.contains("reason:")
            || lowered.contains("reason：")
            || line.contains("理由：")
            || line.contains("理由:"))
        {
            continue;
        }
        if lowered.contains("matched_points") || lowered.contains("missing_points") {
            continue;
        }
        if let Some(value) = extract_value_after_separator(line) {
            return Some(value);
        }
    }

    None
}

fn infer_list_points(reasoning: &str, markers: &[&str]) -> Vec<String> {
    let lines: Vec<&str> = reasoning.lines().collect();
    for (index, line) in lines.iter().enumerate().rev() {
        let lowered = line.to_ascii_lowercase();
        if !markers.iter().any(|marker| lowered.contains(&marker.to_ascii_lowercase()) || line.contains(marker)) {
            continue;
        }

        let mut items = Vec::new();
        for next_line in lines.iter().skip(index + 1) {
            let trimmed = next_line.trim();
            if trimmed.is_empty() {
                if !items.is_empty() {
                    break;
                }
                continue;
            }
            if is_section_header(trimmed) {
                break;
            }
            if let Some(point) = strip_list_marker(trimmed) {
                items.push(point.to_string());
            } else if !items.is_empty() {
                break;
            }
        }
        return items;
    }

    Vec::new()
}

fn summarize_reasoning(reasoning: &str) -> String {
    let cleaned = normalize_reasoning(reasoning);
    let mut summary = cleaned.chars().take(280).collect::<String>();
    if cleaned.chars().count() > 280 {
        summary.push_str("...");
    }
    summary
}

fn normalize_reasoning(reasoning: &str) -> String {
    reasoning.lines().map(str::trim).collect::<Vec<_>>().join("\n")
}

fn infer_verdict_from_line(line: &str) -> Option<JudgeVerdict> {
    let lowered = line.to_ascii_lowercase();
    if lowered.contains("pass/partial/fail") || lowered.contains("pass, partial, fail") {
        return None;
    }
    if !(lowered.contains("verdict")
        || lowered.contains("determine verdict")
        || lowered.contains("is it a")
        || line.contains("判定")
        || line.contains("结论"))
    {
        return None;
    }

    let mut candidates = Vec::new();
    for (needle, verdict) in [
        ("partial", JudgeVerdict::Partial),
        ("fail", JudgeVerdict::Fail),
        ("pass", JudgeVerdict::Pass),
        ("部分正确", JudgeVerdict::Partial),
        ("失败", JudgeVerdict::Fail),
        ("通过", JudgeVerdict::Pass),
    ] {
        if let Some(index) = lowered.rfind(needle) {
            candidates.push((index, verdict));
        }
    }

    candidates.sort_by_key(|(index, _)| *index);
    candidates.last().map(|(_, verdict)| verdict.clone())
}

fn extract_last_score_value(line: &str) -> Option<f64> {
    let mut parsed = None;
    for token in line.split(|ch: char| !(ch.is_ascii_digit() || ch == '.')) {
        if token.is_empty() {
            continue;
        }
        if let Ok(value) = token.parse::<f64>() {
            if (0.0..=1.0).contains(&value) {
                parsed = Some(value);
            }
        }
    }
    parsed
}

fn extract_value_after_separator(line: &str) -> Option<String> {
    for separator in [":", "："] {
        if let Some((_, value)) = line.split_once(separator) {
            let cleaned = value
                .trim()
                .trim_matches('`')
                .trim_matches('"')
                .trim_matches('*')
                .trim();
            if !cleaned.is_empty() {
                return Some(cleaned.to_string());
            }
        }
    }
    None
}

fn is_section_header(line: &str) -> bool {
    let lowered = line.to_ascii_lowercase();
    lowered.starts_with("1.")
        || lowered.starts_with("2.")
        || lowered.starts_with("3.")
        || lowered.starts_with("4.")
        || lowered.starts_with("5.")
        || lowered.starts_with("6.")
        || lowered.starts_with("7.")
        || lowered.starts_with("8.")
}

fn strip_list_marker(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    for prefix in ["*", "-", "•"] {
        if let Some(rest) = trimmed.strip_prefix(prefix) {
            let value = rest.trim();
            if !value.is_empty() {
                return Some(value);
            }
        }
    }
    None
}

fn extract_json_object(input: &str) -> Option<&str> {
    let start = input.find('{')?;
    let end = input.rfind('}')?;
    if end <= start {
        return None;
    }
    Some(&input[start..=end])
}

#[cfg(test)]
mod tests {
    use super::{infer_score_from_reasoning, infer_verdict_from_reasoning, parse_reasoning_fallback};
    use crate::models::JudgeVerdict;

    #[test]
    fn infers_pass_from_reasoning_text() {
        let reasoning = "The actual reply covers the expected answer and is semantically aligned. Verdict: pass.";
        let verdict = infer_verdict_from_reasoning(reasoning).unwrap();
        assert!(matches!(verdict, JudgeVerdict::Pass));
    }

    #[test]
    fn falls_back_to_reasoning_summary() {
        let reasoning = "The actual reply covers the expected answer and is semantically aligned. Verdict: pass.";
        let result = parse_reasoning_fallback(reasoning).unwrap();
        assert!(matches!(result.verdict, JudgeVerdict::Pass));
        assert_eq!(result.score, 1.0);
        assert_eq!(result.detailed_reason.as_deref(), Some(reasoning));
    }

    #[test]
    fn ignores_schema_line_and_uses_final_verdict() {
        let reasoning = "Fields: `verdict` (pass/partial/fail), `score` (0-1).\n5. Determine Verdict and Score:\nTherefore, it is a \"pass\".\n6. Draft the JSON fields:\n* `verdict`: \"pass\"\n* `score`: 1.0";
        let verdict = infer_verdict_from_reasoning(reasoning).unwrap();
        let score = infer_score_from_reasoning(reasoning).unwrap();
        assert!(matches!(verdict, JudgeVerdict::Pass));
        assert_eq!(score, 1.0);
    }

    #[test]
    fn partial_without_explicit_score_uses_default_score() {
        let reasoning = "Fields: `verdict` (pass/partial/fail), `score` (0-1).\n5. Determine Verdict:\nIs it a `partial`? Yes.\n";
        let result = parse_reasoning_fallback(reasoning).unwrap();
        assert!(matches!(result.verdict, JudgeVerdict::Partial));
        assert_eq!(result.score, 0.5);
    }
}
