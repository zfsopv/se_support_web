# /new 上下文清理提示框 — 设计文档

- 日期: 2026-06-23
- 状态: 已批准（待 spec 评审）
- 关联代码: `src/app/features/room/RoomInput.tsx`

## 1. 目标

在 sex-support bot 房间里，当用户回到一个已闲置 10 分钟以上的会话、准备问新问题时，在输入框左侧的 `/new`（清理上下文）按钮附近弹出一个可关闭的小提示框，建议先点击该按钮清理 bot 上下文。用户关闭提示框、或点击 `/new` 按钮清理上下文后，10 分钟内不再弹出。

## 2. 代码现状（已核实）

- `/new` 按钮：`RoomInput.tsx:556-565`，folds `IconButton` + `Icons.Delete`，位于 `CustomEditor` 的 `before` 槽，`onClick={() => mx.sendTextMessage(roomId, '/new')}`。当前无 ref、无 tooltip。
- 最后一条消息时间戳：现成 hook `useRoomLatestRenderedEvent(room)`（`src/app/hooks/useRoomLatestRenderedEvent.ts`）。签名仅 `(room)`，内部读取设置（`hideMembershipEvents` / `hideNickAvatarEvents` / `showHiddenEvents`），返回最新可见事件，并随 `RoomEvent.Timeline` 更新。`.getTs()` → ms。
- bot 判定：`continuation.ts` 的 `isBotSender(userId, botUserIds, mxEvent)` = `botUserIds.includes(userId)` ∨ `sender.events.member.getContent().user_type === 'bot'`。
- 弹层原语：folds `PopOut`（`anchor: RectCords | undefined`，`position/align/offset/content`，受控显示，Portal + 自动翻转）。folds 无箭头、无 toast/coachmark。现有用法见 `RoomVersionSelector.tsx` 等。
- 持久化范式：`navToActivePath.ts` 用 per-user 工厂 + `atomWithLocalStorage`，存 `Map<string, Path>`（room-keyed），在 `ClientInitStorageAtom.tsx` 注册 Context，hook 位于 `state/hooks/`。
- 编辑器：`Editor.tsx` 的 `CustomEditor`（`forwardRef<HTMLDivElement>`）暴露 `before/after/onKeyDown/onKeyUp/onChange/onPaste`，**未暴露 onFocus**。Slate `<Editable>` 在其内部。
- 测试：`package.json` 无测试运行器（无 vitest/jest、无 `test` 脚本）。自动化检查仅有 `typecheck`（tsc `--noEmit`）、`lint`（eslint/prettier）。

## 3. 决策（已与用户确认）

| 项 | 决策 |
|---|---|
| 触发时机 | 进房间（RoomInput 挂载）+ 聚焦输入框 两个时刻检测；冷却防重复 |
| 范围 | 仅含 bot 的房间：`roomHasBot` = `botUserIds` 任一成员 ∨ 任一 joined 成员 `user_type==='bot'` |
| 基线时间戳 | `useRoomLatestRenderedEvent(room)?.getTs()`（最后一条可见消息） |
| 触发阈值 | 距基线 > `TRIGGER_GAP_MS`（10 min） |
| 冷却时长 | `COOLDOWN_MS`（10 min）；✕ / 点框外 / 点 `/new` 任一动作后写入 |
| 持久化 | per-user Jotai atom + localStorage，`Map<roomId, 时间戳>`；跨标签经 `storage` 事件同步 |
| 弹层实现 | folds `PopOut`（受控 `anchor`），位置 `Top` / `Start` |
| 外观 | 纯白卡片（对齐 folds `Surface`/`Tooltip`），开头 ↓（强调色）+ 文案 + ✕；无自绘箭头 |
| 文案 | 「询问新的问题时，推荐点击该按钮清理上下文」 |
| 生命周期 | 显示后保持，直到用户主动关闭；不因新消息/失焦自动隐藏 |
| 测试 | 手动验证清单（项目无测试运行器） |

## 4. 架构

新增文件 + RoomInput 集成：

