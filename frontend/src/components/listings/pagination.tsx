"use client";

import { ChevronLeft, ChevronRight } from "lucide-react";
import { useSearchParams } from "next/navigation";
import { Button } from "@/components/ui/button";
import { useRouter } from "@/i18n/navigation";

type PaginationProps = {
  currentPage: number;
  totalPages: number;
};

export function Pagination({ currentPage, totalPages }: PaginationProps) {
  const router = useRouter();
  const searchParams = useSearchParams();

  function navigate(page: number) {
    const params = new URLSearchParams(searchParams.toString());
    params.set("page", String(page));
    router.push(`?${params.toString()}`);
  }

  if (totalPages <= 1) return null;

  const pages = Array.from({ length: totalPages }, (_, i) => i + 1);
  const visiblePages = pages.filter(
    (p) => p === 1 || p === totalPages || Math.abs(p - currentPage) <= 1
  );

  return (
    <div className="flex items-center justify-center gap-1 mt-8">
      <Button
        variant="ghost"
        size="icon-sm"
        disabled={currentPage === 1}
        onClick={() => navigate(currentPage - 1)}
      >
        <ChevronLeft size={14} />
      </Button>

      {visiblePages.map((page, i) => {
        const prev = visiblePages[i - 1];
        const showEllipsis = prev && page - prev > 1;

        return (
          <div key={page} className="flex items-center gap-1">
            {showEllipsis && <span className="text-xs text-muted-foreground px-1">...</span>}
            <Button
              variant="ghost"
              size="icon-sm"
              onClick={() => navigate(page)}
              className={currentPage === page ? "bg-muted text-foreground" : ""}
            >
              <span className="text-xs">{page}</span>
            </Button>
          </div>
        );
      })}

      <Button
        variant="ghost"
        size="icon-sm"
        disabled={currentPage === totalPages}
        onClick={() => navigate(currentPage + 1)}
      >
        <ChevronRight size={14} />
      </Button>
    </div>
  );
}
