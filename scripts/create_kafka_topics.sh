#!/bin/bash

set -e

KAFKA_CONTAINER=${KAFKA_CONTAINER:-"scandium_kafka_1"}
BOOTSTRAP_SERVER=${BOOTSTRAP_SERVER:-"localhost:9092"}

create_topic() {
  local topic=$1
  local partitions=${2:-1}
  local replication=${3:-1}

  echo "Creating topic: $topic"
  docker exec "$KAFKA_CONTAINER" kafka-topics \
    --create \
    --if-not-exists \
    --topic "$topic" \
    --bootstrap-server "$BOOTSTRAP_SERVER" \
    --partitions "$partitions" \
    --replication-factor "$replication"
}

echo "Creating Kafka topics..."

create_topic "listing.created"

echo ""
echo "Done! Current topics:"
docker exec "$KAFKA_CONTAINER" kafka-topics --list --bootstrap-server "$BOOTSTRAP_SERVER"