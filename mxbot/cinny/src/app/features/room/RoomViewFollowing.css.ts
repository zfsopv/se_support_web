import { style } from '@vanilla-extract/css';
import { recipe } from '@vanilla-extract/recipes';
import { DefaultReset, color, config, toRem } from 'folds';

export const RoomViewFollowingPlaceholder = style([
  DefaultReset,
  {
    height: toRem(40),
  },
]);

export const RoomViewFollowing = recipe({
  base: [
    DefaultReset,
    {
      minHeight: toRem(40),
      padding: `0 ${config.space.S300}`,
      width: '100%',
      backgroundColor: color.Surface.Container,
      color: color.Surface.OnContainer,
      outline: 'none',
      borderTop: `${toRem(1)} solid ${color.Surface.ContainerLine}`,
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
    justifyContent: 'center',
    minHeight: toRem(28),
    maxWidth: toRem(180),
    padding: `0 ${config.space.S300}`,
    borderRadius: toRem(999),
    backgroundColor: color.Secondary.Container,
    color: color.Secondary.OnContainer,
    border: `${toRem(1)} solid ${color.Secondary.ContainerLine}`,
    boxShadow: `0 ${toRem(4)} ${toRem(12)} ${toRem(-8)} rgba(0 0 0 / 0.28)`,
    fontWeight: config.fontWeight.W500,
    letterSpacing: '0.01em',
    whiteSpace: 'nowrap',
  },
]);
