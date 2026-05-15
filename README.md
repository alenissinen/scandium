[![CI](https://github.com/alenissinen/scandium/actions/workflows/ci.yml/badge.svg)](https://github.com/alenissinen/scandium/actions/workflows/ci.yml)
[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](LICENSE)
# Scandium

Scandium (named after the chemical element) is a modern customer-to-customer marketplace for buying and selling winter sports equipment (skis, snowboards, boots, bindings, avalanche gear etc.) with powerful filters, real-time messaging and a modern UI combined with powerful backend.

## Tech stack

| Layer            | Technology                                   |
| ---------------- | -------------------------------------------- |
| Frontend         | Next.js, TypeScript, Tailwind CSS, shadcn/ui |
| Backend          | Rust, Axum, Tokio, SQLx                      |
| Database         | PostgreSQL                                   |
| Search           | Elasticsearch                                |
| Event streaming  | Kafka                                        |
| Cache / Sessions | Redis                                        |
| Infrastructure   | Docker Compose, Nginx, GitHub Actions        |

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

## Development setup

### Prerequisites

- Rust (latest stable)
- Docker + Docker Compose
- Resend.com api key

### Setup

```bash
docker-compose up -d
sqlx migrate run
```

### Run the project

```bash
# Start api
cargo run -p api

# Start frontend
cd frontend && npm run dev
```

### After pulling recent changes

If migrations have changed:

```bash
cargo sqlx migrate run
```

If queries have changed:

```bash
cargo sqlx prepare --workspace
```

### Before committing

```bash
# Make sure everything compiles
cargo check --workspace

# Lint
cargo clippy --workspace -- -D warnings

# Format
cargo fmt

# Lint and format frontend
cd frontend && npm run check

```

### Git hooks

Git hooks are managed via lefthook. Rust hooks (clippy, fmt) run automatically on commit. See more in [frontend README](frontend/README.md)

## License

GNU Affero General Public License v3  
See [LICENSE](LICENSE) for details.
