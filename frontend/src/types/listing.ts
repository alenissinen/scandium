export type ListingCondition = "new" | "excellent" | "good" | "used";
export const listingCategories = [
  "skis",
  "snowboards",
  "boots",
  "bindings",
  "clothing",
  "protection",
] as const;

export type ListingCategory = (typeof listingCategories)[number];

export type Listing = {
  id: string;
  title: string;
  price: number;
  condition: ListingCondition;
  category: ListingCategory;
  year?: number;
  location: string;
  image_url?: string;
  created_at: string;
};
