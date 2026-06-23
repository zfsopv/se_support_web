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
