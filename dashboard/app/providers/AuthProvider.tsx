import React, { createContext, useContext, useEffect, useState } from 'react';
import { API_LOGOUT_URL } from '~/constants';
import { fetchMe } from '~/services/auth';
import { useCsrf } from './CsrfProvider';

type User = { username: string } | null;

type AuthContextType = {
  user: User;
  isLoading: boolean;
  login: (username: string, csrfToken: string) => void;
  logout: () => Promise<void>;
};

const AuthContext = createContext<AuthContextType | null>(null);

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState<User>(null);
  const [isLoading, setIsLoading] = useState(true);
  const { updateCsrfToken } = useCsrf();

  useEffect(() => {
    fetchMe()
      .then((r) => (r.ok ? r.json() : null))
      .then((data) => {
        if (data) setUser(data);
      })
      .catch(() => {
        setUser(null);
      })
      .finally(() => {
        setIsLoading(false);
      });
  }, []);

  function login(username: string, csrfToken: string) {
    setUser({ username });
    updateCsrfToken(csrfToken);
  }

  async function logout() {
    await fetch(API_LOGOUT_URL, {
      method: 'POST',
      credentials: 'include',
    });
    setUser(null);
  }

  return (
    <AuthContext.Provider value={{ user, login, logout, isLoading }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error('useAuth must be used inside AuthProvider');
  return ctx;
}
