"use client";

import { useTranslations } from "next-intl";
import { LocaleMenu } from "@/components/locale-menu";
import { ThemeToggle } from "@/components/theme-toggle";

export function Navbar() {
  const t = useTranslations("nav");

  return (
    <nav className="sticky top-0 z-50 w-full border-b border-border bg-card">
      <div className="mx-auto flex h-14 max-w-7xl items-center px-6">
        {/* Logo */}
        <a href="/" className="flex items-center">
          <span className="text-lg font-bold tracking-widest text-foreground">
            SCAN<span className="text-primary">DIUM</span>
          </span>
        </a>

        {/* Nav links */}
        <div className="ml-8 hidden items-center gap-6 md:flex"></div>

        {/* Right side */}
        <div className="ml-auto flex items-center">
          <LocaleMenu />
          <ThemeToggle />
          <button
            type="button"
            onClick={() => {}}
            className="text-sm font-bold text-foreground hover:text-muted-foreground transition-colors px-2.5"
          >
            {t("login")}
          </button>
        </div>
      </div>
    </nav>
  );
}
