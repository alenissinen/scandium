#!/bin/bash

set -e

TOKEN=${1:-""}
API_URL=${API_URL:-"http://localhost:3001"}

if [ -z "$TOKEN" ]; then
  echo "Usage: ./scripts/seed_listings.sh <access_token>"
  exit 1
fi

create_listing() {
  local title=$1
  local price=$2
  local listing_type=$3
  local condition=$4
  local location=$5
  local description=$6

  curl -s -X POST "$API_URL/api/v1/listings" \
    -H "Content-Type: application/json" \
    -H "Cookie: access_token=$TOKEN" \
    -d "{
      \"title\": \"$title\",
      \"price\": $price,
      \"listing_type\": \"$listing_type\",
      \"condition\": \"$condition\",
      \"location\": \"$location\",
      \"description\": \"$description\"
    }" | jq '.id' 2>/dev/null || echo "Created"

  echo "Created: $title"
}

echo "Seeding listings..."

# Skis
create_listing "Atomic Redster X9 170cm" 485 "skis" "new" "Helsinki" "Top condition racing skis"
create_listing "Völkl Racetiger SL 165cm" 390 "skis" "excellent" "Tampere" "Lightly used slalom skis"
create_listing "Rossignol Experience 80 168cm" 220 "skis" "good" "Turku" "Great all-mountain skis"
create_listing "Head Supershape e-Speed 175cm" 540 "skis" "new" "Espoo" "Brand new racing skis"
create_listing "K2 Mindbender 116 185cm" 180 "skis" "used" "Rovaniemi" "Wide powder skis"
create_listing "Blizzard Bonafide 97 180cm" 430 "skis" "excellent" "Oulu" "Excellent all-mountain skis"
create_listing "Fischer RC4 Worldcup SC 160cm" 610 "skis" "new" "Helsinki" "Professional racing skis"
create_listing "Nordica Enforcer 100 180cm" 350 "skis" "excellent" "Jyväskylä" "Versatile all-mountain skis"

# Snowboards
create_listing "Burton Custom 156cm" 290 "snowboard" "excellent" "Tampere" "Classic board in great condition"
create_listing "Lib Tech Skate Banana 154cm" 260 "snowboard" "good" "Helsinki" "Rocker board"
create_listing "Capita DOA 154cm" 280 "snowboard" "excellent" "Lahti" "Park board"
create_listing "Never Summer Proto Synthesis 156cm" 370 "snowboard" "excellent" "Rovaniemi" "Hybrid camber"
create_listing "Jones Mountain Twin 158cm" 410 "snowboard" "new" "Rovaniemi" "Brand new mountain board"
create_listing "Rome Mod Rocker 158cm" 195 "snowboard" "good" "Oulu" "All-mountain rocker"
create_listing "GNU Carbon Credit 156cm" 360 "snowboard" "excellent" "Helsinki" "High performance board"
create_listing "Bataleon Evil Twin 155cm" 330 "snowboard" "excellent" "Espoo" "3BT technology"

# Boots
create_listing "Salomon X Pro 120 26.5" 95 "boots" "good" "Oulu" "Stiff racing boots"
create_listing "Nordica Speedmachine 100 27.5" 140 "boots" "good" "Tampere" "Comfortable all-mountain boots"
create_listing "Tecnica Mach1 MV 120 27.0" 165 "boots" "good" "Kuopio" "Medium width racing boots"
create_listing "Lange RX 130 LV 26.5" 210 "boots" "excellent" "Espoo" "Narrow width racing boots"
create_listing "Atomic Hawx Ultra 130 27.0" 195 "boots" "excellent" "Helsinki" "Ultra stiff boots"
create_listing "Scarpa Maestrale RS 26.0" 385 "boots" "excellent" "Rovaniemi" "Touring boots"
create_listing "Dalbello Panterra 120 27.5" 175 "boots" "good" "Lahti" "All-mountain boots"
create_listing "Full Tilt First Chair 120 27.0" 135 "boots" "used" "Turku" "Used racing boots"

# Bindings
create_listing "Union Force" 120 "bindings" "excellent" "Espoo" "Universal snowboard bindings"
create_listing "Marker Griffon 13" 175 "bindings" "good" "Tampere" "Freeride bindings"
create_listing "Look Pivot 18" 145 "bindings" "excellent" "Tampere" "Racing bindings"
create_listing "Tyrolia Attack2 13" 110 "bindings" "used" "Oulu" "All-mountain bindings"
create_listing "Atomic Shift MNC 13" 285 "bindings" "excellent" "Jyväskylä" "Touring bindings"
create_listing "Fritschi Tecton 12" 220 "bindings" "excellent" "Kuopio" "Touring bindings"
create_listing "Marker Duke PT 16" 195 "bindings" "excellent" "Espoo" "Pin touring bindings"
create_listing "Plum Guide 145" 340 "bindings" "excellent" "Rovaniemi" "Ultralight touring bindings"

echo ""
echo "Done! ES listing count:"
curl http://localhost:9200/listings/_count