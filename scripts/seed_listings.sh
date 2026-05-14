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
  local year=$7

  curl -s -X POST "$API_URL/api/v1/listings" \
    -H "Content-Type: application/json" \
    -H "Cookie: access_token=$TOKEN" \
    -d "{
      \"title\": \"$title\",
      \"price\": $price,
      \"listing_type\": \"$listing_type\",
      \"condition\": \"$condition\",
      \"location\": \"$location\",
      \"description\": \"$description\",
      \"year\": $year
    }" | jq '.id' 2>/dev/null || echo "Created"

  echo "Created: $title"
}

echo "Seeding listings..."

# Skis
create_listing "Atomic Redster X9 170cm" 485 "skis" "new" "Helsinki" "Top condition racing skis" 2024
create_listing "Völkl Racetiger SL 165cm" 390 "skis" "excellent" "Tampere" "Lightly used slalom skis" 2022
create_listing "Rossignol Experience 80 168cm" 220 "skis" "good" "Turku" "Great all-mountain skis" 2021
create_listing "Head Supershape e-Speed 175cm" 540 "skis" "new" "Espoo" "Brand new racing skis" 2025
create_listing "K2 Mindbender 116 185cm" 180 "skis" "used" "Rovaniemi" "Wide powder skis" 2020
create_listing "Blizzard Bonafide 97 180cm" 430 "skis" "excellent" "Oulu" "Excellent all-mountain skis" 2022
create_listing "Fischer RC4 Worldcup SC 160cm" 610 "skis" "new" "Helsinki" "Professional racing skis" 2018
create_listing "Nordica Enforcer 100 180cm" 350 "skis" "excellent" "Jyväskylä" "Versatile all-mountain skis" 2010

# Snowboards
create_listing "Burton Custom 156cm" 290 "snowboard" "excellent" "Tampere" "Classic board in great condition" 2020
create_listing "Lib Tech Skate Banana 154cm" 260 "snowboard" "good" "Helsinki" "Rocker board" 2022
create_listing "Capita DOA 154cm" 280 "snowboard" "excellent" "Lahti" "Park board" 2025
create_listing "Never Summer Proto Synthesis 156cm" 370 "snowboard" "excellent" "Rovaniemi" "Hybrid camber" 2028
create_listing "Jones Mountain Twin 158cm" 410 "snowboard" "new" "Rovaniemi" "Brand new mountain board" 2023
create_listing "Rome Mod Rocker 158cm" 195 "snowboard" "good" "Oulu" "All-mountain rocker" 2021
create_listing "GNU Carbon Credit 156cm" 360 "snowboard" "excellent" "Helsinki" "High performance board" 2022
create_listing "Bataleon Evil Twin 155cm" 330 "snowboard" "excellent" "Espoo" "3BT technology" 2020

# Boots
create_listing "Salomon X Pro 120 26.5" 95 "boots" "good" "Oulu" "Stiff racing boots" 2026
create_listing "Nordica Speedmachine 100 27.5" 140 "boots" "good" "Tampere" "Comfortable all-mountain boots" 2026
create_listing "Tecnica Mach1 MV 120 27.0" 165 "boots" "good" "Kuopio" "Medium width racing boots" 2026
create_listing "Lange RX 130 LV 26.5" 210 "boots" "excellent" "Espoo" "Narrow width racing boots" 2022
create_listing "Atomic Hawx Ultra 130 27.0" 195 "boots" "excellent" "Helsinki" "Ultra stiff boots" 2023
create_listing "Scarpa Maestrale RS 26.0" 385 "boots" "excellent" "Rovaniemi" "Touring boots" 2021
create_listing "Dalbello Panterra 120 27.5" 175 "boots" "good" "Lahti" "All-mountain boots" 2024
create_listing "Full Tilt First Chair 120 27.0" 135 "boots" "used" "Turku" "Used racing boots" 2023

# Bindings
create_listing "Union Force" 120 "bindings" "excellent" "Espoo" "Universal snowboard bindings" 2018
create_listing "Marker Griffon 13" 175 "bindings" "good" "Tampere" "Freeride bindings" 2016
create_listing "Look Pivot 18" 145 "bindings" "excellent" "Tampere" "Racing bindings" 2024
create_listing "Tyrolia Attack2 13" 110 "bindings" "used" "Oulu" "All-mountain bindings" 2023
create_listing "Atomic Shift MNC 13" 285 "bindings" "excellent" "Jyväskylä" "Touring bindings" 2022
create_listing "Fritschi Tecton 12" 220 "bindings" "excellent" "Kuopio" "Touring bindings" 2021
create_listing "Marker Duke PT 16" 195 "bindings" "excellent" "Espoo" "Pin touring bindings" 2025
create_listing "Plum Guide 145" 340 "bindings" "excellent" "Rovaniemi" "Ultralight touring bindings" 2026

echo ""
echo "Done! ES listing count:"
curl http://localhost:9200/listings/_count