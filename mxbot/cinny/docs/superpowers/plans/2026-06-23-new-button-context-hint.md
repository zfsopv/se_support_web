# /new 上下文清理提示框 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** In Cinny bot rooms, show a dismissible hint near the input box's `/new` (clear-context) button when the room has been idle >10min, recommending the user clear bot context before asking a new question; suppress for 10min after the user dismisses it or clicks `/new`.

**Architecture:** A per-user Jotai atom backed by localStorage stores a `Map<roomId, timestamp>` cooldown (mirrors the existing `navToActivePath` pattern). A `useNewContextHint` hook computes the trigger (entry + input focus) using the existing `useRoomLatestRenderedEvent` hook and a new `roomHasBot` helper, and drives a folds `PopOut` (controlled `anchor`). The hint card reuses the proven `PopOut` + `focus-trap-react` dismiss pattern from `RoomVersionSelector`.

**Tech Stack:** React 18, TypeScript, Jotai (atoms), folds (UI primitives), focus-trap-react, slate (editor), matrix-js-sdk (`Room`/`MatrixEvent`). Vanilla-extract for CSS.

**Testing note:** Cinny has **no test runner** (`package.json` has no `test` script, no vitest/jest). Verification is via `yarn typecheck` (tsc `--noEmit`) + `yarn lint` (eslint + prettier) after each task, plus a manual verification checklist at the end. Do NOT add a test framework — that is out of scope. If you want one, stop and ask first.

**Spec:** `docs/superpowers/specs/2026-06-23-new-button-context-hint-design.md`

---

## File Structure

| File | Action | Responsibility |
|---|---|---|
| `src/app/state/newContextHintCooldown.ts` | Create | Per-user Jotai atom factory: `Map<roomId, number>` backed by localStorage; exports `COOLDOWN_MS` + `makeNewContextHintCooldownAtom` + `clearNewContextHintCooldownStore`. Mirrors `navToActivePath.ts`. |
| `src/app/state/hooks/newContextHintCooldown.ts` | Create | React Context + `NewContextHintCooldownProvider` + `useNewContextHintCooldownAtom()` hook. Mirrors `state/hooks/navToActivePath.ts`. |
| `src/app/pages/client/ClientInitStorageAtom.tsx` | Modify | Register the cooldown provider alongside the other per-user storage providers. |
| `src/app/features/room/message/continuation.ts` | Modify | Add `roomHasBot(room, botUserIds)` helper (same `user_type==='bot'` predicate as `isBotSender`, raised to room level). |
| `src/app/features/room/useNewContextHint.ts` | Create | Trigger + popover-state hook. Returns `{ newButtonRef, anchor, checkAndShow, dismiss }`. |
| `src/app/features/room/NewContextHint.tsx` | Create | The `PopOut` content: white `Menu` card with `<b>↓</b>` + message + ✕ close. |
| `src/app/components/editor/Editor.tsx` | Modify | Add optional `onFocus` prop to `CustomEditor`, forwarded to the Slate `<Editable>` (so RoomInput can detect "about to type"). Backward-compatible. |
| `src/app/features/room/RoomInput.tsx` | Modify | Wire the hook: ref on `/new` IconButton, `onFocus` on the editor, render the hint, send `/new` + `dismiss()` on button click. |

---

### Task 1: Cooldown store + hook

**Files:**
- Create: `src/app/state/newContextHintCooldown.ts`
- Create: `src/app/state/hooks/newContextHintCooldown.ts`

- [ ] **Step 1: Create `src/app/state/newContextHintCooldown.ts`**

