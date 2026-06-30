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
    if (!visible) return undefined;
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
      >
        <Icon src={Icons.Cross} size="50" />
      </IconButton>
    </Box>
  );
}
