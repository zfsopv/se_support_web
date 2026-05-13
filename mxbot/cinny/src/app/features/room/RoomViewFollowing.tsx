import React, { useMemo } from 'react';
import { Box, Text, as } from 'folds';
import { Room } from 'matrix-js-sdk';
import classNames from 'classnames';
import * as css from './RoomViewFollowing.css';
import { useMatrixClient } from '../../hooks/useMatrixClient';
import { useRoomLatestRenderedEvent } from '../../hooks/useRoomLatestRenderedEvent';

type TokenUsageSummary = {
  total?: string;
};

const SUPPORT_BOT_USER_ID_REG = /^@.+support-bot:[^:]+$/;
const TOKEN_USAGE_MESSAGE_REG =
  /(?:\S+\s+)?Conversation Token usage\s*\(ID:\s*([^)]*)\)\s*Total:\s*([\d,]+)\s*Input\s*\(cached\):\s*([\d,]+)\s*Input\s*\(other\):\s*([\d,]+)\s*Output:\s*([\d,]+)/i;
const EMPTY_STATS_MESSAGE_REG = /(?:\S+\s+)?No stats available for this conversation yet\./i;

const normalizeTokenUsageBody = (body: string): string =>
  body.replace(/\u00a0/g, ' ').replace(/\s+/g, ' ').trim();

const isSupportAssistantRoom = (room: Room, userId?: string | null): boolean =>
  room
    .getJoinedMembers()
    .some((member) => member.userId !== userId && SUPPORT_BOT_USER_ID_REG.test(member.userId));

const parseTokenUsage = (body: string): TokenUsageSummary | undefined => {
  const normalizedBody = normalizeTokenUsageBody(body);
  if (EMPTY_STATS_MESSAGE_REG.test(normalizedBody)) {
    return {
      total: undefined,
    };
  }

  const matched = normalizedBody.match(TOKEN_USAGE_MESSAGE_REG);
  if (!matched?.[2]) return undefined;

  return {
    total: matched[2].replace(/,/g, ''),
  };
};

const getLatestTokenUsage = (room: Room, userId?: string | null): TokenUsageSummary | undefined => {
  if (!isSupportAssistantRoom(room, userId)) return undefined;

  const liveEvents = room.getLiveTimeline().getEvents();
  for (let index = liveEvents.length - 1; index >= 0; index -= 1) {
    const event = liveEvents[index];
    const senderId = event?.getSender();
    if (!event || !senderId || senderId === userId || !SUPPORT_BOT_USER_ID_REG.test(senderId)) continue;

    const content = event.getContent() as { body?: string };
    if (typeof content.body !== 'string') continue;

    const usage = parseTokenUsage(content.body);
    if (usage) return usage;
  }

  return undefined;
};

const formatTokenTotal = (total?: string): string => {
  if (!total) return '--';

  const totalNumber = Number(total);
  if (Number.isNaN(totalNumber)) return '--';

  const totalInK = totalNumber / 1000;
  if (totalInK >= 100) return totalInK.toFixed(0);
  if (totalInK >= 10) return totalInK.toFixed(1);
  return totalInK.toFixed(2).replace(/\.0+$|0+$/g, '');
};

const getTokenUsageValue = (usage?: TokenUsageSummary): string =>
  `${formatTokenTotal(usage?.total)} K`;

export function RoomViewFollowingPlaceholder() {
  return <div className={css.RoomViewFollowingPlaceholder} />;
}

export type RoomViewFollowingProps = {
  room: Room;
};
export const RoomViewFollowing = as<'div', RoomViewFollowingProps>(
  ({ className, room, ...props }, ref) => {
    const mx = useMatrixClient();
    const latestEvent = useRoomLatestRenderedEvent(room);
    const tokenUsageValue = useMemo(
      () => getTokenUsageValue(getLatestTokenUsage(room, mx.getUserId())),
      [latestEvent, mx, room]
    );

    return (
      <Box
        className={css.RoomViewFollowing({ clickable: false })}
        alignItems="Center"
        justifyContent="End"
        {...props}
        ref={ref}
      >
        <span className={classNames(css.TokenUsageBadge, className)}>
          <Text as="span" className={css.TokenUsageLabel} size="T200">
            会话Token累计
          </Text>
          <Text as="span" className={css.TokenUsageValue} size="T300">
            {tokenUsageValue}
          </Text>
        </span>
      </Box>
    );
  }
);
