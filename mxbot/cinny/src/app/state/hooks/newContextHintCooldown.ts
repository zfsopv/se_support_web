import { createContext, useContext } from 'react';
import { NewContextHintCooldownAtom } from '../newContextHintCooldown';

const NewContextHintCooldownAtomContext = createContext<NewContextHintCooldownAtom | null>(null);
export const NewContextHintCooldownProvider = NewContextHintCooldownAtomContext.Provider;

export const useNewContextHintCooldownAtom = (): NewContextHintCooldownAtom => {
  const anAtom = useContext(NewContextHintCooldownAtomContext);

  if (!anAtom) {
    throw new Error('NewContextHintCooldownAtom is not provided!');
  }

  return anAtom;
};