```ts
import { WritableAtom, atom } from 'jotai';
import produce from 'immer';
import {
  atomWithLocalStorage,
  getLocalStorageItem,
  setLocalStorageItem,
} from './utils/atomWithLocalStorage';

const NEW_CONTEXT_HINT_COOLDOWN = 'newContextHintCooldown';

// 冷却时长（ms）：同时作为冷却判定阈值与过期清理阈值，由 useNewContextHint 复用
export const COOLDOWN_MS = 10 * 60 * 1000;

const getStoreKey = (userId: string): string => `${NEW_CONTEXT_HINT_COOLDOWN}${userId}`;

type NewContextHintCooldown = Map<string, number>;

type NewContextHintCooldownAction = { roomId: string; ts: number };

export type NewContextHintCooldownAtom = WritableAtom<
  NewContextHintCooldown,
  [NewContextHintCooldownAction],
  void
>;

export const makeNewContextHintCooldownAtom = (
  userId: string
): NewContextHintCooldownAtom => {
  const storeKey = getStoreKey(userId);

  const baseCooldownAtom = atomWithLocalStorage<NewContextHintCooldown>(
    storeKey,
    (key) => new Map(getLocalStorageItem<[string, number][]>(key, [])),
    (key, value) => setLocalStorageItem(key, Array.from(value.entries()))
  );

  const cooldownAtom = atom<NewContextHintCooldown, [NewContextHintCooldownAction], void>(
    (get) => get(baseCooldownAtom),
    (get, set, { roomId, ts }) => {
      set(
        baseCooldownAtom,
        produce(get(baseCooldownAtom), (draft) => {
          draft.set(roomId, ts);
          // 清理过期项，控制体积
          const expired: string[] = [];
          draft.forEach((at, id) => {
            if (id !== roomId && ts - at > COOLDOWN_MS) expired.push(id);
          });
          expired.forEach((id) => draft.delete(id));
        })
      );
    }
  );

  return cooldownAtom;
};

export const clearNewContextHintCooldownStore = (userId: string) => {
  localStorage.removeItem(getStoreKey(userId));
};
```

- [ ] **Step 2: Create `src/app/state/hooks/newContextHintCooldown.ts`**

```ts
import { createContext, useContext } from 'react';
import { NewContextHintCooldownAtom } from '../newContextHintCooldown';

const NewContextHintCooldownAtomContext = createContext<
  NewContextHintCooldownAtom | null
>(null);
export const NewContextHintCooldownProvider = NewContextHintCooldownAtomContext.Provider;

export const useNewContextHintCooldownAtom = (): NewContextHintCooldownAtom => {
  const anAtom = useContext(NewContextHintCooldownAtomContext);

  if (!anAtom) {
    throw new Error('NewContextHintCooldownAtom is not provided!');
  }

  return anAtom;
};
```

- [ ] **Step 3: Verify it typechecks and lints**

Run: `yarn typecheck && yarn lint`
Expected: PASS (no errors). If eslint complains about import order, run `yarn fix:prettier` and re-lint.

- [ ] **Step 4: Commit**

```bash
git add src/app/state/newContextHintCooldown.ts src/app/state/hooks/newContextHintCooldown.ts
git commit -m "feat: add newContextHintCooldown per-user store"
```

---

### Task 2: Register the cooldown provider

**Files:**
- Modify: `src/app/pages/client/ClientInitStorageAtom.tsx`

- [ ] **Step 1: Replace the entire file contents with**

```tsx
import React, { ReactNode, useMemo } from 'react';
import { useMatrixClient } from '../../hooks/useMatrixClient';
import { makeClosedNavCategoriesAtom } from '../../state/closedNavCategories';
import { ClosedNavCategoriesProvider } from '../../state/hooks/closedNavCategories';
import { makeClosedLobbyCategoriesAtom } from '../../state/closedLobbyCategories';
import { ClosedLobbyCategoriesProvider } from '../../state/hooks/closedLobbyCategories';
import { makeNavToActivePathAtom } from '../../state/navToActivePath';
import { NavToActivePathProvider } from '../../state/hooks/navToActivePath';
import { makeOpenedSidebarFolderAtom } from '../../state/openedSidebarFolder';
import { OpenedSidebarFolderProvider } from '../../state/hooks/openedSidebarFolder';
import { makeCallPreferencesAtom } from '../../state/callPreferences';
import { CallPreferencesProvider } from '../../state/hooks/callPreferences';
import { makeNewContextHintCooldownAtom } from '../../state/newContextHintCooldown';
import { NewContextHintCooldownProvider } from '../../state/hooks/newContextHintCooldown';

type ClientInitStorageAtomProps = {
  children: ReactNode;
};
export function ClientInitStorageAtom({ children }: ClientInitStorageAtomProps) {
  const mx = useMatrixClient();
  const userId = mx.getUserId()!;

  const closedNavCategoriesAtom = useMemo(() => makeClosedNavCategoriesAtom(userId), [userId]);

  const closedLobbyCategoriesAtom = useMemo(() => makeClosedLobbyCategoriesAtom(userId), [userId]);

  const navToActivePathAtom = useMemo(() => makeNavToActivePathAtom(userId), [userId]);

  const openedSidebarFolderAtom = useMemo(() => makeOpenedSidebarFolderAtom(userId), [userId]);

  const callPreferencesAtom = useMemo(() => makeCallPreferencesAtom(userId), [userId]);

  const newContextHintCooldownAtom = useMemo(
    () => makeNewContextHintCooldownAtom(userId),
    [userId]
  );

  return (
    <ClosedNavCategoriesProvider value={closedNavCategoriesAtom}>
      <ClosedLobbyCategoriesProvider value={closedLobbyCategoriesAtom}>
        <NavToActivePathProvider value={navToActivePathAtom}>
          <OpenedSidebarFolderProvider value={openedSidebarFolderAtom}>
            <CallPreferencesProvider value={callPreferencesAtom}>
              <NewContextHintCooldownProvider value={newContextHintCooldownAtom}>
                {children}
              </NewContextHintCooldownProvider>
            </CallPreferencesProvider>
          </OpenedSidebarFolderProvider>
        </NavToActivePathProvider>
      </ClosedLobbyCategoriesProvider>
    </ClosedNavCategoriesProvider>
  );
}
```

