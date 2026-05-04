# Domain

Contains all business logic, entites and repository traits.

## Responsibilities

- Define core entities: Listing, User, Message, Conversation
- Define repository traits that infrastructure implements
- Business rules and domain errors

## This crate doesn't do the following

- No database calls
- No HTTP logic
- No external service dependencies

## Modules

| Module      | Description                                         |
| ----------- | --------------------------------------------------- |
| `auth`      | Password reset token entity                         |
| `listing`   | Gear listing entity, status transitions, validation |
| `user`      | User entity, profile                                |
| `search`    | Search query types and filter definitions           |
| `messaging` | Conversation and message entities                   |
