use crate::models::FaqItem;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn parse_faq_file(path: &Path) -> Result<Vec<FaqItem>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read FAQ file {}", path.display()))?;
    parse_faq_markdown(&content)
}

fn parse_faq_markdown(content: &str) -> Result<Vec<FaqItem>> {
    let normalized = content.replace("\r\n", "\n");
    let body = strip_frontmatter(&normalized);
    let lines: Vec<&str> = body.lines().collect();

    let mut items = Vec::new();
    let mut section = String::new();
    let mut subsection = String::new();
    let mut index = 1usize;
    let mut cursor = 0usize;

    while cursor < lines.len() {
        let line = lines[cursor].trim_end();

        if let Some((level, title)) = parse_heading(line) {
            let next_nonempty = next_nonempty_line(&lines, cursor + 1);
            let is_question = match level {
                4 => true,
                3 | 2 => next_nonempty.is_some_and(|next| parse_heading(next).is_none()),
                _ => false,
            };

            if is_question {
                cursor += 1;
                let start = cursor;

                while cursor < lines.len() {
                    let next = lines[cursor];
                    if parse_heading(next).is_some() {
                        break;
                    }
                    cursor += 1;
                }

                let answer_block = normalize_answer_block(&lines[start..cursor]);
                if answer_block.is_empty() {
                    continue;
                }

                let (item_section, item_subsection) = match level {
                    4 => (section.clone(), subsection.clone()),
                    3 => (section.clone(), String::new()),
                    2 => (String::new(), String::new()),
                    _ => (section.clone(), subsection.clone()),
                };

                items.push(FaqItem {
                    index,
                    section: item_section,
                    subsection: item_subsection,
                    question: title.to_string(),
                    answer_text: collapse_whitespace(&answer_block),
                    answer_markdown: answer_block,
                });
                index += 1;
                continue;
            }

            match level {
                2 => {
                    section = title.to_string();
                    subsection.clear();
                }
                3 => {
                    subsection = title.to_string();
                }
                _ => {}
            }
            cursor += 1;
            continue;
        }

        cursor += 1;
    }

    Ok(items)
}

fn parse_heading(line: &str) -> Option<(usize, &str)> {
    let trimmed = line.trim_end();
    if let Some(value) = trimmed.strip_prefix("#### ") {
        return Some((4, value.trim()));
    }
    if let Some(value) = trimmed.strip_prefix("### ") {
        return Some((3, value.trim()));
    }
    if let Some(value) = trimmed.strip_prefix("## ") {
        return Some((2, value.trim()));
    }
    None
}

fn next_nonempty_line<'a>(lines: &'a [&'a str], start: usize) -> Option<&'a str> {
    lines[start..]
        .iter()
        .map(|line| line.trim_end())
        .find(|line| !line.trim().is_empty())
}

fn strip_frontmatter(content: &str) -> &str {
    let trimmed = content.trim_start_matches('\u{feff}');
    if !trimmed.starts_with("---\n") {
        return trimmed;
    }

    let remainder = &trimmed[4..];
    if let Some(end) = remainder.find("\n---\n") {
        &remainder[end + 5..]
    } else {
        trimmed
    }
}

fn normalize_answer_block(lines: &[&str]) -> String {
    let mut start = 0usize;
    while start < lines.len() && lines[start].trim().is_empty() {
        start += 1;
    }

    if start >= lines.len() {
        return String::new();
    }

    let mut collected: Vec<String> = lines[start..]
        .iter()
        .map(|line| (*line).to_string())
        .collect();
    if let Some(first) = collected.first_mut() {
        if let Some(rest) = first.strip_prefix("答：") {
            *first = rest.trim_start().to_string();
        } else if let Some(rest) = first.strip_prefix("答:") {
            *first = rest.trim_start().to_string();
        }
    }

    while collected.first().is_some_and(|line| line.trim().is_empty()) {
        collected.remove(0);
    }
    while collected.last().is_some_and(|line| line.trim().is_empty()) {
        collected.pop();
    }

    collected.join("\n")
}

fn collapse_whitespace(input: &str) -> String {
    input
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::parse_faq_markdown;

    #[test]
    fn parses_questions_and_answers() {
        let input = r#"---
title: Demo
---

## Section A
### Common
#### Question One
答：Answer one.

#### Question Two
现象：something
解决：do something
"#;

        let items = parse_faq_markdown(input).expect("parse success");
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].question, "Question One");
        assert_eq!(items[0].answer_markdown, "Answer one.");
        assert!(items[1].answer_markdown.contains("解决：do something"));
    }

    #[test]
    fn parses_level_three_questions() {
        let input = r#"## 刷机相关问题

### 刷机包在哪里

刷机包在官网资料包中。

### 如何使用TF卡进行刷机

1. 准备一张TF卡
2. 拷贝刷机包
"#;

        let items = parse_faq_markdown(input).expect("parse success");
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].section, "刷机相关问题");
        assert_eq!(items[0].subsection, "");
        assert_eq!(items[0].question, "刷机包在哪里");
        assert!(items[1].answer_markdown.contains("准备一张TF卡"));
    }

    #[test]
    fn parses_level_two_questions() {
        let input = r#"## 如何恢复出厂设置

请先备份数据，然后执行恢复脚本。

## 如何查看版本信息

执行 bm_version 命令。
"#;

        let items = parse_faq_markdown(input).expect("parse success");
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].section, "");
        assert_eq!(items[0].subsection, "");
        assert_eq!(items[0].question, "如何恢复出厂设置");
        assert_eq!(items[1].question, "如何查看版本信息");
    }
}
