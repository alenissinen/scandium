# Scripts

Utility scripts for Scandium development.

## seed_listings.sh

Seeds the database and Elasticsearch with example listings.

### Prerequisities

- API running
- Valid access token (login first and snatch the token from cookies)
- `jq` (pretty json, optional)

### Usage

```bash

#Login first to get a token!

./scripts/seed_listings.sh <access_token>

# Custom API URL
API_URL=http://localhost:3001 ./scripts/seed_listings.sh <access_token>
```

## create_kafka_topics.sh

Creates required Kafka topics.

```bash

./scripts/create_kafka_topics.sh
```
