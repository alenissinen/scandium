"use client";

import { useTheme } from "@teispace/next-themes";
import { Moon, Sun } from "lucide-react";
import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";

export function ThemeToggle() {
  const { theme, setTheme } = useTheme();
  const [mounted, setMounted] = useState(false);

  // Avoid hydration mismatch
  useEffect(() => setMounted(true), []);
  if (!mounted) return null;

  const isDarkTheme = theme === "dark";

  return (
    <Button
      variant="ghost"
      size="icon"
      onClick={() => setTheme(isDarkTheme ? "light" : "dark")}
      aria-label={isDarkTheme ? "Switch to light mode" : "Switch to dark mode"}
    >
      {isDarkTheme ? <Sun size={16} /> : <Moon size={16} />}
    </Button>
  );
}
