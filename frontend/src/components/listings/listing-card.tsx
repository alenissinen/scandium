import { ImageOff } from "lucide-react";
import Image from "next/image";
import { useTranslations } from "next-intl";
import type { Listing, ListingCondition } from "@/types/listing";

const conditionConfig: Record<ListingCondition, { labelKey: string; dotClassName: string }> = {
  new: { labelKey: "new", dotClassName: "bg-blue-400" },
  excellent: { labelKey: "excellent", dotClassName: "bg-green-400" },
  good: { labelKey: "good", dotClassName: "bg-yellow-400" },
  used: { labelKey: "used", dotClassName: "bg-zinc-400" },
};

type ListingCardProps = {
  listing: Listing;
};

export function ListingCard({ listing }: ListingCardProps) {
  const t = useTranslations();
  const condition = conditionConfig[listing.condition];

  return (
    <div className="bg-card border border-border rounded-xl overflow-hidden cursor-pointer hover:border-accent transition-colors">
      <div className="h-40 bg-muted relative overflow-hidden">
        {listing.image_url ? (
          <Image
            src={listing.image_url}
            alt={listing.title}
            fill
            sizes="(max-width: 768px) 50vw, (max-width: 1024px) 33vw, 25vw"
            className="object-cover"
          />
        ) : (
          <div className="w-full h-full flex items-center justify-center">
            <ImageOff size={32} className="text-border" />
          </div>
        )}
      </div>

      <div className="p-3">
        <div className="flex items-center justify-between mb-1">
          <p className="text-sm font-medium text-foreground truncate flex-1">{listing.title}</p>
          <span className="text-xs text-muted-foreground shrink-0 ml-2 flex items-center gap-1">
            <span className={`w-1.5 h-1.5 rounded-full shrink-0 ${condition.dotClassName}`} />
            {t(`conditions.${condition.labelKey}`)}
          </span>
        </div>
        <p className="text-xs text-muted-foreground mb-3">
          {t(`categories.${listing.category}`)}
          {listing.year ? ` · ${listing.year}` : ""}
        </p>
        <div className="flex items-center justify-between">
          <span className="text-sm font-semibold text-primary">{listing.price} €</span>
          <span className="text-xs text-muted-foreground">{listing.location}</span>
        </div>
      </div>
    </div>
  );
}
