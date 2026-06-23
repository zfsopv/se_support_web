import { WritableAtom, atom } from 'jotai';
import produce from 'immer';
import {
  atomWithLocalStorage,
  getLocalStorageItem,
  setLocalStorageItem,
} from './utils/atomWithLocalStorage';

const NEW_CONTEXT_HINT_COOLDOWN = 'newContextHintCooldown';

// 冷却时长（ms）：同时作为冷却判定阈值与过期清理阈值，由 useNewContextHint 复用
export const COOLDOWN_MS = 10 * 60 * 1000;

const getStoreKey = (userId: string): string => `${NEW_CONTEXT_HINT_COOLDOWN}${userId}`;

type NewContextHintCooldown = Map<string, number>;

type NewContextHintCooldownAction = { roomId: string; ts: number };

export type NewContextHintCooldownAtom = WritableAtom<
  NewContextHintCooldown,
  [NewContextHintCooldownAction],
  void
>;

export const makeNewContextHintCooldownAtom = (userId: string): NewContextHintCooldownAtom => {
  const storeKey = getStoreKey(userId);

  const baseCooldownAtom = atomWithLocalStorage<NewContextHintCooldown>(
    storeKey,
    (key) => new Map(getLocalStorageItem<[string, number][]>(key, [])),
    (key, value) => setLocalStorageItem(key, Array.from(value.entries()))
  );

  const cooldownAtom = atom<NewContextHintCooldown, [NewContextHintCooldownAction], void>(
    (get) => get(baseCooldownAtom),
    (get, set, { roomId, ts }) => {
      set(
        baseCooldownAtom,
        produce(get(baseCooldownAtom), (draft) => {
          draft.set(roomId, ts);
          // 清理过期项，控制体积
          const expired: string[] = [];
          draft.forEach((at, id) => {
            if (id !== roomId && ts - at > COOLDOWN_MS) expired.push(id);
          });
          expired.forEach((id) => draft.delete(id));
        })
      );
    }
  );

  return cooldownAtom;
};

export const clearNewContextHintCooldownStore = (userId: string) => {
  localStorage.removeItem(getStoreKey(userId));
};
