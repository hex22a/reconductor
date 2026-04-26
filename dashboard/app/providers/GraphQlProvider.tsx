import React, { useMemo } from 'react';
import Relay from 'react-relay';
import { createEnvironment } from '~/relay/environment';
import { useCsrf } from './CsrfProvider';

export function RelayProvider({ children }: { children: React.ReactNode }) {
  const { csrfToken } = useCsrf();
  const environment = useMemo(() => createEnvironment(csrfToken), [csrfToken]);
  return (
    <Relay.RelayEnvironmentProvider environment={environment}>
      {children}
    </Relay.RelayEnvironmentProvider>
  );
}
