# Bot 房间顶部公示栏 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 在 bot 房间顶栏下方显示一条单行公示栏横幅，文案来自 `config.json` 的 `announcement` 字段；每会话只显示一次，10s 自动关闭，可 ✕ 手动关闭。

**Architecture:** 公告文案作为可选字段写入 `config.json`，启动时由现有 `ClientConfigLoader` 读进 `ClientConfig` context（无运行时网络请求）。新增 `RoomAnnouncementBar` 组件挂在 `Room.tsx` 的 `RoomViewHeader` 之后：进 bot 房（`roomHasBot`）且文案非空且 `sessionStorage` 无标志时显示一次并记标志；`visible` 变 true 时启 10s 定时器自动关闭；✕ 按钮立即关闭。更新公告 = 改 `config.json` + 重新部署。

**Tech Stack:** React 18 + TypeScript（`jsx: "react"` 经典模式，每个 `.tsx` 需 `import React`）、folds UI 库、Jotai、matrix-js-sdk、vanilla-extract（项目自带 `ContainerColor` CSS recipe）。

**Spec:** `docs/superpowers/specs/2026-06-30-bot-room-announcement-bar-design.md`

**测试说明（重要）:** 本项目**无测试运行器**（`package.json` 无 `test` 脚本、无 vitest/jest）。无法写可运行的单测。自动化门禁用 `yarn typecheck`（tsc `--noEmit`）+ `yarn build`（vite 构建，最强的结构正确性门禁）+ `yarn lint`。每个实现任务以 `yarn typecheck` 通过为收尾；最后有一个手动验证任务跑 spec 第 8 节清单。

**已知环境坑（不要被误导）:** `yarn typecheck` 可能报数百个 `matrix-js-sdk` "has no exported member" 错误——这是本机 matrix-js-sdk 未 `prepare`（`lib/index.d.ts` 是 stub）的既有环境问题，出现在**未触及的文件**里。**判定标准：只看本次新增/修改的文件是否有错**；若仅剩 matrix-js-sdk 相关错误且都在无关文件，视为通过。`yarn build` 是更可靠的门禁。

---

## 文件结构

| 文件 | 责任 | 动作 |
|---|---|---|
| `src/app/hooks/useClientConfig.ts` | `ClientConfig` 类型定义 + `useClientConfig()` hook | 修改：类型加 `announcement?: string` |
| `config.json` | 部署期静态配置 | 修改：加 `"announcement"` 字段 |
| `src/app/features/room/RoomAnnouncementBar.tsx` | 横幅组件：读 config + bot 房判定 + 显示/关闭/会话去重 | 新建 |
| `src/app/features/room/Room.tsx` | 房间视图装配 | 修改：非 call 分支 `RoomViewHeader` 后插入组件 |

无新增 state/provider/网络层。复用：`useClientConfig`、`useSetting(settingsAtom,'botUserIds')`、`roomHasBot`、`ContainerColor` recipe、folds `Box/Text/IconButton/Icon/Icons.Cross`。

---

## Task 1: 扩展 ClientConfig 类型并加 config.json 字段

**Files:**
- Modify: `src/app/hooks/useClientConfig.ts:8-21`（`ClientConfig` 类型）
- Modify: `config.json`（根目录）

- [ ] **Step 1: 在 `ClientConfig` 类型加 `announcement` 字段**

打开 `src/app/hooks/useClientConfig.ts`，在 `hashRouter?: HashRouterConfig;` 之后、类型闭合 `}` 之前加一行：

```ts
export type ClientConfig = {
  defaultHomeserver?: number;
  homeserverList?: string[];
  allowCustomHomeservers?: boolean;

  featuredCommunities?: {
    openAsDefault?: boolean;
    spaces?: string[];
    rooms?: string[];
    servers?: string[];
  };

  hashRouter?: HashRouterConfig;
  announcement?: string;
};
```

- [ ] **Step 2: 在 `config.json` 加 `announcement` 字段**

打开 `/home/zzt/workspace/sex_support_web/mxbot/cinny/config.json`，在 `hashRouter` 块之后加 `announcement` 字段（带尾随逗号）。完整文件应为：

