# chassis
Opinionated Rust microservice chassis. The frame your service bolts onto.

Built on [axum](https://github.com/tokio-rs/axum) with batteries included so you can focus on routes and business logic.

## What you get

- Config from env (`BIND_ADDR`, `JWT_SECRET`, `OTEL_*`, etc. — see [config.rs](crates/chassis-core/src/config.rs))
- JWT auth (HS256) with extractors
- Structured logging (`tracing`, JSON or pretty)
- OpenTelemetry/OTLP distributed tracing
- Prometheus metrics on a separate admin port
- `/livez` and `/readyz` health endpoints
- Request timeouts, body limits, compression, request IDs (`tower-http`)
- Graceful shutdown on `SIGINT`/`SIGTERM`

## Layout

- [crates/chassis-core](crates/chassis-core) — the framework
- [crates/chassis-server](crates/chassis-server) — thin binary that calls `chassis_core::run()`
- [deploy/](deploy/) — Helm chart wrapping [bjw-s app-template](https://bjw-s-labs.github.io/helm-charts/docs/app-template/)

## Run

```sh
JWT_SECRET=dev cargo run -p chassis-server
```

App listens on `:3000`, admin (metrics) on `:9090`.
