# Application

Use cases that combine domain logic and infrastructure.

## Responsibilities

- One file per use case (e.g. create_listing.rs)
- Calls domain services and infrastructure repositories

## This crate doesn't do the following

- No HTTP calls (no Axum types)
- No raw SQL

## Modules

| Module      | Description                    |
| ----------- | ------------------------------ |
| `listing`   | Listing related methods        |
| `user`      | User related methods           |
| `search`    | Listing search related methods |
| `messaging` | Chat related methods           |
| `auth`      | Auth related methods           |
