import React from 'react';
import Relay from 'react-relay';
import { environment } from '~/relay/environment';

export function RelayProvider({ children }: { children: React.ReactNode }) {
  return (
    <Relay.RelayEnvironmentProvider environment={environment}>
      {children}
    </Relay.RelayEnvironmentProvider>
  );
}