```
src/app/state/newContextHintCooldown.ts         # 工厂 + atom（Map<roomId, number>）
src/app/state/hooks/newContextHintCooldown.ts   # Context + useNewContextHintCooldownAtom()
src/app/features/room/useNewContextHint.ts      # 触发 + 弹层状态 hook
src/app/features/room/NewContextHint.tsx        # PopOut 内容组件
src/app/features/room/message/continuation.ts   # 新增 roomHasBot（见 5.4，与 isBotSender 同处）
```

集成点 `src/app/features/room/RoomInput.tsx`：
- `const hint = useNewContextHint(room)`
- `/new` `IconButton` 挂 `ref={hint.newButtonRef}`，`onClick` 改为发送 `/new` 后调 `hint.dismiss()`
- 编辑器聚焦 → `hint.checkAndShow()`
- 渲染 `<NewContextHint anchor={hint.anchor} onDismiss={hint.dismiss} />`

注册点 `src/app/pages/client/ClientInitStorageAtom.tsx`：按 `navToActivePath` 方式包一层 provider。

## 5. 组件详述

### 5.1 冷却存储 `newContextHintCooldown.ts`

镜像 `navToActivePath.ts`：

```ts
// 冷却时长（ms）：同时作为冷却判定与过期清理阈值，由 useNewContextHint 复用
export const COOLDOWN_MS = 10 * 60 * 1000;
const COOLDOWN_KEY = 'newContextHintCooldown'; // 最终 localStorage key = COOLDOWN_KEY + userId

export const makeNewContextHintCooldownAtom = (userId: string) => {
  const storeKey = `${COOLDOWN_KEY}${userId}`;
  const base = atomWithLocalStorage<Map<string, number>>(
    storeKey,
    (key) => new Map(getLocalStorageItem<[string, number][]>(key, [])),
    (key, value) => setLocalStorageItem(key, Array.from(value.entries())),
  );
  // 派生 atom：暴露 markCooldown(roomId, ts)
  const cooldownAtom = atom(
    (get) => get(base),
    (get, set, roomId: string, ts = Date.now()) => {
      const next = new Map(get(base));
      next.set(roomId, ts);
      for (const [id, at] of next) if (ts - at > COOLDOWN_MS) next.delete(id); // 清理过期项
      set(base, next);
    },
  );
  return cooldownAtom;
};
```

- 值：`Map<roomId, number>`（roomId → 关闭时间戳 ms）
- `markCooldown(roomId)`：写 `Date.now()`，清理超过 `COOLDOWN_MS` 的旧项以控制体积
- `atomWithLocalStorage` 已监听 `storage` 事件 → 多标签一致

### 5.2 hook `state/hooks/newContextHintCooldown.ts`

镜像 `state/hooks/navToActivePath.ts`：React Context + provider + `useNewContextHintCooldownAtom()`。在 `ClientInitStorageAtom.tsx` 用 `useMemo(() => makeNewContextHintCooldownAtom(userId), [userId])` 创建并包裹。

### 5.3 触发 hook `useNewContextHint.ts`

```ts
const TRIGGER_GAP_MS = 10 * 60 * 1000; // 触发阈值（距最后一条消息）
// COOLDOWN_MS 从 newContextHintCooldown.ts 导入复用

export function useNewContextHint(room: Room) {
  const { roomId } = room;
  const [botUserIds] = useSetting(settingsAtom, 'botUserIds');
  const latestEvent = useRoomLatestRenderedEvent(room);
  const cooldownAtom = useNewContextHintCooldownAtom();
  const [cooldown, setCooldown] = useAtom(cooldownAtom);
  const newButtonRef = useRef<HTMLButtonElement>(null);
  const [anchor, setAnchor] = useState<RectCords>();

  const isBotRoom = useMemo(() => roomHasBot(room, botUserIds), [room, botUserIds]);
  const lastTs = latestEvent?.getTs();
  const lastInteraction = cooldown.get(roomId);

  const checkAndShow = useCallback(() => {
    const now = Date.now();
    const shouldShow =
      isBotRoom &&
      lastTs !== undefined &&
      now - lastTs > TRIGGER_GAP_MS &&
      (!lastInteraction || now - lastInteraction > COOLDOWN_MS);
    if (shouldShow && !anchor && newButtonRef.current) {
      setAnchor(newButtonRef.current.getBoundingClientRect());
    }
  }, [isBotRoom, lastTs, lastInteraction, anchor]);

  // 进房间：挂载及依赖变化时检测
  useEffect(() => { checkAndShow(); }, [checkAndShow]);

  const dismiss = useCallback(() => {
    setCooldown(roomId, Date.now()); // markCooldown + 清理
    setAnchor(undefined);
  }, [roomId, setCooldown]);

  return { newButtonRef, anchor, checkAndShow, dismiss };
}
```

