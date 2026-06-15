import React, { ReactNode, useState } from 'react';
import { MatrixEvent } from 'matrix-js-sdk';
import { Text } from 'folds';
import * as css from './styles.css';

type BotMessageContinuationProps = {
  events: MatrixEvent[];
  renderEvent: (event: MatrixEvent, collapse: boolean) => ReactNode;
};

export function BotMessageContinuation({ events, renderEvent }: BotMessageContinuationProps) {
  const [expanded, setExpanded] = useState(false);

  if (events.length === 0) return null;

  const hasReply = events.some((e) => e.replyEventId);
  const foldThreshold = hasReply ? 1 : 2;

  if (events.length < foldThreshold) {
    return <>{renderEvent(events[0], false)}</>;
  }

  const count = events.length;

  return (
    <>
      <div
        className={css.ContinuationSummaryBar}
        onClick={() => setExpanded(!expanded)}
        role="button"
        tabIndex={0}
        onKeyDown={(e) => {
          if (e.key === 'Enter' || e.key === ' ') {
            e.preventDefault();
            setExpanded(!expanded);
          }
        }}
      >
        <div
          className={`${css.ContinuationSummaryIcon}${expanded ? ` ${css.ContinuationSummaryIconExpanded}` : ''}`}
        >
          <Text size="T200">▶</Text>
        </div>
        <div className={css.ContinuationSummaryText}>
          <Text className={css.ContinuationSummaryTitle} size="T300">
            {expanded ? '收起以上消息' : '助手正在查阅并分析足够的信息'}
          </Text>
          {!expanded && (
            <Text className={css.ContinuationSummarySubtitle} size="T200">
              {count} 条消息
            </Text>
          )}
        </div>
      </div>

      {expanded && (
        <div className={css.BotMessageContinuationExpanded}>
          {events.map((event, i) => (
            <React.Fragment key={event.getId() ?? `bot-ev-${i}`}>
              {renderEvent(event, i > 0)}
            </React.Fragment>
          ))}
        </div>
      )}
    </>
  );
}