```json
{
  "defaultHomeserver": 0,
  "homeserverList": ["https://sexbot.mx.zztweb.top"],
  "allowCustomHomeservers": false,

  "featuredCommunities": {
    "openAsDefault": false,
    "spaces": [],
    "rooms": [],
    "servers": []
  },

  "hashRouter": {
    "enabled": false,
    "basename": "/"
  },

  "announcement": "询问新的问题时，推荐点击输入框左侧的 /new 按钮清理上下文"
}
```

> 这是默认示例文案，部署方可随时改。空串 `""` 或删除该字段 = 关闭功能。

- [ ] **Step 3: 跑 typecheck 验证类型改动无误**

Run: `yarn typecheck`
Expected: 不应出现与 `useClientConfig.ts` 相关的新错误（既有 matrix-js-sdk 错误忽略，判定标准见计划顶部"已知环境坑"）。

- [ ] **Step 4: 跑 build 验证 config.json 是合法 JSON 且类型一致**

Run: `yarn build`
Expected: 构建成功（vite 会把 config.json 当静态资源拷贝，tsc 阶段校验类型）。若报 JSON 解析错，检查上一 step 的逗号/括号。

- [ ] **Step 5: Commit**

```bash
git add src/app/hooks/useClientConfig.ts config.json
git commit -m "$(cat <<'EOF'
feat: add announcement field to ClientConfig and config.json

Adds optional `announcement` string to ClientConfig type and a default
value in config.json. Consumed by the upcoming RoomAnnouncementBar.
Empty/absent = feature off.

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

---

## Task 2: 新建 RoomAnnouncementBar 组件

**Files:**
- Create: `src/app/features/room/RoomAnnouncementBar.tsx`

- [ ] **Step 1: 写组件文件**

创建 `src/app/features/room/RoomAnnouncementBar.tsx`，完整内容：

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

  // 决定是否显示：进 bot 房、文案非空、本会话未展示过 → 写标志并显示（只跑一次）。
  // 第二个 effect 单独管定时器，与显示决策解耦，规避 StrictMode 双调用下
  // "timer 被 cleanup 清掉却不再重启" 的竞态。
  useEffect(() => {
    if (!announcement || !isBotRoom) return;
    if (sessionStorage.getItem(SESSION_KEY)) return;
    sessionStorage.setItem(SESSION_KEY, '1');
    setVisible(true);
  }, [announcement, isBotRoom]);

  // 自动关闭：visible 变 true 时启 10s 定时器；到点或卸载时清理。
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

要点：
- `import React` 必需（`tsconfig` `jsx: "react"`）。
- `ContainerColor` 来自项目 `src/app/styles/ContainerColor.css.ts`（非 folds）。folds 无 `Surface` 组件，`SurfaceVariant` 是 `ContainerColor` 的 variant 值。
- `roomHasBot`、`useSetting`、`useClientConfig` 的导入路径与 `useNewContextHint.ts` 一致。
- 两个 effect 解耦显示决策与定时器（见注释）。
- `useMemo(isBotRoom)` 避免每次渲染重算 `room.getMembers()`。

- [ ] **Step 2: 跑 typecheck 验证新文件无类型错**

Run: `yarn typecheck`
Expected: 不应出现 `RoomAnnouncementBar.tsx` 相关错误。若报 `ContainerColor` 导入不存在，确认 `src/app/styles/ContainerColor.css.ts` 存在且导出 `ContainerColor`（见 spec 第 2 节引用）。既有 matrix-js-sdk 错误忽略。

- [ ] **Step 3: 跑 build 验证导入解析与 JSX 有效**

Run: `yarn build`
Expected: 构建成功。这是最强门禁：确认所有 import 能解析、JSX 合法、vanilla-extract recipe 调用有效。

- [ ] **Step 4: Commit**

```bash
git add src/app/features/room/RoomAnnouncementBar.tsx
git commit -m "$(cat <<'EOF'
feat: add RoomAnnouncementBar component

