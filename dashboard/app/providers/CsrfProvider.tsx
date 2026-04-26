import type React from 'react';
import { createContext, useContext, useEffect, useState } from 'react';
import { fetchCsrf } from '~/services/csrf';

type CsrfContextType = {
  csrfToken: string | null;
};

const CsrfContext = createContext<CsrfContextType | null>(null);

export function CsrfProvider({ children }: { children: React.ReactNode }) {
  const [csrfToken, setCsrfToken] = useState<string | null>(null);
  useEffect(() => {
    fetchCsrf()
      .then((r) => (r.ok ? r.json() : null))
      .then((data) => {
        if (data) setCsrfToken(data.csrfToken);
      })
      .catch(() => {
        setCsrfToken(null);
      });
  }, []);
  return <CsrfContext.Provider value={{ csrfToken }}>{children}</CsrfContext.Provider>;
}

export function useCsrf() {
  const ctx = useContext(CsrfContext);
  if (!ctx) throw new Error('useCsrf must be insice CsrfProvider');
  return ctx;
}
