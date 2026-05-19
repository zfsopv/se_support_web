import React, { ReactNode, useCallback, useMemo } from 'react';
import { AutoDiscoveryInfoProvider } from '../../hooks/useAutoDiscoveryInfo';
import { AsyncStatus, useAsyncCallbackValue } from '../../hooks/useAsyncCallback';
import { autoDiscovery, AutoDiscoveryInfo } from '../../cs-api';
import { getMxIdServer } from '../../utils/matrix';

type AutoDiscoveryProps = {
  userId: string;
  baseUrl: string;
  children: ReactNode;
};
export function AutoDiscovery({ userId, baseUrl, children }: AutoDiscoveryProps) {
  const server = getMxIdServer(userId);
  const [state] = useAsyncCallbackValue(
    useCallback(async () => {
      if (!server) {
        return [undefined, undefined] as const;
      }

      const normalizedBaseUrl = baseUrl.replace(/\/+$/g, '');
      if (normalizedBaseUrl === `https://${server}`) {
        return [undefined, undefined] as const;
      }

      return autoDiscovery(fetch, server);
    }, [baseUrl, server])
  );

  const [, info] = state.status === AsyncStatus.Success ? state.data : [];

  const fallback: AutoDiscoveryInfo = useMemo(
    () => ({
      'm.homeserver': {
        base_url: baseUrl,
      },
    }),
    [baseUrl]
  );

  return <AutoDiscoveryInfoProvider value={info ?? fallback}>{children}</AutoDiscoveryInfoProvider>;
}
