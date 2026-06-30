# Bot 房间顶部公示栏 — 设计文档

- 日期: 2026-06-30
- 状态: 已确认（待 spec 评审）
- 关联代码: `src/app/features/room/Room.tsx`、`src/app/hooks/useClientConfig.ts`、`config.json`

## 1. 目标

在 sex-support bot 房间顶部（顶栏与消息列表之间）显示一条单行公示栏横幅，文案来自 Cinny 部署时的 `config.json`。横幅在每个浏览器会话内**只显示一次**：进入任一 bot 房间时弹出，**10 秒后自动关闭**，用户也可点 ✕ 立即关闭；本会话内不再出现，刷新页面后重置。更新公告 = 改 `config.json` + 重新部署 Cinny，无需运行时网络请求。

## 2. 代码现状（已核实）

- 房间视图主组件 `src/app/features/room/Room.tsx:20-75`。非 call 分支渲染层级：`RoomViewHeader`（line 57）→ `Box grow` → `RoomView`（line 59）。`callView` 分支（line 47-54）走 `CallView`，不含普通消息视图。
- 顶栏组件 `RoomViewHeader.tsx`；顶栏与时间线之间**无任何现成横幅**。`RoomTombstone.tsx` 是底部 state-event 驱动横幅的范式（本设计不沿用其 state-event 机制，仅参考布局）。
- 房间切换生命周期：`RoomProvider key={room.roomId}`（`pages/client/home/RoomProvider.tsx:30`）→ 切房时 `<Room>` 子树重挂载。进 bot 房 = `RoomAnnouncementBar` 挂载。
- 客户端配置：`config.json` 由 `ClientConfigLoader.tsx:6-10` 在启动时 `fetch` 一次，结果经 `ClientConfigProvider` 注入 context；`useClientConfig()`（`useClientConfig.ts:27-31`）读取。`ClientConfig` 类型见 `useClientConfig.ts:8-21`。**会话内 config 恒定**（无重新拉取）。
- bot 房间判定：`roomHasBot(room, botUserIds)`（`src/app/features/room/message/continuation.ts:74-82`）；`botUserIds` 来自 `settingsAtom`（`state/settings.ts`），经 `useSetting(settingsAtom, 'botUserIds')` 读取。
- folds 组件：`Box`/`Text`/`IconButton`/`Icon`/`Icons.Cross` 均已在项目内使用（见 `NewContextHint.tsx`、`RoomViewHeader.tsx`）。容器上色用项目自带的 `ContainerColor` CSS recipe（`src/app/styles/ContainerColor.css.ts`，`RoomViewHeader.tsx:51` 即 `ContainerColor({variant:'Surface'})`）。folds 无独立 `Surface` 组件。
- 项目无测试运行器（`package.json` 无 `test` 脚本）。自动化门禁：`yarn typecheck`、`yarn lint`、`yarn build`。

## 3. 决策（已与用户确认）

| 项 | 决策 |
|---|---|
| 内容来源 | `config.json` 静态字段 `announcement?: string`；启动时随 config 一起加载进 context |
| 更新方式 | 改 `config.json` + 重新部署；运行时无网络请求、无轮询 |
| 作用范围 | 全局一条文案，所有 bot 房间共用 |
| 显示时机 | 进入 bot 房间（组件挂载）且 `announcement` 非空 |
| 显示频率 | **每会话一次**：`sessionStorage` 标志位控制；刷新页面重置 |
| 自动关闭 | 显示后 10s 自动隐藏 |
| 手动关闭 | ✕ 按钮立即隐藏 |
| 再显示 | 本会话内不再显示；刷新页面后下次进 bot 房再显示一次 |
| 可见性门控 | bot 房间（`roomHasBot`）∧ `announcement` 非空 ∧ 本会话未展示过 |
| 外观 | 单行横幅：左侧文案（truncate 省略）+ 右侧 ✕；`Box` + `ContainerColor` recipe（`SurfaceVariant` 底色） |
| 测试 | 手动验证清单 + `typecheck`/`lint`/`build` |

## 4. 架构

```
config.json (announcement) ──boot fetch──▶ ClientConfig context
                                              │  useClientConfig()
Room.tsx (非 call 分支):                        │
  RoomViewHeader                              ▼
  RoomAnnouncementBar ──▶ isBotRoom? announcement非空? sessionStorage未标记?
                          │ yes → setVisible(true) + 记 sessionStorage + 10s timer
                          │ ✕ / 10s / 卸载 → setVisible(false) + clearTimeout
                          │ no  → 不渲染
  RoomView
```

新增/修改文件：

```
src/app/features/room/RoomAnnouncementBar.tsx   # 新增：横幅组件（含显示/关闭/会话去重逻辑）
src/app/hooks/useClientConfig.ts                # 修改：ClientConfig 类型加 announcement?: string
config.json                                      # 修改：加 "announcement" 字段（部署方填）
src/app/features/room/Room.tsx                  # 修改：非 call 分支 RoomViewHeader 后插入组件
```