说明：
- `checkAndShow` 由 RoomInput 在编辑器 `onFocus` 时调用（第二个触发点）。
- `!anchor` 守卫避免重复弹。
- `dismiss` 同时用于 ✕ / 点框外 / 点 `/new`（三者都记冷却 + 隐藏）。
- `useEffect([checkAndShow])`：挂载即检测（进房间）；`lastTs`/`lastInteraction`/`isBotRoom` 变化时也重算（覆盖多标签 cooldown 同步等场景）。冷却内 `shouldShow` 为假，不会重弹。

### 5.4 `roomHasBot`（置于 `continuation.ts`）

复用 `isBotSender` 的 user_type 口径，提升为房间级：

```ts
export function roomHasBot(room: Room, botUserIds: string[]): boolean {
  if (botUserIds.some((id) => room.getMember(id))) return true; // 白名单成员
  return room
    .getMembers()
    .some((m) => m.membership === 'join' && m.events?.member?.getContent()?.user_type === 'bot');
}
```

置于 `continuation.ts`（与 `isBotSender` 同处）便于复用与一致性。

### 5.5 UI `NewContextHint.tsx`

```tsx
export function NewContextHint({
  anchor,
  onDismiss,
}: {
  anchor: RectCords | undefined;
  onDismiss: () => void;
}) {
  return (
    <PopOut
      anchor={anchor}
      position="Top"
      align="Start"
      offset={4}
      content={
        // 点框外关闭经 FocusTrap onDeactivate → onDismiss（确切接线见 §8）
        <Surface> {/* 纯白卡片，对齐 folds Tooltip/Surface */}
          <Box direction="Row" alignItems="Center" gap="200">
            <Text size="T200">
              <span style={{ color: 'var(--accent)', fontWeight: 700 }}>↓</span>
              {' 询问新的问题时，推荐点击该按钮清理上下文'}
            </Text>
            <IconButton
              size="200"
              radii="300"
              variant="SurfaceVariant"
              onClick={onDismiss}
              aria-label="关闭"
            >
              <Icon src={Icons.Cross} />
            </IconButton>
          </Box>
        </Surface>
      }
    />
  );
}
```

- `anchor` 为 `undefined` 时 `PopOut` 不渲染。
- ✕ 与点框外（FocusTrap `onDeactivate`）均调 `onDismiss`。
- 纯白卡片用 folds `Surface`/`Tooltip` 风格；↓ 用强调色字符。
- 位置 `Top`/`Start`：folds `getRelativeFixedPosition` 自动翻转兜底（输入框在底部，上方有空间）。

### 5.6 RoomInput 集成（`RoomInput.tsx`）

- 顶部：`const hint = useNewContextHint(room);`
- `/new` 按钮（556-565）：
  ```tsx
  <IconButton
    ref={hint.newButtonRef}
    variant="SurfaceVariant"
    size="300"
    radii="300"
    onClick={() => {
      mx.sendTextMessage(roomId, '/new');
      hint.dismiss();
    }}
  >
    <Icon src={Icons.Delete} />
  </IconButton>
  ```
- 编辑器聚焦：给 `CustomEditor` 外层包一层 `onFocus`（React `onFocus` 经 `focusin` 冒泡，可捕获内部 `<Editable>` 聚焦）→ `hint.checkAndShow()`。或在 `CustomEditor` 加可选 `onFocus` prop 转发到 Slate `<Editable onFocus>`（见 §8 待确认）。
- 在 `RoomInput` 返回 JSX 末尾渲染 `<NewContextHint anchor={hint.anchor} onDismiss={hint.dismiss} />`。

## 6. 数据流