- [ ] **Step 2: Verify it typechecks and lints**

Run: `yarn typecheck && yarn lint`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add src/app/pages/client/ClientInitStorageAtom.tsx
git commit -m "feat: register NewContextHintCooldown provider"
```

---

### Task 3: `roomHasBot` helper

**Files:**
- Modify: `src/app/features/room/message/continuation.ts` (add `Room` to the matrix-js-sdk import on line 1; append the new function at end of file)

- [ ] **Step 1: Update the matrix-js-sdk import**

Change line 1 from:
```ts
import { MatrixEvent } from 'matrix-js-sdk';
```
to:
```ts
import { MatrixEvent, Room } from 'matrix-js-sdk';
```

- [ ] **Step 2: Append `roomHasBot` at the end of `continuation.ts`**

```ts
/**
 * Room-level bot detection: a room "has a bot" if any currently-joined member is
 * in the configured botUserIds whitelist, or has m.room.member content
 * user_type === 'bot'. Mirrors isBotSender's per-event predicate.
 */
export function roomHasBot(room: Room, botUserIds: string[]): boolean {
  if (botUserIds.some((id) => room.getMember(id)?.membership === 'join')) return true;
  return room
    .getMembers()
    .some(
      (m) =>
        m.membership === 'join' && m.events?.member?.getContent()?.user_type === 'bot'
    );
}
```

- [ ] **Step 3: Verify it typechecks and lints**

Run: `yarn typecheck && yarn lint`
Expected: PASS. If eslint flags unused `Room` (it won't — it's used in `roomHasBot`), re-check the function is present.

- [ ] **Step 4: Commit**

```bash
git add src/app/features/room/message/continuation.ts
git commit -m "feat: add roomHasBot helper"
```

---

### Task 4: `useNewContextHint` hook

**Files:**
- Create: `src/app/features/room/useNewContextHint.ts`

- [ ] **Step 1: Create `src/app/features/room/useNewContextHint.ts`**

```ts
import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { useAtom } from 'jotai';
import { Room } from 'matrix-js-sdk';
import { RectCords } from 'folds';
import { useSetting } from '../../state/hooks/settings';
import { settingsAtom } from '../../state/settings';
import { useNewContextHintCooldownAtom } from '../../state/hooks/newContextHintCooldown';
import { COOLDOWN_MS } from '../../state/newContextHintCooldown';
import { useRoomLatestRenderedEvent } from '../../hooks/useRoomLatestRenderedEvent';
import { roomHasBot } from './message/continuation';

