# chronicle

[![Join the chat at https://gitter.im/chronicle-framework/Lobby](https://badges.gitter.im/chronicle-framework/Lobby.svg)](https://gitter.im/chronicle-framework/Lobby?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

_WARNING: this project is a work in progress and is nowhere near ready for production!_

An event sourced CQRS framework for Rust.

There are a number of crates in this repository:

- `chronicle`: Common traits for event stores, snapshot stores, and projections
- `chronicle_domain`: Async command processing and aggregate trait
- `chronicle_memory`: In-memory implementation of `chronicle` APIs
- `chronicle_postgres`: Postgres implementation of `chronicle` APIs

## Why Rust?

- Predictable performance with minimal overhead
- Fast, zero-cost async-io abstractions (futures, streams, etc.) via [Tokio](https://tokio.rs/)
- Powerful, modern type system is very for domain modelling
- Deployment should be simplified through the static compilation of a single binary
