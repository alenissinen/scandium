# Start all services
dev:
    tmux new-session -d -s scandium -n api 'cargo run -p api'
    tmux new-window -t scandium -n consumer 'cargo run -p consumer'
    tmux new-window -t scandium -n frontend 'cd frontend && npm run dev'
    tmux attach -t scandium

# Docker-compose
docker:
    docker-compose up -d

# Individual services
api:
    cargo run -p api

consumer:
    cargo run -p consumer

frontend:
    cd frontend && npm run dev

# Database
migrate:
    sqlx migrate run

seed token:
    ./scripts/seed_listings.sh {{token}}

# Kafka
topics:
    ./scripts/create_kafka_topics.sh

# Tests
test:
    SQLX_OFFLINE=true cargo test --workspace

# Setup new environment
setup:
    sqlx migrate run
    ./scripts/create_kafka_topics.sh