CREATE TYPE listing_type AS ENUM (
    'skis',
    'snowboard',
    'boots',
    'bindings',
    'clothing',
    'protection'
);

CREATE TYPE listing_condition AS ENUM (
    'new',
    'excellent',
    'good',
    'used'
);

CREATE TABLE listings (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title       TEXT NOT NULL,
    description TEXT,
    price       INTEGER NOT NULL,
    listing_type    listing_type NOT NULL,
    condition   listing_condition NOT NULL,
    location    TEXT NOT NULL,
    is_active   BOOLEAN NOT NULL DEFAULT true,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_listings_user_id ON listings(user_id);
CREATE INDEX idx_listings_listing_type ON listings(listing_type);
CREATE INDEX idx_listings_condition ON listings(condition);
CREATE INDEX idx_listings_is_active ON listings(is_active);

-- Updated 'updated_at' field automatically
CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER listings_updated_at
    BEFORE UPDATE ON listings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();