# Clean Rust

### Реализация чистой архитектуры на языке программирования Rust

## Структура проекта:

```
.
├── Cargo.toml
├── docker-compose.yaml
├── docs
│   └── README.md
├── README.md
├── sql
│   └── schema.sql
├── src
│   ├── aplication
│   │   ├── mod.rs
│   │   ├── services
│   │   │   └── mod.rs
│   │   └── usecases
│   │       └── mod.rs
│   ├── domain
│   │   ├── aggregates
│   │   │   └── mod.rs
│   │   ├── entities
│   │   │   └── mod.rs
│   │   ├── mod.rs
│   │   ├── repositories
│   │   │   └── mod.rs
│   │   └── value_objects
│   │       └── mod.rs
│   ├── infrastructure
│   │   ├── data_access
│   │   │   └── mod.rs
│   │   ├── external_services
│   │   │   └── mod.rs
│   │   ├── mod.rs
│   │   └── repositories
│   │       └── mod.rs
│   ├── interfaces
│   │   ├── cli
│   │   │   └── mod.rs
│   │   ├── http
│   │   │   ├── controllers
│   │   │   │   └── mod.rs
│   │   │   ├── middleware
│   │   │   │   └── mod.rs
│   │   │   ├── mod.rs
│   │   │   ├── routes
│   │   │   │   └── mod.rs
│   │   │   └── views
│   │   │       └── mod.rs
│   │   └── mod.rs
│   ├── lib.rs
│   └── main.rs
└── tests
    └── integration_tests.rs
```
