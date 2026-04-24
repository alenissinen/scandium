"use client";

import { Globe } from "lucide-react";
import { usePathname, useRouter } from "next/navigation";
import { useLocale, useTranslations } from "next-intl";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { type Locale, locales } from "@/i18n/request";

export function LocaleMenu() {
  const locale = useLocale();
  const router = useRouter();
  const pathName = usePathname();
  const tLang = useTranslations("language");
  const tNav = useTranslations("nav");
  const [dropdownOpen, setDropdownOpen] = useState(false);

  function switchLocale(next: Locale) {
    const segments = pathName.split("/");
    segments[1] = next;
    router.push(segments.join("/"));
  }

  return (
    <DropdownMenu open={dropdownOpen} onOpenChange={setDropdownOpen}>
      <Tooltip open={dropdownOpen ? false : undefined}>
        <DropdownMenuTrigger asChild>
          <TooltipTrigger asChild>
            <Button variant="ghost" size="icon">
              <Globe size={14} />
            </Button>
          </TooltipTrigger>
        </DropdownMenuTrigger>
        <TooltipContent>{tNav("language")}</TooltipContent>
      </Tooltip>
      <DropdownMenuContent align="end">
        {locales.map((language) => (
          <DropdownMenuItem
            key={language}
            onClick={() => switchLocale(language)}
            className={locale === language ? "text-primary" : ""}
          >
            {tLang(language)}
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