const TRIGGER_GAP_MS = 10 * 60 * 1000;

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

  // 进房间（挂载）+ 依赖变化时检测
  useEffect(() => {
    checkAndShow();
  }, [checkAndShow]);

  const dismiss = useCallback(() => {
    setCooldown({ roomId, ts: Date.now() });
    setAnchor(undefined);
  }, [roomId, setCooldown]);

  return { newButtonRef, anchor, checkAndShow, dismiss };
}
```

- [ ] **Step 2: Verify it typechecks and lints**

Run: `yarn typecheck && yarn lint`
Expected: PASS. (Requires Tasks 1–3 done.)

- [ ] **Step 3: Commit**

```bash
git add src/app/features/room/useNewContextHint.ts
git commit -m "feat: add useNewContextHint trigger hook"
```

---

### Task 5: `NewContextHint` component

**Files:**
- Create: `src/app/features/room/NewContextHint.tsx`

- [ ] **Step 1: Create `src/app/features/room/NewContextHint.tsx`**

```tsx
import { Box, Icon, IconButton, Icons, Menu, PopOut, RectCords, Text, config, toRem } from 'folds';
import FocusTrap from 'focus-trap-react';

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
      offset={4}
      position="Top"
      align="Start"
      content={
        <FocusTrap
          focusTrapOptions={{
            initialFocus: false,
            onDeactivate: onDismiss,
            clickOutsideDeactivates: true,
            escapeDeactivates: () => true,
          }}
        >
          <Menu>
            <Box
              direction="Row"
              alignItems="Center"
              gap="200"
              style={{
                padding: `${config.space.S200} ${config.space.S300}`,
                maxWidth: toRem(360),
              }}
            >
              <Text size="T200">
                <b>↓</b> 询问新的问题时，推荐点击该按钮清理上下文
              </Text>
              <IconButton
                onClick={onDismiss}
                variant="SurfaceVariant"
                size="300"
                radii="300"
                aria-label="关闭提示"
              >
                <Icon src={Icons.Cross} size="50" />
              </IconButton>
            </Box>
          </Menu>
        </FocusTrap>
      }
    />
  );
}
```

- [ ] **Step 2: Verify it typechecks and lints**

Run: `yarn typecheck && yarn lint`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add src/app/features/room/NewContextHint.tsx
git commit -m "feat: add NewContextHint popover component"
```

---

### Task 6: Add `onFocus` to `CustomEditor`

**Files:**
- Modify: `src/app/components/editor/Editor.tsx` (import line 2-9, `CustomEditorProps` type ~line 61-74, destructure ~line 77-91, `<Editable>` ~line 139-149)

- [ ] **Step 1: Add `FocusEventHandler` to the React import**

Change lines 2-9 from:
```tsx
import React, {
  ClipboardEventHandler,
  KeyboardEventHandler,
  ReactNode,
  forwardRef,
  useCallback,
  useState,
} from 'react';
```
to:
```tsx
import React, {
  ClipboardEventHandler,
  FocusEventHandler,
  KeyboardEventHandler,
  ReactNode,
  forwardRef,
  useCallback,
  useState,
} from 'react';
```

- [ ] **Step 2: Add `onFocus` to `CustomEditorProps`**

In the `CustomEditorProps` type (around line 61-74), add `onFocus` next to the other handlers. Change:
```tsx
type CustomEditorProps = {
  editableName?: string;
  top?: ReactNode;
  bottom?: ReactNode;
  before?: ReactNode;
  after?: ReactNode;
  maxHeight?: string;
  editor: Editor;
  placeholder?: string;
  onKeyDown?: KeyboardEventHandler;
  onKeyUp?: KeyboardEventHandler;
  onChange?: EditorChangeHandler;
  onPaste?: ClipboardEventHandler;
};
```
to:
```tsx
type CustomEditorProps = {
  editableName?: string;
  top?: ReactNode;
  bottom?: ReactNode;
  before?: ReactNode;
  after?: ReactNode;
  maxHeight?: string;
  editor: Editor;
  placeholder?: string;
  onKeyDown?: KeyboardEventHandler;
  onKeyUp?: KeyboardEventHandler;
  onChange?: EditorChangeHandler;
  onPaste?: ClipboardEventHandler;
  onFocus?: FocusEventHandler;
};
```

- [ ] **Step 3: Destructure `onFocus` in the component body**

Change the destructure (around line 77-91) from:
```tsx
  (
    {
      editableName,
      top,
      bottom,
      before,
      after,
      maxHeight = '50vh',
      editor,
      placeholder,
      onKeyDown,
      onKeyUp,
      onChange,
      onPaste,
    },
    ref
  ) => {
```
to:
```tsx
  (
    {
      editableName,
      top,
      bottom,
      before,
      after,
      maxHeight = '50vh',
      editor,
      placeholder,
      onKeyDown,
      onKeyUp,
      onChange,
      onPaste,
      onFocus,
    },
    ref
  ) => {
```

- [ ] **Step 4: Forward `onFocus` to the Slate `<Editable>`**

In the `<Editable>` element (around line 139-149), add `onFocus={onFocus}`. Change:
```tsx
              <Editable
                data-editable-name={editableName}
                className={css.EditorTextarea}
                placeholder={placeholder}
                renderPlaceholder={renderPlaceholder}
                renderElement={renderElement}
                renderLeaf={renderLeaf}
                onKeyDown={handleKeydown}
                onKeyUp={onKeyUp}
                onPaste={onPaste}
              />
```
to:
```tsx
              <Editable
                data-editable-name={editableName}
                className={css.EditorTextarea}
                placeholder={placeholder}
                renderPlaceholder={renderPlaceholder}
                renderElement={renderElement}
                renderLeaf={renderLeaf}
                onKeyDown={handleKeydown}
                onKeyUp={onKeyUp}
                onPaste={onPaste}
                onFocus={onFocus}
              />
```

