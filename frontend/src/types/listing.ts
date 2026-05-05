export type ListingCondition = "new" | "excellent" | "good" | "used";
export type ListingCategory =
  | "skis"
  | "snowboards"
  | "boots"
  | "bindings"
  | "clothing"
  | "protection";

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
