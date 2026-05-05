import { getTranslations } from "next-intl/server";
import { ListingCard } from "@/components/listings/listing-card";
import { Pagination } from "@/components/listings/pagination";
import { Navbar } from "@/components/navbar/navbar";
import { TooltipProvider } from "@/components/ui/tooltip";
import { MOCK_LISTINGS } from "@/lib/mock-data";

const LISTINGS_PER_PAGE = 20;

export default async function HomePage({
  searchParams,
}: {
  searchParams: Promise<{ page?: string }>;
}) {
  const { page } = await searchParams;
  const t = await getTranslations("listings");

  const currentPage = Math.max(1, Number(page) || 1);
  const totalPages = Math.ceil(MOCK_LISTINGS.length / LISTINGS_PER_PAGE);
  const start = (currentPage - 1) * LISTINGS_PER_PAGE;
  const listings = MOCK_LISTINGS.slice(start, start + LISTINGS_PER_PAGE);

  return (
    <TooltipProvider>
      <Navbar />
      <main className="min-h-screen bg-background p-4 max-w-7xl mx-auto">
        <div className="flex items-center justify-between mb-3">
          <span className="text-sm text-muted-foreground font-medium">{`${MOCK_LISTINGS.length} ${t("amount")}`}</span>
        </div>
        <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3">
          {listings.map((listing) => (
            <ListingCard key={listing.id} listing={listing} />
          ))}
        </div>
        <Pagination currentPage={currentPage} totalPages={totalPages} />
      </main>
    </TooltipProvider>
  );
}