Single-line banner shown once per session on entering a bot room when
config.announcement is non-empty. Auto-closes after 10s, dismissible
via ✕. Visibility decision and auto-close timer are split into two
effects to stay correct under StrictMode double-invoke.

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

---

## Task 3: 在 Room.tsx 集成横幅

**Files:**
- Modify: `src/app/features/room/Room.tsx:1-19`（imports）和 `Room.tsx:55-62`（非 call 分支）

- [ ] **Step 1: 加 import**

打开 `src/app/features/room/Room.tsx`。在 `import { RoomViewHeader } from './RoomViewHeader';`（line 16）之后加一行：

```ts
import { RoomAnnouncementBar } from './RoomAnnouncementBar';
```

- [ ] **Step 2: 在非 call 分支的 RoomViewHeader 后插入组件**

定位非 call 分支（约 line 55-62）。把：

```tsx
        {!callView && (
          <Box grow="Yes" direction="Column">
            <RoomViewHeader />
            <Box grow="Yes">
              <RoomView eventId={eventId} />
            </Box>
          </Box>
        )}
```

改为：

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

> 仅非 call 分支插入。`callView` 分支（line 47-54）走 `CallView`，不插。组件在非 bot 房 / 空文案 / 已关闭时返回 `null`，不占位、不影响 `RoomView` 高度。`room` 变量在 `Room()` 顶部已由 `useRoom()` 取得（line 22）。

- [ ] **Step 3: 跑 typecheck 验证集成无误**

Run: `yarn typecheck`
Expected: 不应出现 `Room.tsx` 相关新错误（既有 matrix-js-sdk 错误忽略）。

- [ ] **Step 4: 跑 build 验证最终集成**

Run: `yarn build`
Expected: 构建成功。

- [ ] **Step 5: 跑 lint 确认无新增 lint 错**

Run: `yarn lint`
Expected: 不应在 `Room.tsx` / `RoomAnnouncementBar.tsx` 出现新 lint 错。若报既有基准债务（与本次无关的未使用导入等），不属本次回归；但本次新增文件本身应干净。

- [ ] **Step 6: Commit**

```bash
git add src/app/features/room/Room.tsx
git commit -m "$(cat <<'EOF'
feat: mount RoomAnnouncementBar below RoomViewHeader

Renders the announcement bar in the non-call room branch, directly
under the header. Returns null when not applicable so layout is
unaffected.

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

---

## Task 4: 手动验证（spec 第 8 节清单）

**Files:** 无（运行时验证）

项目无测试运行器，此任务用浏览器跑 spec 第 8 节的手动清单。

- [ ] **Step 1: 起开发服务器**

Run: `yarn dev`
Expected: vite dev 服务起来，浏览器打开 Cinny。

- [ ] **Step 2: 清会话状态，进 bot 房验证显示**

在浏览器 DevTools → Application → Session Storage 清掉 `cinny.announcement.shown`（若存在）。登录后进一个 bot 房间（含 `@se7-support-bot` 等的房间）。
Expected: 顶栏正下方出现一条 `SurfaceVariant` 底色横幅，文案为 `config.json` 的 `announcement` 值。

- [ ] **Step 3: 验证 10s 自动关闭**

保持不动，等 10 秒。
Expected: 横幅自动消失。

- [ ] **Step 4: 验证手动 ✕ 关闭**

清掉 Session Storage 的 `cinny.announcement.shown`，刷新页面，再进 bot 房（横幅重新出现）。点右侧 ✕。
Expected: 横幅立即消失。

- [ ] **Step 5: 验证本会话只显示一次**

横幅消失后（无论 auto 还是 ✕），切换到另一个 bot 房间、再切回。
Expected: 不再出现横幅（Session Storage 里已有 `cinny.announcement.shown`）。

- [ ] **Step 6: 验证刷新后重置**

刷新页面（F5），进 bot 房。
Expected: 横幅再次出现一次。

- [ ] **Step 7: 验证空文案 = 关闭功能**

把 `config.json` 的 `announcement` 改为 `""`，刷新页面，进 bot 房。
Expected: 不显示横幅；Session Storage 不写入 `cinny.announcement.shown`。

- [ ] **Step 8: 验证非 bot 房间不显示**

把 `announcement` 改回非空，刷新，进一个**不含 bot** 的普通房间。
Expected: 不显示横幅，Session Storage 不写入标志。

- [ ] **Step 9: 验证长文案单行省略**

把 `announcement` 改成一长串文字（如 200 字），刷新进 bot 房。
Expected: 横幅单行显示，超出部分省略号，横幅不换行不撑高。

- [ ] **Step 10: 验证多标签独立**

A 标签已显示并关闭后，新开 B 标签登录进 bot 房。
Expected: B 标签仍会显示一次（`sessionStorage` 按标签独立）。

- [ ] **Step 11: 还原 config.json 到默认文案**

把 `config.json` 的 `announcement` 还原成 Task 1 Step 2 的默认文案。若改过，commit：

```bash
git add config.json
git commit -m "$(cat <<'EOF'
chore: restore default announcement text after manual verification

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

