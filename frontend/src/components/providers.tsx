"use client";

import { ThemeProvider } from "@teispace/next-themes";
import { UserProvider } from "@/contexts/user-context";

export function Providers({ children }: { children: React.ReactNode }) {
  return (
    <UserProvider>
      <ThemeProvider
        attribute="class"
        defaultTheme="dark"
        enableSystem={false}
        themes={["light", "dark"]}
      >
        {children}
      </ThemeProvider>
    </UserProvider>
  );
}
