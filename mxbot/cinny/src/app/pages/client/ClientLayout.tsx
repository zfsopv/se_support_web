import React, { ReactNode } from 'react';
import { Box } from 'folds';
import { PageNavProvider } from '../../state/pageNav';

type ClientLayoutProps = {
  nav: ReactNode;
  children: ReactNode;
};
export function ClientLayout({ nav, children }: ClientLayoutProps) {
  return (
    <PageNavProvider>
      <Box grow="Yes">
        <Box shrink="No">{nav}</Box>
        <Box grow="Yes">{children}</Box>
      </Box>
    </PageNavProvider>
  );
}
