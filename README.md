[![CI](https://github.com/alenissinen/scandium/actions/workflows/ci.yml/badge.svg)](https://github.com/alenissinen/scandium/actions/workflows/ci.yml)
[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](LICENSE)

# Scandium

Scandium (named after the chemical element) is a modern customer-to-customer marketplace for buying and selling winter sports equipment (skis, snowboards, boots, bindings, avalanche gear etc.) with powerful filters, real-time messaging and a modern UI combined with scalable backend.

## Tech stack

| Layer            | Technology                                   |
| ---------------- | -------------------------------------------- |
| Frontend         | Next.js, TypeScript, Tailwind CSS, shadcn/ui |
| Backend          | Rust, Axum, Tokio, SQLx                      |
| Database         | PostgreSQL                                   |
| Search           | Elasticsearch                                |
| Event streaming  | Kafka                                        |
| Cache / Sessions | Redis                                        |
| Infrastructure   | Docker, Nix, GitHub Actions                  |

## Workspace Structure

| Path                    | Description                                     |
| ----------------------- | ----------------------------------------------- |
| `apps/api`              | Axum HTTP server -> routing and middleware only |
| `apps/consumer`         | Kafka consumer -> syncs events to Elasticsearch |
| `crates/domain`         | Business logic, entities, repository traits     |
| `crates/infrastructure` | Implementations: PostgreSQL, ES, Kafka, Redis   |
| `crates/application`    | Use cases -> combines domain and infrastructure |
| `crates/shared`         | Shared types, errors, pagination                |
| `frontend/`             | Next.js frontend                                |
| `scripts`               | Utility scripts for development                 |
| `migrations`            | SQLx migrations                                 |

## Development setup

### Prerequisites

- [Nix](https://determinate.systems/posts/determinate-nix-installer) (recommended)
- Docker + Docker Compose
- Resend API key (for password reset emails)

### Setup with Nix

```bash
nix develop
just setup # starts docker, runs migrations, creates kafka topics
just dev # starts api, consumer and frontend in a tmux session
```

### Setup without Nix

```bash
docker-compose up -d
sqlx migrate run
./scripts/create_kafka_topics.sh
cargo run -p api
cd frontend && npm run dev
```

## Available Commands

| Command             | Description                         |
| ------------------- | ----------------------------------- |
| `just dev`          | Start all services in tmux          |
| `just api`          | Start API                           |
| `just consumer`     | Start Kafka consumer                |
| `just frontend`     | Start frontend                      |
| `just setup`        | Setup new environment               |
| `just migrate`      | Run database migrations             |
| `just topics`       | Create Kafka topics                 |
| `just test`         | Run all tests                       |
| `just seed <token>` | Seed database with example listings |

## Docker Images

Images are published to GitHub Container Registry on every merge to `main`

```bash
docker pull ghcr.io/alenissinen/scandium-api:latest
docker pull ghcr.io/alenissinen/scandium-consumer:latest
docker pull ghcr.io/alenissinen/scandium-frontend:latest
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.

## License

GNU Affero General Public License v3  
See [LICENSE](LICENSE) for details.
