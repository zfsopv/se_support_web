import { ValidatedAuthMetadata } from 'matrix-js-sdk';
import { createContext, useContext, useEffect, useState } from 'react';
import { useMatrixClient } from './useMatrixClient';

const AuthMetadataContext = createContext<ValidatedAuthMetadata | undefined>(undefined);

type AuthMetadataCacheEntry = {
  loaded: boolean;
  metadata?: ValidatedAuthMetadata;
  promise?: Promise<ValidatedAuthMetadata | undefined>;
};

const authMetadataCache = new Map<string, AuthMetadataCacheEntry>();

const getCacheEntry = (homeserverUrl: string): AuthMetadataCacheEntry => {
  const cached = authMetadataCache.get(homeserverUrl);
  if (cached) return cached;

  const entry: AuthMetadataCacheEntry = { loaded: false };
  authMetadataCache.set(homeserverUrl, entry);
  return entry;
};

export const AuthMetadataProvider = AuthMetadataContext.Provider;

export const useAuthMetadata = (): ValidatedAuthMetadata | undefined => {
  const providedMetadata = useContext(AuthMetadataContext);
  const mx = useMatrixClient();
  const homeserverUrl = mx.getHomeserverUrl();
  const [lazyMetadata, setLazyMetadata] = useState<ValidatedAuthMetadata | undefined>(() =>
    providedMetadata ?? getCacheEntry(homeserverUrl).metadata
  );

  useEffect(() => {
    const cacheEntry = getCacheEntry(homeserverUrl);

    if (providedMetadata) {
      cacheEntry.loaded = true;
      cacheEntry.metadata = providedMetadata;
      cacheEntry.promise = Promise.resolve(providedMetadata);
      setLazyMetadata(providedMetadata);
      return;
    }

    if (cacheEntry.loaded) {
      setLazyMetadata(cacheEntry.metadata);
      return;
    }

    let cancelled = false;

    if (!cacheEntry.promise) {
      cacheEntry.promise = mx
        .getAuthMetadata()
        .then((metadata) => {
          cacheEntry.metadata = metadata;
          return metadata;
        })
        .catch(() => undefined)
        .finally(() => {
          cacheEntry.loaded = true;
        });
    }

    cacheEntry.promise.then((metadata) => {
      if (!cancelled) {
        setLazyMetadata(metadata);
      }
    });

    return () => {
      cancelled = true;
    };
  }, [providedMetadata, mx, homeserverUrl]);

  return providedMetadata ?? lazyMetadata;
};
