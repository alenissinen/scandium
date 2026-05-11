"use client";

import { useRouter, useSearchParams } from "next/navigation";
import { useTranslations } from "next-intl";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

export function PriceFilter() {
  const t = useTranslations("filters");
  const router = useRouter();
  const searchParams = useSearchParams();

  const [min, setMin] = useState(searchParams.get("min_price") ?? "");
  const [max, setMax] = useState(searchParams.get("max_price") ?? "");

  function apply() {
    const params = new URLSearchParams(searchParams.toString());

    if (min) {
      params.set("min_price", min);
    } else {
      params.delete("min_price");
    }

    if (max) {
      params.set("max_price", max);
    } else {
      params.delete("max_price");
    }

    params.delete("page");
    router.push(`?${params.toString()}`);
  }

  return (
    <div className="flex flex-col gap-2">
      <div className="flex items-center gap-2">
        <Input
          type="number"
          placeholder="Min €"
          value={min}
          onChange={(e) => setMin(e.target.value)}
          className="h-8 text-xs w-fit"
          min={0}
        />
        <span className="text-muted-foreground text-xs shrink-0">-</span>
        <Input
          type="number"
          placeholder="Max €"
          value={max}
          onChange={(e) => setMax(e.target.value)}
          className="h-8 text-xs w-fit"
          min={min ?? 0}
        />
      </div>
      <Button size="sm" className="w-full" onClick={apply}>
        {t("apply")}
      </Button>
    </div>
  );
}
