import React from 'react';
import { Icon, Icons } from 'folds';
import { SidebarItem, SidebarItemTooltip, SidebarAvatar } from '../../../components/sidebar';
import { useMatrixClient } from '../../../hooks/useMatrixClient';
import { logoutClient } from '../../../../client/initMatrix';

export function SettingsTab() {
  const mx = useMatrixClient();

  const handleLogout = async () => {
    await logoutClient(mx);
  };

  return (
    <SidebarItem active={false}>
      <SidebarItemTooltip tooltip="退出登录">
        {(triggerRef) => (
          <SidebarAvatar as="button" ref={triggerRef} onClick={handleLogout}>
            <Icon src={Icons.Power} />
          </SidebarAvatar>
        )}
      </SidebarItemTooltip>
    </SidebarItem>
  );
}
