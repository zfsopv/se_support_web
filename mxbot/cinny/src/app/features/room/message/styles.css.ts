import { style } from '@vanilla-extract/css';
import { color, DefaultReset, config, toRem } from 'folds';

export const MessageBase = style({
  position: 'relative',
});
export const MessageBaseBubbleCollapsed = style({
  paddingTop: 0,
});

export const MessageOptionsBase = style([
  DefaultReset,
  {
    position: 'absolute',
    top: '50%',
    right: 'calc(100% + 8px)',
    transform: 'translateY(-50%)',
    zIndex: 1,
  },
]);
export const MessageOptionsBar = style([
  DefaultReset,
  {
    padding: config.space.S100,
  },
]);

export const BubbleAvatarBase = style({
  paddingTop: 0,
});

export const MessageAvatar = style({
  cursor: 'pointer',
});

export const MessageQuickReaction = style({
  minWidth: toRem(32),
});

export const MessageMenuGroup = style({
  padding: config.space.S100,
});

export const MessageMenuItemText = style({
  flexGrow: 1,
});

export const ReactionsContainer = style({
  selectors: {
    '&:empty': {
      display: 'none',
    },
  },
});

export const ReactionsTooltipText = style({
  wordBreak: 'break-word',
});

/* ===== Bot Message Continuation Styles ===== */

export const ContinuationSummaryBar = style({
  display: 'flex',
  alignItems: 'center',
  gap: config.space.S300,
  padding: `${config.space.S200} ${config.space.S400}`,
  borderRadius: config.radii.R400,
  border: `1px solid ${color.Surface.ContainerLine}`,
  backgroundColor: color.Surface.Container,
  marginTop: config.space.S200,
  cursor: 'pointer',
  maxWidth: toRem(400),
  selectors: {
    '&:hover': {
      backgroundColor: color.Surface.ContainerActive,
    },
  },
});

export const ContinuationSummaryIcon = style({
  width: toRem(24),
  height: toRem(24),
  borderRadius: '50%',
  border: `1px solid ${color.Surface.ContainerLine}`,
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
  flexShrink: 0,
  transition: 'transform 150ms ease',
});

export const ContinuationSummaryIconExpanded = style({
  transform: 'rotate(90deg)',
});

export const ContinuationSummaryText = style({
  flex: 1,
  minWidth: 0,
});

export const ContinuationSummaryTitle = style({
  color: color.Primary.Main,
});

export const ContinuationSummarySubtitle = style({
  color: color.Surface.OnSurfaceVariant,
  marginTop: toRem(2),
});

export const BotMessageContinuationExpanded = style({
  paddingLeft: toRem(42),
  borderLeft: `2px solid ${color.Surface.ContainerLine}`,
  selectors: {
    '& > *': {
      marginTop: 0,
      paddingTop: config.space.S100,
      paddingBottom: config.space.S100,
    },
  },
});
