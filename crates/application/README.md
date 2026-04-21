# Application

Use cases that combine domain logic and infrastructure.

## Responsibilities

- One file per use case (e.g. create_listing.rs)
- Calls domain services and infrastructure repositories

## This crate doesn't do the following

- No HTTP calls (no Axum types)
- No raw SQL

## Modules

| Module      | Description                        |
| ----------- | ---------------------------------- |
| `listing`   | CreateListing, UpdateListing, etc. |
| `user`      | RegisterUser, LoginUser, etc.      |
| `search`    | SearchListings, GetFilters, etc.   |
| `messaging` | SendMessage, GetConversation, etc. |
