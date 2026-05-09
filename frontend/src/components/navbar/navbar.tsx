"use client";

import { useTranslations } from "next-intl";
import { Suspense } from "react";
import { LocaleMenu } from "@/components/navbar/locale-menu";
import { ThemeToggle } from "@/components/navbar/theme-toggle";
import { SearchBar } from "@/components/search-bar";
import { Button } from "@/components/ui/button";
import { useUser } from "@/contexts/user-context";
import { Link } from "@/i18n/navigation";

export function Navbar() {
  const t = useTranslations("nav");
  const { user } = useUser();

  return (
    <nav className="sticky top-0 z-50 w-full border-b border-border bg-card">
      <div className="mx-auto flex h-14 max-w-7xl items-center px-6">
        <a href="/" className="flex items-center">
          <span className="text-lg font-bold tracking-widest text-foreground">
            SCAN<span className="text-primary">DIUM</span>
          </span>
        </a>
        <div className="flex-1 justify-center hidden md:flex">
          <div className="w-full max-w-md">
            <Suspense fallback={null}>
              <SearchBar className="w-full" />
            </Suspense>
          </div>
        </div>
        <div className="ml-auto flex items-center">
          <LocaleMenu />
          <ThemeToggle />
          {user ? (
            <Button variant="ghost" size="lg" asChild>
              <Link href="/settings" className="text-base text-foreground transition-colors px-2.5">
                {t("profile")}
              </Link>
            </Button>
          ) : (
            <Button variant="ghost" size="lg" asChild>
              <Link href="/auth" className="text-base text-foreground transition-colors px-2.5">
                {t("login")}
              </Link>
            </Button>
          )}
        </div>
      </div>
    </nav>
  );
}