无新增 state 文件、无 provider、无网络层——复用现有 `ClientConfig` context 与 `roomHasBot`。

## 5. 组件详述

### 5.1 `ClientConfig` 类型扩展（`useClientConfig.ts`）

在 `ClientConfig`（line 8-21）加一个可选字段：

```ts
export type ClientConfig = {
  defaultHomeserver?: number;
  homeserverList?: string[];
  allowCustomHomeservers?: boolean;
  featuredCommunities?: { /* ... */ };
  hashRouter?: HashRouterConfig;
  announcement?: string;   // 新增：bot 房间顶部公示栏文案；空/缺省 = 关闭功能
};
```

`ClientConfigLoader` 无需改动——它已把整个 config.json 透传进 context（`ClientConfigLoader.tsx:35-37`）。

### 5.2 `config.json` 字段

```json
{
  "defaultHomeserver": 0,
  "homeserverList": ["https://sexbot.mx.zztweb.top"],
  "allowCustomHomeservers": false,
  "announcement": "询问新问题前推荐点击 /new 清理上下文"
}
```

- `announcement` 为空串 `""` 或缺省 → 功能关闭，横幅永不渲染、`sessionStorage` 不写入。
- 文案纯字符串，单行展示；过长由 UI `truncate` 省略（不做多行/展开）。

### 5.3 横幅组件 `RoomAnnouncementBar.tsx`

经典 JSX（项目 `tsconfig` 为 `jsx: "react"`），需 `import React`。

```tsx
import React, { useCallback, useEffect, useMemo, useState } from 'react';
import { Box, Icon, IconButton, Icons, Text, config } from 'folds';
import { Room } from 'matrix-js-sdk';
import { useClientConfig } from '../../hooks/useClientConfig';
import { useSetting } from '../../state/hooks/settings';
import { settingsAtom } from '../../state/settings';
import { roomHasBot } from './message/continuation';
import { ContainerColor } from '../../styles/ContainerColor.css';

const SESSION_KEY = 'cinny.announcement.shown';
const AUTO_CLOSE_MS = 10 * 1000;

export function RoomAnnouncementBar({ room }: { room: Room }) {
  const { announcement } = useClientConfig();
  const [botUserIds] = useSetting(settingsAtom, 'botUserIds');
  const isBotRoom = useMemo(() => roomHasBot(room, botUserIds), [room, botUserIds]);
  const [visible, setVisible] = useState(false);

  // 决定是否显示：进 bot 房、文案非空、本会话未展示过 → 显示并记标志（只跑一次）
  useEffect(() => {
    if (!announcement || !isBotRoom) return;
    if (sessionStorage.getItem(SESSION_KEY)) return;
    sessionStorage.setItem(SESSION_KEY, '1');
    setVisible(true);
  }, [announcement, isBotRoom]);

  // 自动关闭：visible 变 true 时启 10s 定时器，到点或卸载时清理（与显示决策解耦，规避 StrictMode 双调用）
  useEffect(() => {
    if (!visible) return;
    const timer = window.setTimeout(() => setVisible(false), AUTO_CLOSE_MS);
    return () => window.clearTimeout(timer);
  }, [visible]);

  const dismiss = useCallback(() => setVisible(false), []);

  if (!visible || !announcement) return null;

  return (
    <Box
      className={ContainerColor({ variant: 'SurfaceVariant' })}
      direction="Row"
      alignItems="Center"
      gap="200"
      shrink="No"
      style={{ padding: `${config.space.S200} ${config.space.S400}` }}
    >
      <Text size="T200" truncate>
        {announcement}
      </Text>
      <IconButton
        onClick={dismiss}
        variant="SurfaceVariant"
        size="300"
        radii="300"
        aria-label="关闭公示"
        shrink="No"
      >
        <Icon src={Icons.Cross} size="50" />
      </IconButton>
    </Box>
  );
}
```

说明：
- `useEffect` 依赖 `[announcement, isBotRoom]`：挂载即判定（进 bot 房）。`announcement` 会话内恒定；`isBotRoom` 经 `useMemo` 缓存避免每次渲染重算 `room.getMembers()`。
- **显示决策与定时器解耦**：第一个 effect 只负责"是否显示 + 写 `sessionStorage` 标志"（StrictMode 双调用下，第二次因标志已存在而早返，不会重复显示）；第二个 effect 只在 `visible` 变 true 时启 timer，到点或卸载清理。两者解耦避免 dev 模式下"timer 被 cleanup 清掉却不再重启、横幅不自动关闭"的竞态。
- **会话去重**：进 bot 房且文案非空且 `sessionStorage` 无标志 → 写标志 + `setVisible(true)`。本会话再进任何 bot 房，标志已存在 → 第一个 effect 早返，不再显示。
- `dismiss` 仅 `setVisible(false)`；第二个 effect 的 cleanup 会清掉在途 timer，无泄漏。手动 ✕ 与 10s 超时都只置 `visible=false`，不操作 `sessionStorage`（标志已在显示时写入，保证"本会话一次"）。
- `if (!visible || !announcement) return null`：非 bot 房、无文案、已关闭时都不渲染，布局零影响。
- **容器上色**：folds 无 `Surface` 组件，`Surface`/`SurfaceVariant` 是 `ContainerColor` 的 variant 值。用 `Box` + 项目自带的 `ContainerColor` CSS recipe（`src/app/styles/ContainerColor.css.ts`）上色，与 `RoomViewHeader` 的 `ContainerColor({variant:'Surface'})` 同源；横幅用 `SurfaceVariant` 取得与顶栏有微妙区分的底色。`Text truncate` 单行省略。

