import { MatrixEvent, MatrixClient } from 'matrix-js-sdk';

const CONTINUATION_MAX_INTERVAL = 5 * 60 * 1000; // 5 minutes

const continuedTypes = ['m.sticker', 'm.room.message'];

/**
 * Determines whether mxEvent should be visually grouped with prevEvent.
 * Ported from Element Web's shouldFormContinuation in MessagePanel.tsx.
 * This is a pure event-to-event comparison — external checks like
 * dayDivider, newDivider, and isPrevRendered are handled by the caller.
 */
export function shouldFormContinuation(
  prevEvent: MatrixEvent | undefined | null,
  mxEvent: MatrixEvent,
): boolean {
  if (!prevEvent?.sender || !mxEvent.sender) return false;

  if (mxEvent.getTs() - prevEvent.getTs() > CONTINUATION_MAX_INTERVAL) return false;

  if (mxEvent.isRedacted() !== prevEvent.isRedacted()) return false;

  if (
    mxEvent.getType() !== prevEvent.getType() &&
    (!continuedTypes.includes(mxEvent.getType()) ||
      !continuedTypes.includes(prevEvent.getType()))
  ) {
    return false;
  }

  if (
    mxEvent.sender.userId !== prevEvent.sender.userId ||
    mxEvent.sender.name !== prevEvent.sender.name ||
    mxEvent.sender.getMxcAvatarUrl() !== prevEvent.sender.getMxcAvatarUrl()
  ) {
    return false;
  }

  return true;
}

/**
 * Checks whether a MatrixEvent's sender is a bot.
 * 1. Whitelist check (from settings botUserIds)
 * 2. Checks the m.room.member event content for user_type === 'bot'
 */
export function isBotSender(
  userId: string | undefined,
  botUserIds: string[],
  mxEvent: MatrixEvent,
): boolean {
  if (!userId) return false;
  if (botUserIds.includes(userId)) return true;

  const sender = mxEvent.sender;
  if (!sender) return false;

  const memberEvent = sender.events?.member;
  if (!memberEvent) return false;

  const content = memberEvent.getContent();
  return content?.user_type === 'bot';
}
