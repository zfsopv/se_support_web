import { style } from '@vanilla-extract/css';
import { recipe } from '@vanilla-extract/recipes';
import { DefaultReset, color, config, toRem } from 'folds';

export const RoomViewFollowingPlaceholder = style([
  DefaultReset,
  {
    height: toRem(28),
  },
]);

export const RoomViewFollowing = recipe({
  base: [
    DefaultReset,
    {
      minHeight: toRem(28),
      padding: `${toRem(3)} ${config.space.S300}`,
      width: '100%',
      backgroundColor: color.Surface.Container,
      color: color.Surface.OnContainer,
      outline: 'none',
    },
  ],
  variants: {
    clickable: {
      true: {
        cursor: 'pointer',
        selectors: {
          '&:hover, &:focus-visible': {
            color: color.Primary.Main,
          },
          '&:active': {
            color: color.Primary.Main,
          },
        },
      },
    },
  },
});

export const TokenUsageBadge = style([
  DefaultReset,
  {
    display: 'inline-flex',
    alignItems: 'center',
    gap: config.space.S200,
    minHeight: toRem(20),
    padding: `${toRem(2)} ${config.space.S300}`,
    borderRadius: toRem(999),
    backgroundColor: color.Surface.ContainerHigh,
    color: color.Surface.OnContainer,
    border: 'none',
    boxShadow: 'none',
    opacity: 0.8,
    whiteSpace: 'nowrap',
    flexShrink: 0,
  },
]);

export const TokenUsageLabel = style([
  DefaultReset,
  {
    opacity: 0.72,
    fontWeight: config.fontWeight.W400,
    letterSpacing: '0.01em',
  },
]);

export const TokenUsageValue = style([
  DefaultReset,
  {
    fontWeight: config.fontWeight.W600,
    letterSpacing: '0.02em',
  },
]);
