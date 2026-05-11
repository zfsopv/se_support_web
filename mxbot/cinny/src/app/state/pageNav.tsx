import React, { ReactNode, createContext, useContext, useMemo, useState } from 'react';

type PageNavContextState = {
  visible: boolean;
  setVisible: (visible: boolean) => void;
  toggle: () => void;
};

const PageNavContext = createContext<PageNavContextState | undefined>(undefined);

export function PageNavProvider({ children }: { children: ReactNode }) {
  const [visible, setVisible] = useState(true);

  const value = useMemo<PageNavContextState>(
    () => ({
      visible,
      setVisible,
      toggle: () => setVisible((current) => !current),
    }),
    [visible]
  );

  return <PageNavContext.Provider value={value}>{children}</PageNavContext.Provider>;
}

export function usePageNav() {
  const context = useContext(PageNavContext);

  if (!context) {
    throw new Error('usePageNav must be used within PageNavProvider');
  }

  return context;
}