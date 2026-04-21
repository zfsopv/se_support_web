# Matrix FAQ Test Tool

一个独立的 Matrix 技术支持助手自动测试工具，用于从 FAQ Markdown 文档提取问题，向指定 Matrix 房间逐题发问，并使用 OpenAI API 判断房间回复与 FAQ 标准答案是否语义等价。

## 功能

- 使用 Matrix 测试账号登录 homeserver
- 查询已加入房间并交互选择测试房间
- 解析类似 `#### 问题` + `答：答案` 的 FAQ Markdown 文档
- 将所有提取出的 FAQ 问题发送到指定房间
- 等待房间回复，并将回复与标准答案提交到 OpenAI 评判
- 输出终端汇总报告，同时保存 JSON 和 CSV 报告

## 环境变量

参考 `.env.example`：

- `HOMESERVER`
- `TEST_USER`
- `TEST_PASSWORD`
- `OPENAI_API_KEY`
- `OPENAI_MODEL`
- `OPENAI_BASE_URL`
- `REPLY_TIMEOUT_SECONDS`
- `SYNC_TIMEOUT_MS`
- `OUTPUT_DIR`

## 运行

```bash
cargo run -- --faq-file ../../docs/docs/src/content/docs/se7/10-media-faq.md
```

如果需要自定义输出目录：

```bash
cargo run -- --faq-file ../../docs/docs/src/content/docs/se7/10-media-faq.md --output-dir ./reports/se7
```

## FAQ 文档格式假设

第一版按以下结构提取：

- `##` 作为大类
- `###` 作为子类
- `####` 作为问题标题
- 问题标题后的内容直到下一个标题作为答案块
- 如果答案块第一行以 `答：` 或 `答:` 开头，会去掉该前缀

这意味着像“现象/解决”这种没有 `答：` 前缀的块也会被保留并参与评判。

## 输出

- 终端汇总统计
- `matrix_faq_report_*.json`
- `matrix_faq_report_*.csv`

### CSV 表头

CSV 报告当前固定输出以下表头：

```text
source_file,room_id,question_index,section,subsection,question,expected_answer,actual_reply,reply_sender,status,score,reason,detailed_reason,matched_points,missing_points,latency_ms,started_at,completed_at
```

### CSV 内容说明

- `source_file`：本次测试使用的 FAQ 文件路径
- `room_id`：执行测试的 Matrix 房间 ID
- `question_index`：问题在 FAQ 提取结果中的顺序编号
- `section`：FAQ 中对应的 `##` 大类标题
- `subsection`：FAQ 中对应的 `###` 子类标题
- `question`：发送到房间中的问题文本
- `expected_answer`：从 FAQ 文档提取出的标准答案原文
- `actual_reply`：房间中捕获到的首条有效文本回复；超时或发送失败时为空
- `reply_sender`：回复消息的发送者 Matrix 用户 ID；未收到回复时为空
- `status`：本题最终状态，可能为 `Passed`、`Partial`、`Failed`、`Timeout`、`SendError`、`JudgeError`
- `score`：评判模型返回的分数，范围通常为 `0` 到 `1`；未成功评判时为空
- `reason`：简短评分理由，适合快速浏览；出错时这里会写错误说明
- `detailed_reason`：详细评分理由，保留评判模型返回的完整说明文本，便于复盘
- `matched_points`：评判模型认为“已命中”的要点列表，换行分隔；无内容时为空
- `missing_points`：评判模型认为“缺失”的要点列表，换行分隔；无内容时为空
- `latency_ms`：从发送问题到收到有效回复的耗时，单位毫秒；未收到回复时为空
- `started_at`：本题开始处理时间，RFC 3339 格式
- `completed_at`：本题完成处理时间，RFC 3339 格式

### CSV 示例

```csv
source_file,room_id,question_index,section,subsection,question,expected_answer,actual_reply,reply_sender,status,score,reason,detailed_reason,matched_points,missing_points,latency_ms,started_at,completed_at
../../docs/docs/src/content/docs/se7/10-media-faq.md,!room:example.org,1,视频问题,解码异常,如何判断视频花屏的原因,请先确认输入源和编码格式是否正常。,请检查输入源和编码格式。,@bot:example.org,Passed,1,语义一致,"回复覆盖了 FAQ 的核心判断条件，因此可视为语义一致。","输入源检查\n编码格式检查","",1250,2026-04-15T09:32:10Z,2026-04-15T09:32:11Z
```

## 已知限制

- 当前只支持单个 FAQ 文件
- 当前只读取文本消息 `m.text`
- 回复关联策略为“提问后收到的首条非测试账号文本回复”
- 房间枚举基于标准 Matrix Client API，不依赖 Synapse 管理员接口
