export const listingCategories = [
  "skis",
  "snowboard",
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
  listing_type: ListingCategory;
  location: string;
  year?: number;
  image_url?: string;
  created_at: string;
};
