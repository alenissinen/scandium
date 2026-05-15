# Start dev environment in tmux
dev:
    tmux new-session -d -s scandium -n api 'cargo run -p api'
    tmux new-window -t scandium -n consumer 'cargo run -p consumer'
    tmux new-window -t scandium -n frontend 'cd frontend && npm run dev -- --port 3000'
    tmux attach -t scandium

# Start prod environment
prod:
    docker-compose -f docker-compose.prod.yml up -d

# Build images
build-images:
    docker build -f docker/rust.Dockerfile --target api -t scandium-api:local .
    docker build -f docker/rust.Dockerfile --target consumer -t scandium-consumer:local .
    docker build -f docker/frontend.Dockerfile -t scandium-frontend:local .

# Docker-compose
docker:
    docker-compose up -d

# Individual services
api:
    cargo run -p api

consumer:
    cargo run -p consumer

frontend:
    cd frontend && npm run dev -- --port 3000

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