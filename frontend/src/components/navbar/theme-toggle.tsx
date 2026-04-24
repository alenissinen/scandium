"use client";

import { useTheme } from "@teispace/next-themes";
import { Moon, Sun } from "lucide-react";
import { useTranslations } from "next-intl";
import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";

export function ThemeToggle() {
  const { theme, setTheme } = useTheme();
  const [mounted, setMounted] = useState(false);
  const t = useTranslations("nav");

  // Avoid hydration mismatch
  useEffect(() => setMounted(true), []);
  if (!mounted) return null;

  const isDarkTheme = theme === "dark";

  return (
    <Tooltip key={"bottom"}>
      <TooltipTrigger asChild>
        <Button
          variant="ghost"
          size="icon"
          onClick={() => setTheme(isDarkTheme ? "light" : "dark")}
          aria-label={isDarkTheme ? "Switch to light mode" : "Switch to dark mode"}
        >
          {isDarkTheme ? <Sun size={16} /> : <Moon size={16} />}
        </Button>
      </TooltipTrigger>
      <TooltipContent>{t(isDarkTheme ? "themeDark" : "themeLight")}</TooltipContent>
    </Tooltip>
  );
}
