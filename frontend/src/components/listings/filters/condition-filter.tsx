"use client";

import { Check } from "lucide-react";
import { useSearchParams } from "next/navigation";
import { useTranslations } from "next-intl";
import { Button } from "@/components/ui/button";
import { useRouter } from "@/i18n/navigation";
import { type ListingCondition, listingConditions } from "@/types/listing";

export function ConditionFilter() {
  const t = useTranslations("conditions");
  const router = useRouter();
  const searchParams = useSearchParams();

  const active = (searchParams.get("condition") ?? "")
    .split("|")
    .filter(Boolean) as ListingCondition[];

  function toggle(condition: ListingCondition) {
    const params = new URLSearchParams(searchParams.toString());

    const next = active.includes(condition)
      ? active.filter((c) => c !== condition)
      : [...active, condition];

    if (next.length === 0) {
      params.delete("condition");
    } else {
      params.set("condition", next.join("|"));
    }

    params.delete("page");
    router.push(`?${params.toString()}`);
  }

  return (
    <div className="flex flex-col gap-1">
      {listingConditions.map((condition) => {
        const isActive = active.includes(condition);

        return (
          <Button
            key={condition}
            variant="ghost"
            size="sm"
            onClick={() => toggle(condition)}
            className={`w-full justify-between ${
              isActive ? "text-foreground" : "text-muted-foreground"
            }`}
          >
            <span className="flex items-center gap-2">{t(condition)}</span>
            {isActive && <Check size={12} />}
          </Button>
        );
      })}
    </div>
  );
}
