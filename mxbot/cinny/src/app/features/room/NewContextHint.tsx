import React from 'react';
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
