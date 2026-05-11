export const listingCategories = [
  "skis",
  "snowboards",
  "boots",
  "bindings",
  "clothing",
  "protection",
] as const;

export const listingConditions = ["new", "excellent", "good", "used"] as const;

export type ListingCategory = (typeof listingCategories)[number];

export type ListingCondition = (typeof listingConditions)[number];

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
