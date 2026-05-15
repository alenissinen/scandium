# Contributing to Scandium

## Development Environment

[Nix](https://determinate.systems/posts/determinate-nix-installer) is recommended for development, it gives you the same environment as CI.

```bash
nix develop
```

## Architecture

Scandium follows hexagonal architecture

```
apps/api -> HTTP layer (routes, middleware, DI)
apps/consumer -> Kafka consumer (event processing)
crates/application -> Use cases (combine domain + infrastructure)
crates/domain -> Business logic, entities, traits
crates/infrastructure -> Implementations: PostgreSQL, ES, Kafka, Redis, Jwt, Resend
crates/shared -> Code that is used in multiple places
```

**Dependency rules:**

`domain` has no external dependencies. `infrastructure` implements domain traits. `application` calls domain traits.

## Commit Guidelines

Please commit too often rather than too rarely.

Use [conventional commit messages](https://www.conventionalcommits.org/):

```
feat(api): add GET /listings endpoint
fix(consumer): use serde alias for listing_id deserialization
chore(docker): add cargo-chef dockerfile
test(application): add create listing use case tests
```

## Before Committing

Git hooks run automatically via lefthook on every commit that has matching files:

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- Frontend Biome check

Run manually:

```bash
# Rust
cargo check --workspace
cargo clippy --workspace -- -D warnings
cargo fmt

# Frontend
cd frontend && npm run check
```

## Adding a New Feature

1. Create a branch: `git checkout -b feat/my-feature`
2. Follow the layer order:
   - Migration (if DB change needed)
   - Domain entity + trait
   - Infrastructure implementation
   - Application use case
   - API route + state
   - Frontend
3. Write tests for the use case
4. Commit each layer separately
5. Open a PR

## Database Migrations

```bash
sqlx migrate add <name> # create migration
sqlx migrate run # apply migrations
cargo sqlx prepare --workspace # update offline query cache (commit .sqlx/)
```

## After Pulling Changes

```bash
sqlx migrate run # if migrations changed
cargo sqlx prepare --workspace # if queries changed
```
