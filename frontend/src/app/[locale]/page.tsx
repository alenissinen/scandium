import { getTranslations } from "next-intl/server";
import { Suspense } from "react";
import { FilterSidebar } from "@/components/listings/filter-sidebar";
import { ListingCard } from "@/components/listings/listing-card";
import { Pagination } from "@/components/listings/pagination";
import { Navbar } from "@/components/navbar/navbar";
import { SearchBar } from "@/components/search-bar";
import { TooltipProvider } from "@/components/ui/tooltip";
import type { Listing } from "@/types/listing";

type SearchParams = {
  q?: string;
  listing_type?: string;
  condition?: string;
  min_price?: string;
  max_price?: string;
  page?: string;
};

async function fetchListings(params: SearchParams) {
  const url = new URL(`${process.env.NEXT_PUBLIC_API_URL}/api/v1/listings`);

  if (params.q) url.searchParams.set("q", params.q);
  if (params.listing_type) url.searchParams.set("listing_type", params.listing_type);
  if (params.condition) url.searchParams.set("condition", params.condition);
  if (params.min_price) url.searchParams.set("min_price", params.min_price);
  if (params.max_price) url.searchParams.set("max_price", params.max_price);
  if (params.page) url.searchParams.set("page", params.page);

  const response = await fetch(url.toString(), { cache: "no-store" });

  if (!response.ok) return { listings: [], total: 0 };

  return response.json() as Promise<{ listings: Listing[]; total: number }>;
}

const PER_PAGE = 21;

export default async function Home({ searchParams }: { searchParams: Promise<SearchParams> }) {
  const params = await searchParams;
  const t = await getTranslations("listings");

  const { listings, total } = await fetchListings(params);
  const currentPage = Number(params.page) || 1;
  const totalPages = Math.ceil(total / PER_PAGE);

  return (
    <TooltipProvider>
      <Navbar />
      <div className="md:hidden px-4 py-2 border-b border-border">
        <Suspense fallback={null}>
          <SearchBar className="w-full" />
        </Suspense>
      </div>
      <div className="flex max-w-7xl mx-auto">
        <FilterSidebar />
        <main className="flex-1 min-h-screen bg-background p-4">
          <div className="flex items-center justify-between mb-3">
            <span className="text-sm text-muted-foreground font-medium">
              {`${total} ${t("amount")}`}
            </span>
          </div>
          <div className="grid grid-cols-2 md:grid-cols-3 gap-3">
            {listings.map((listing) => (
              <ListingCard key={listing.id} listing={listing} />
            ))}
          </div>
          <Suspense fallback={null}>
            <Pagination currentPage={currentPage} totalPages={totalPages} />
          </Suspense>
        </main>
      </div>
    </TooltipProvider>
  );
}