（若未改过则跳过本步。）

- [ ] **Step 12: 停掉 dev 服务器**

停掉 `yarn dev`（Ctrl-C 或对应方式）。

---

## Self-Review

**1. Spec 覆盖：**
- §3 决策表"内容来源 config.json announcement" → Task 1。✓
- "作用范围全局" → 组件读同一 config 字段，所有 bot 房共用。✓
- "每会话一次 sessionStorage" → Task 2 第一个 effect。✓
- "10s 自动关闭" → Task 2 第二个 effect。✓
- "✕ 手动关闭" → Task 2 `dismiss` + IconButton。✓
- "bot 房间门控 roomHasBot" → Task 2 `isBotRoom`。✓
- "外观 Box+ContainerColor SurfaceVariant + truncate" → Task 2 JSX。✓
- §5.4 插入点 Room.tsx RoomViewHeader 后非 call 分支 → Task 3。✓
- §7 边界（空文案/非 bot 房/call 房/长文案/多标签/挂载竞态）→ Task 2 守卫 + Task 4 清单覆盖。✓
- §8 测试清单 → Task 4。✓
- §9 YAGNI（无 fetch/轮询/多字段/跨会话持久化）→ 实现未引入这些。✓

**2. 占位符扫描：** 无 TBD/TODO/"add error handling" 等。所有代码步骤含完整代码。✓

**3. 类型一致性：**
- `announcement?: string`（Task 1）与 `const { announcement } = useClientConfig()`（Task 2）一致。✓
- `roomHasBot(room: Room, botUserIds: string[])`（continuation.ts:74）与调用 `roomHasBot(room, botUserIds)` 一致；`botUserIds` 由 `useSetting(settingsAtom, 'botUserIds')` 取得（类型 `string[]`）。✓
- `ContainerColor({ variant: 'SurfaceVariant' })`——`ContainerColor` recipe 的 `variant` 接受 `'SurfaceVariant'`（ContainerColor.css.ts:28-44）。✓
- `IconButton` props `variant/size/radii/aria-label/shrink` 与 folds d.ts 一致（`shrink` 来自 Box 的 AsProp 透传？——见下注）。✓
- `Text` `size="T200" truncate` 与 folds d.ts 一致。✓
- `Icons.Cross`、`Icon src size="50"` 与 NewContextHint.tsx 一致。✓
- `RoomAnnouncementBar` props `{ room: Room }`（Task 2）与 `<RoomAnnouncementBar room={room} />`（Task 3）一致；`room` 在 `Room()` 由 `useRoom()` 取得（类型 `Room`）。✓

**注（IconButton 的 `shrink` prop）：** `IconButton` 经 folds `as` 包装透传 Box props，`shrink` 是 Box prop。若 typecheck 报 `shrink` 不在 `IconButton` 上，去掉 `shrink="No"`（非必需，仅为防止按钮被文案挤压）。`<Box shrink="No">` 的 `shrink` 合法。

实现时可按需调整；上述为预期。
