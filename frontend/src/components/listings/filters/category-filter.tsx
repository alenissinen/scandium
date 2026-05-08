"use client";

import { ChevronDown } from "lucide-react";
import { useSearchParams } from "next/navigation";
import { useTranslations } from "next-intl";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { useRouter } from "@/i18n/navigation";
import { type ListingCategory, listingCategories } from "@/types/listing";

export function CategoryFilter() {
  const t = useTranslations("categories");
  const router = useRouter();
  const searchParams = useSearchParams();
  const active = searchParams.get("category");

  function select(category: ListingCategory | null) {
    const params = new URLSearchParams(searchParams.toString());

    if (category === null) {
      params.delete("category");
    } else {
      params.set("category", category);
    }

    params.delete("page");
    router.push(`?${params.toString()}`);
  }

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="outline" size="sm" className="w-full justify-between">
          {active ? t(active) : t("all")}
          <ChevronDown size={12} />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-42">
        <DropdownMenuItem onClick={() => select(null)}>{t("all")}</DropdownMenuItem>
        {listingCategories.map((category) => (
          <DropdownMenuItem
            key={category}
            onClick={() => select(category)}
            className={active === category ? "text-primary" : ""}
          >
            {t(category)}
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