### 5.4 `Room.tsx` 集成

在非 call 分支（line 55-62）`<RoomViewHeader />`（line 57）之后插入：

```tsx
{!callView && (
  <Box grow="Yes" direction="Column">
    <RoomViewHeader />
    <RoomAnnouncementBar room={room} />
    <Box grow="Yes">
      <RoomView eventId={eventId} />
    </Box>
  </Box>
)}
```

- 仅非 call 房间显示（call 房间走 `CallView` 分支，无普通消息视图）。
- 顶部新增 `import { RoomAnnouncementBar } from './RoomAnnouncementBar';`。
- 组件返回 `null` 时不占位、不影响 `RoomView` 高度。

## 6. 数据流

1. 启动 → `ClientConfigLoader` fetch `/config.json` → `announcement` 进 context。
2. 进 bot 房 → `<Room>` 重挂 → `RoomAnnouncementBar` 挂载 → `useEffect` 判定：文案非空 ∧ bot 房 ∧ `sessionStorage` 无标志 → 显示 + 写标志 + 启 10s 定时器。
3. 10s 到 / 点 ✕ → `setVisible(false)` → 横幅消失。
4. 本会话再进 bot 房 → `sessionStorage` 有标志 → `useEffect` 早返，不显示。
5. 刷新页面 → `sessionStorage` 清空 → 下次进 bot 房再显示一次（若 `announcement` 仍非空）。
6. 部署方改 `config.json` 并重新部署 → 下次启动加载新文案 → 正常显示。

## 7. 边界与错误处理

- **`announcement` 空/缺省**：`useEffect` 早返，永不显示，`sessionStorage` 不写。功能静默关闭。
- **非 bot 房间**：`isBotRoom=false` → `useEffect` 早返，不显示、不写标志、不计时。
- **call 房间**：走 `callView` 分支，组件不挂载，无开销。
- **长文案**：`Text truncate` 单行省略，不换行、不撑高横幅。
- **手动关闭后停留本房**：`visible=false` 保持，不再出现（`sessionStorage` 已写、`useEffect` 依赖未变不重跑）。
- **多标签**：`sessionStorage` 按标签独立 → 每个新标签各显示一次（符合"每会话一次"，标签间互不影响）。
- **挂载竞态/快速切房**：`RoomProvider key={roomId}` 重挂，旧实例 cleanup 清 timer，新实例重新判定；`sessionStorage` 保证已展示则不重弹。
- **无网络失败面**：不发起运行时请求；config.json 加载失败由 `ClientConfigLoader` 现有 `error` 兜底处理（与本功能无关，此时 `announcement` 为 `undefined` → 不显示）。

## 8. 测试

项目无测试运行器。自动化：`yarn typecheck`、`yarn lint`、`yarn build` 须通过。

手动验证清单：
1. `config.json` 填非空 `announcement`，刷新进 bot 房 → 顶部出现横幅，文案正确。
2. 等 10s → 横幅自动消失。
3. 点 ✕ → 立即消失。
4. 本会话内切换到另一 bot 房、再切回 → 不再出现（`sessionStorage` 生效）。
5. 刷新页面 → 下次进 bot 房再次出现一次。
6. `announcement` 设为 `""` 或删除字段 → 进 bot 房不显示，DevTools `sessionStorage` 无 `cinny.announcement.shown`。
7. 进非 bot 房间 → 不显示。
8. 长文案 → 单行省略，横幅不换行不撑高。
9. 多标签：A 标签已显示并关闭，新开 B 标签进 bot 房 → B 仍会显示一次。

## 9. 范围外（YAGNI）

- 不做运行时 fetch / 轮询 / 外部端点 / CORS（文案随部署静态写入 config）。
- 不做标题/正文/链接/严重度等多字段——纯单行字符串。
- 不做"已读版本"持久化（`localStorage` 跨会话）——`sessionStorage` 每会话一次即满足需求。
- 不做跨标签同步去重（每标签独立显示一次，符合会话语义）。
- 不做可配置的自动关闭时长 / 显示频率（常量写死 10s、每会话一次）。
- 不在 call 房间显示。
- 不做 Markdown / 富文本渲染（纯文本）。
