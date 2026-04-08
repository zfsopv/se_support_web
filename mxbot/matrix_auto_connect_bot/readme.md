# 自动建立matrix私聊房间

## 功能说明

本程序会使用管理员账户登录matrix服务器（synapse），然后使用synapse专有的API每隔5s全量巡检用户与支持 bot 的私聊关系。对于每个「非白名单用户 × 每个支持 bot」组合，程序会确保双方只有一个双人私聊房间：

1. 如果不存在双人私聊房间：自动创建
2. 如果存在多个双人私聊房间：保留一个并自动去重（将 bot 移出多余房间）

注意：

1. 默认不开启E2EE加密
2. 不再区分“新用户”，而是每轮全量校正
3. 创建私聊房间后如果管理员账户仍然在房间中，会离开房间
4. 可通过 `SUPPORT_BOTS` 配置需要自动建私聊的 bot
5. 可通过 `USER_WHITELIST` 配置不参与巡检的用户

## 程序语言和编译方式

程序采用rust语言，使用cargo进行编译

## 配置说明

程序通过环境变量进行配置，可以在 `.env` 文件中设置以下变量：

| 环境变量 | 说明 | 示例 | 默认值 |
|---------|------|------|--------|
| HOMESERVER | Matrix服务器地址 | `http://localhost:8008` | `http://localhost:8008` |
| ADMIN_USER | 管理员账户ID | `@admin:localhost` | `@admin:localhost` |
| ADMIN_PASSWORD | 管理员密码 | `your_password` | 无（必须设置） |
| SUPPORT_BOTS | 参与自动建私聊的机器人列表（逗号分隔） | `se7-support-bot,se9-support-bot` | `se7-support-bot,se9-support-bot` |
| USER_WHITELIST | 不参与巡检的用户白名单（逗号分隔） | `admin,test-user` | 空 |
| POLL_INTERVAL_SECONDS | 巡检间隔（秒） | `10` | `5` |

**注意：**
- `SUPPORT_BOTS` / `USER_WHITELIST` 支持两种格式：
  - 简写用户名：如 `se7-support-bot`（会自动拼接为 `@se7-support-bot:<homeserver>`）
  - 完整形式：如 `@se7-support-bot:example.com` 或兼容写法 `!se7-support-bot@example.com`
- `ADMIN_USER` 也支持以上格式（推荐使用完整形式）

**配置示例文件 `.env`：**
```bash
HOMESERVER=http://localhost:8008
ADMIN_USER=@admin:localhost
ADMIN_PASSWORD=your_admin_password_here
SUPPORT_BOTS=se7-support-bot,se9-support-bot
USER_WHITELIST=admin,test-user
POLL_INTERVAL_SECONDS=5
```

## 使用方式

### 安装依赖
确保已安装 Rust 和 Cargo：
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 编译程序
```bash
cd matrix_auto_connect_bot
cargo build --release
```

### 运行程序

**方式1：使用环境变量**
```bash
export HOMESERVER="http://localhost:8008"
export ADMIN_USER="@admin:localhost"
export ADMIN_PASSWORD="your_password"
export SUPPORT_BOTS="se7-support-bot,se9-support-bot"
export USER_WHITELIST="admin,test-user"
export POLL_INTERVAL_SECONDS="5"
cargo run --release
```

**方式2：使用 .env 文件**
```bash
cp .env.example .env
# 编辑 .env 文件，填入实际配置
cargo run --release
```

### 后台运行（推荐）
使用 systemd 或其他进程管理工具保持程序持续运行。

**systemd 服务示例：**
创建 `/etc/systemd/system/matrix-auto-connect-bot.service`：
```ini
[Unit]
Description=Matrix Auto Connect Bot
After=network.target

[Service]
Type=simple
User=your_user
WorkingDirectory=/path/to/matrix_auto_connect_bot
Environment="HOMESERVER=http://localhost:8008"
Environment="ADMIN_USER=@admin:localhost"
Environment="ADMIN_PASSWORD=your_password"
Environment="SUPPORT_BOTS=se7-support-bot,se9-support-bot"
Environment="USER_WHITELIST=admin,test-user"
Environment="POLL_INTERVAL_SECONDS=5"
ExecStart=/path/to/cargo run --release
Restart=always

[Install]
WantedBy=multi-user.target
```

启用服务：
```bash
sudo systemctl enable matrix-auto-connect-bot
sudo systemctl start matrix-auto-connect-bot
```