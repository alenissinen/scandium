"use client";

import type React from "react";
import { createContext, createElement, useCallback, useContext, useEffect, useState } from "react";

interface User {
  id: string;
  username: string;
  display_name: string;
  avatar_url?: string;
  location?: string;
  created_at: string;
}

interface UserContextType {
  user: User | null;
  refreshUser: () => void;
}

const UserContext = createContext<UserContextType>({
  user: null,
  refreshUser: () => {},
});

export function UserProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState<User | null>(null);

  const refreshUser = useCallback(() => {
    fetch(`${process.env.NEXT_PUBLIC_API_URL}/api/v1/auth/me`, {
      credentials: "include",
    })
      .then((res) => (res.ok ? res.json() : null))
      .then(setUser)
      .catch(() => setUser(null));
  }, []);

  useEffect(() => {
    refreshUser();
  }, [refreshUser]);

  return createElement(UserContext.Provider, {
    value: { user, refreshUser },
    children,
  });
}

export const useUser = () => useContext(UserContext);