- [ ] **Step 5: Verify it typechecks and lints**

Run: `yarn typecheck && yarn lint`
Expected: PASS. (`MessageEditor` and other `CustomEditor` consumers don't pass `onFocus`, so it stays `undefined` → no behavior change for them.)

- [ ] **Step 6: Commit**

```bash
git add src/app/components/editor/Editor.tsx
git commit -m "feat: add optional onFocus prop to CustomEditor"
```

---

### Task 7: Integrate into `RoomInput`

**Files:**
- Modify: `src/app/features/room/RoomInput.tsx`

- [ ] **Step 1: Add imports for the hook and component**

After the existing import of `useRoomCreatorsTag` (line 120) — i.e. near the other `../../hooks` / feature imports — add:
```tsx
import { useNewContextHint } from './useNewContextHint';
import { NewContextHint } from './NewContextHint';
```

(Place these two lines after the `useComposingCheck` import at line 122, before `interface RoomInputProps`.)

- [ ] **Step 2: Call the hook inside `RoomInput`**

Inside the component body, after `const creators = useRoomCreators(room);` (line 142), add:
```tsx
    const hint = useNewContextHint(room);
```

- [ ] **Step 3: Wire `onFocus` on the `CustomEditor`**

On the `<CustomEditor>` element (around line 510-516), add `onFocus={hint.checkAndShow}`. Change:
```tsx
        <CustomEditor
          editableName="RoomInput"
          editor={editor}
          placeholder="在此输入您的问题(Shift+Enter可以换行)"
          onKeyDown={handleKeyDown}
          onKeyUp={handleKeyUp}
          onPaste={handlePaste}
```
to:
```tsx
        <CustomEditor
          editableName="RoomInput"
          editor={editor}
          placeholder="在此输入您的问题(Shift+Enter可以换行)"
          onKeyDown={handleKeyDown}
          onKeyUp={handleKeyUp}
          onPaste={handlePaste}
          onFocus={hint.checkAndShow}
```

- [ ] **Step 4: Attach ref + dismiss on the `/new` button**

Change the `before` prop (around line 556-565) from:
```tsx
          before={
            <IconButton
              onClick={() => mx.sendTextMessage(roomId, '/new')}
              variant="SurfaceVariant"
              size="300"
              radii="300"
            >
              <Icon src={Icons.Delete} />
            </IconButton>
          }
```
to:
```tsx
          before={
            <IconButton
              ref={hint.newButtonRef}
              onClick={() => {
                mx.sendTextMessage(roomId, '/new');
                hint.dismiss();
              }}
              variant="SurfaceVariant"
              size="300"
              radii="300"
            >
              <Icon src={Icons.Delete} />
            </IconButton>
          }
```

- [ ] **Step 5: Render the hint popover**

The `RoomInput` returns a fragment inside `<div ref={ref}> ... </div>`. After the closing `/>` of `<CustomEditor ... />` (line 572) and before the closing `</div>` (line 573), add the hint:
```tsx
        <NewContextHint anchor={hint.anchor} onDismiss={hint.dismiss} />
```

So the tail of the JSX becomes:
```tsx
          bottom={undefined}
        />
        <NewContextHint anchor={hint.anchor} onDismiss={hint.dismiss} />
      </div>
    );
  }
);
```

- [ ] **Step 6: Verify it typechecks and lints**

Run: `yarn typecheck && yarn lint`
Expected: PASS. Run `yarn fix:prettier` if prettier flags formatting, then re-lint.

- [ ] **Step 7: Commit**

```bash
git add src/app/features/room/RoomInput.tsx
git commit -m "feat: show context-clear hint near /new button on idle bot rooms"
```

---

### Task 8: Manual verification

**Files:** none (runtime verification)

- [ ] **Step 1: Start the dev server**

Run: `yarn start`
Expected: Vite dev server starts; open the printed URL.

- [ ] **Step 2: Verify the happy path**

1. Open a bot room whose last message is older than 10 minutes.
2. Confirm the hint appears above the `/new` (🗑) button, left-aligned, with `↓ 询问新的问题时，推荐点击该按钮清理上下文` and a ✕.
3. Click ✕ → the hint disappears.

- [ ] **Step 3: Verify the cooldown**

1. Within 10 minutes, re-enter the room and focus the input → hint must NOT reappear.
2. Refresh the page within 10 minutes → hint must NOT reappear (cooldown persisted to localStorage).

- [ ] **Step 4: Verify re-trigger after cooldown**

1. Wait >10 minutes (or temporarily lower `COOLDOWN_MS`/`TRIGGER_GAP_MS` in code for testing), then focus the input → hint reappears.

- [ ] **Step 5: Verify `/new` click path**

1. With the hint visible, click the `/new` (🗑) button → a `/new` message is sent AND the hint disappears; within 10 minutes the hint does not reappear.

- [ ] **Step 6: Verify scope (non-bot rooms)**

1. Open a room with no bot member → the hint must NOT appear. The `/new` button still sends `/new` normally.

- [ ] **Step 7: Verify empty room**

1. Open a bot room with no messages → the hint must NOT appear (`lastTs` undefined).

- [ ] **Step 8: Final commit (if any test-helper tweaks were reverted)**

If you changed `COOLDOWN_MS`/`TRIGGER_GAP_MS` for testing, revert them. Run `yarn typecheck && yarn lint`. If clean, no commit needed (nothing changed). If you reverted test tweaks, commit:
```bash
git commit -am "chore: revert test-only threshold tweaks"
```

---

## Self-Review

(Completed by the plan author after writing — see notes below. No subagent.)

**1. Spec coverage:**
- Trigger on entry + input focus → Task 4 (`useEffect` mount = entry; `checkAndShow` on `onFocus` via Task 6/7). ✓
- Scope = bot rooms only → Task 3 `roomHasBot` + Task 4 `isBotRoom`. ✓
- Baseline = last message → Task 4 `useRoomLatestRenderedEvent(room)?.getTs()`. ✓
- 10-min trigger threshold + 10-min cooldown → Task 1 `COOLDOWN_MS` + Task 4 `TRIGGER_GAP_MS`. ✓
- Persist per-user, `Map<roomId, ts>`, localStorage, cross-tab → Task 1 (`atomWithLocalStorage` listens to `storage` events). ✓
- PopOut controlled anchor, Top/Start → Task 5. ✓
- White card, `↓` + text + ✕ → Task 5 (`Menu` panel + `<b>↓</b>` + `Icons.Cross`). ✓
- Dismiss on ✕ / outside-click / `/new` click → Task 5 FocusTrap (`onDeactivate`+`clickOutsideDeactivates`+`escapeDeactivates`) + Task 7 `/new` `onClick` → `hint.dismiss()`. ✓
- Lifecycle: stays until dismissed (no auto-hide) → Task 4 `anchor` persists across re-renders; only `dismiss` clears it. ✓
- Edge cases (empty room, ref not ready, non-bot room, send failure, multi-tab, clock, permalink) → Task 4 guards (`lastTs !== undefined`, `newButtonRef.current` null check, `isBotRoom`); documented in spec §7. ✓
- Testing = manual checklist (no runner) → Task 8. ✓

**2. Placeholder scan:** No TBD/TODO/"handle edge cases"/"similar to Task N". Every code step has full code. ✓

**3. Type consistency:**
- `useNewContextHint` returns `{ newButtonRef, anchor, checkAndShow, dismiss }` — Task 7 uses exactly these (`hint.newButtonRef`, `hint.checkAndShow`, `hint.dismiss`, `hint.anchor`). ✓
- `setCooldown({ roomId, ts })` (Task 1 write signature `[NewContextHintCooldownAction]`) matches `dismiss` call in Task 4 (`setCooldown({ roomId, ts: Date.now() })`). ✓
- `NewContextHint` props `{ anchor, onDismiss }` match Task 7 render `<NewContextHint anchor={hint.anchor} onDismiss={hint.dismiss} />`. ✓
- `COOLDOWN_MS` exported from Task 1, imported in Task 4. ✓
- `roomHasBot(room, botUserIds)` signature (Task 3) matches Task 4 call. ✓
- `Icons.Cross`, `IconButton` ref, `Menu`/`Box`/`Text`/`PopOut`/`RectCords`/`config`/`toRem` all verified against folds dist `.d.ts`. ✓
- `CustomEditor` `onFocus?: FocusEventHandler` (Task 6) → `onFocus={hint.checkAndShow}` (Task 7): `checkAndShow: () => void` is assignable to `FocusEventHandler` (extra args ignored). ✓

No issues found.
