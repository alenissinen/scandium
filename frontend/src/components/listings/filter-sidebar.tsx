"use client";

import { SlidersHorizontal } from "lucide-react";
import { useTranslations } from "next-intl";
import { CategoryFilter } from "@/components/listings/filters/category-filter";
import { Button } from "@/components/ui/button";
import { Sheet, SheetContent, SheetTitle, SheetTrigger } from "@/components/ui/sheet";

type FilterSectionProps = {
  title: string;
  children?: React.ReactNode;
};

function FilterSection({ title, children }: FilterSectionProps) {
  return (
    <div className="flex flex-col gap-3 py-4 border-b border-border last:border-0">
      <p className="text-xs font-medium text-muted-foreground uppercase tracking-wider">{title}</p>
      {children}
    </div>
  );
}

function SidebarContent() {
  const t = useTranslations("filters");

  return (
    <aside className="w-full px-4 min-h-screen">
      <FilterSection title={t("category")}>
        <CategoryFilter />
      </FilterSection>
    </aside>
  );
}

export function FilterSidebar() {
  const t = useTranslations("filters");

  return (
    <>
      {/* Desktop */}
      <div className="hidden lg:block">
        <SidebarContent />
      </div>

      {/* Phone */}
      <div className="lg:hidden fixed bottom-4 right-4 z-50">
        <Sheet>
          <SheetTrigger asChild>
            <Button size="lg" className="gap-2 shadow-lg">
              <SlidersHorizontal size={16} />
            </Button>
          </SheetTrigger>
          <SheetContent side="left" className="w-32 p-0">
            <SheetTitle className="sr-only">{t("filters")}</SheetTitle>
            <SidebarContent />
          </SheetContent>
        </Sheet>
      </div>
    </>
  );
}