1. 进 bot 房间 → `RoomInput` 挂载 → `useEffect` → `checkAndShow()` → 谓词真 → 取按钮 rect → `setAnchor` → `PopOut` 渲染。
2. 聚焦输入框 → `onFocus` → `checkAndShow()` → 同上（冷却内跳过）。
3. 点 ✕ / 点框外 → `onDismiss` → `markCooldown(roomId, now)` + `setAnchor(undefined)` → 写 localStorage。
4. 点 `/new` → `sendTextMessage('/new')` + `hint.dismiss()`（冷却 + 隐藏）。
5. 10 分钟内再次进房/聚焦 → `now - lastInteraction < COOLDOWN` → `shouldShow=false` → 跳过。
6. 满 10 分钟 + 聚焦/进房 → `shouldShow=true`（若 gap 仍 > 阈值）→ 再弹。

## 7. 边界与错误处理

- **空房间**（无最后消息）：`lastTs` 为 `undefined` → `shouldShow=false`。
- **按钮 ref 未就绪**（挂载竞态）：`newButtonRef.current` 为 null → `checkAndShow` 跳过本帧；聚焦时重试。
- **非 bot 房间**：`isBotRoom=false` → 不显示；`/new` 按钮仍正常发 `/new`，不会进入 `dismiss`（非 bot 房不显示，故不记冷却）。
- **/new 发送失败**（网络）：仍记冷却（按用户意图；最坏 10 分钟内不再提示，可接受）。
- **多标签**：`storage` 事件同步冷却；某标签关闭提示后，其它标签 `lastInteraction` 更新 → `checkAndShow` 重算跳过。
- **时钟**：`getTs()` 与 `Date.now()` 均为 ms。
- **同房间 permalink 跳转**重挂：重跑 `checkAndShow`，冷却/谓词把关，无害。

## 8. 测试

项目无测试运行器（`package.json` 无 `test` 脚本、无 vitest/jest）。采用手动验证清单 + 现有自动化检查：

**自动化**：`yarn typecheck`（tsc `--noEmit`）、`yarn lint`（eslint + prettier）须通过。

**手动验证清单**：
1. 进一个上次消息 >10 分钟前的 bot 房间 → 提示框在 `/new` 按钮上方弹出。
2. 点 ✕ → 框消失；10 分钟内重进/聚焦输入框 → 不再弹。
3. 等 >10 分钟后聚焦输入框 → 再次弹出。
4. 点 `/new` 按钮 → 发送 `/new`，框消失；10 分钟内聚焦 → 不弹。
5. 10 分钟内刷新页面 → 不弹（冷却持久化生效）。
6. 进无 bot 房间 → 不弹；点 `/new` 仍正常发送。
7. 多标签：A 标签关闭提示，B 标签 10 分钟内聚焦 → 不弹。

## 9. 范围外（YAGNI）

- 不做引导式 onboarding / 多步 tour。
- 不做 toast / snackbar 体系。
- 不做服务端（Matrix account data）存储——localStorage 足够，跨设备同步非必需。
- 不做「冷却中」倒计时 UI、不做「不再提示」永久关闭开关（仅 10 分钟冷却）。
- 不因新消息到达 / 失焦自动隐藏提示框（保持可预测）。
- 不做触发阈值的用户可配置项（常量写死 10 min；两个常量 `TRIGGER_GAP_MS` / `COOLDOWN_MS` 可独立调）。

## 10. 待 planning 确认的实现细节

1. folds `PopOut` + `FocusTrap` 的点框外关闭确切接线（`onDeactivate` / `clickOutsideDeactivates`），读 `node_modules/folds/dist` 源码确认。
2. 编辑器聚焦：优先用外层 `onFocus`（冒泡）避免改 `CustomEditor`；若需更精确，给 `CustomEditor` 加可选 `onFocus` prop 转发到 Slate `<Editable onFocus>`。
3. folds `IconButton` 是否 forwardRef（用于 `newButtonRef`）；若否，用外层包裹元素取 rect。
4. `Icons.Cross`（或等价关闭图标）是否存在。
5. 纯白卡片用 folds `Surface` 还是 `Tooltip` 样式更贴切（读 folds 组件）。
6. `room.getMembers()` 在大房间的性能（已 memo + 白名单短路；必要时仅取 joined 成员）。
7. `RoomInput` 是否在切换 room 时重挂载（依赖 `RoomProvider key={roomId}`）；若不重挂，entry 检测改为按 `roomId` 变化的 effect。
