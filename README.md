> [!IMPORTANT]
> This project is my personal portfolio project, AI (and agents) will be used but I won't be blindly generating code with them. I've learned to code because I like to build things and **write** code! This is also great exercise on building a scalable modern web application.
> 
> If you want to contribute something to this project, **please don't** create pull requests with completely generated code, it might be a lot more efficient and better looking, but I just feel like it doesn't belong in a portfolio project which I intend to show to my possible future employers. Thank you for understanding.

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

### Prerequisites

- Rust (latest stable)
- Docker + Docker Compose
- Node.js 24+

### Run infrastructure

```bash
docker-compose up -d
```

### Run backend

```bash
cargo run -p api
```

### Run frontend

```bash
cd frontend && npm run dev
```

## License

GNU Affero General Public License v3  
See [LICENSE](LICENSE) for details.
