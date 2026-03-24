# Nova VNDB Server

> **This project is in early scratch phase.** Before touching anything, read the docs.

## For AI Agents

**Read `roles/` before generating any code.**

The `roles/` directory contains mandatory guidelines. The most important one is `roles/backend-code-guideline.md`, which defines the `kanau::Processor` pattern and three-layer architecture every backend crate must follow. Violating these patterns creates architectural inconsistencies that are hard to untangle.

## Architecture Overview

The system is built around three core constraints:

- **Cache All** — All public-facing data is pre-rendered into MongoDB view documents. The API gateway serves these views directly without hitting PostgreSQL.
- **Event-Driven** — State changes propagate through RabbitMQ. Services publish events; view handlers consume them and refresh the cache reactively.
- **Stateless** — The application processes own no state. Everything lives in the database or a shared store.

### Backend Components

| Component            | Role                                                                |
| -------------------- | ------------------------------------------------------------------- |
| User Request Handler | Handles authenticated mutations via `axum`                          |
| API Gateway          | Read-only public API via `poem` + `poem_openapi`, serves view cache |
| Event Handler        | Processes domain events from RabbitMQ                               |
| View Manager         | Refreshes MongoDB view documents in response to events              |
| Email Sender         | Sends email                                                         |
| Scheduler            | Pushes scheduled events to the message queue (single instance)      |

### External Tools

| Tool            | Role                                                         |
| --------------- | ------------------------------------------------------------ |
| `nova-vndb-cli` | Admin CLI (manual view refresh, etc.)                        |
| `vndb-migrator` | Migrates data from VNDB's MySQL to this project's PostgreSQL |

## Tech Stack

| Technology                                        | Purpose                                       |
| ------------------------------------------------- | --------------------------------------------- |
| Rust (pinned `1.94.0`, see `rust-toolchain.toml`) | Primary language                              |
| Tokio                                             | Async runtime                                 |
| [Kanau](https://github.com/suitsu31-club/kanau)   | Tower-like framework for event-driven systems |
| axum                                              | User request handler HTTP server              |
| poem + poem_openapi                               | API gateway HTTP server                       |
| PostgreSQL                                        | Single source of truth                        |
| Redis                                             | Session storage and hot cache                 |
| MongoDB                                           | View cache (pre-rendered documents)           |
| Meilisearch                                       | Full-text search                              |
| RabbitMQ                                          | Message queue                                 |

## Quick Start

```bash
cargo build
```
