"use client";

import { Search } from "lucide-react";
import { useSearchParams } from "next/navigation";
import { useTranslations } from "next-intl";
import { useRouter } from "@/i18n/navigation";

export function SearchBar({ className }: { className?: string }) {
  const router = useRouter();
  const searchParams = useSearchParams();
  const t = useTranslations("search");

  function handleSubmit(formData: FormData) {
    const q = (formData.get("q") as string)?.trim() ?? "";
    const params = new URLSearchParams(searchParams.toString());

    if (q.length > 0) {
      params.set("q", q);
    } else {
      params.delete("q");
    }

    params.delete("page");
    router.push(`/?${params.toString()}`);
  }

  return (
    <form action={handleSubmit} className={className}>
      <div className="relative">
        <Search
          size={14}
          className="absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground"
        />
        <input
          name="q"
          type="search"
          defaultValue={searchParams.get("q") ?? ""}
          placeholder={t("placeholder")}
          className="w-full bg-muted border border-border rounded-md pl-8 pr-3 py-1.5 text-sm text-foreground placeholder:text-muted-foreground outline-none focus:border-primary transition-colors"
        />
      </div>
    </form>
  );
}
