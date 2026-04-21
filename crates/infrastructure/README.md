# Infarstructure

Implements the repository traits defined in domain using external services.

## Responsibilities

- PostgreSQL repositories via SQLx
- Elasticsearch indexing and querying
- Kafka producer and consumer logic
- Redis cache and session store

## This crate doesn't do the following

- No business logic, only data access
- Doesn't define any domain entities

## Modules

| Module          | Description                     |
| --------------- | ------------------------------- |
| `postgres`      | SQLx repository implementations |
| `elasticsearch` | ES index management and search  |
| `kafka`         | Producer/consumer wrappers      |
| `redis`         | Cache, sessions, rate limiting  |
