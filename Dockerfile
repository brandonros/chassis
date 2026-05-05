# syntax=docker/dockerfile:1

FROM lukemathwalker/cargo-chef:0.1.77-rust-1.94.1-trixie AS chef
WORKDIR /app

# Stage 1: figure out what dependencies we need
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2: build dependencies (this is the cached layer)
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Now copy the actual source and build the app
COPY . .
RUN cargo build --release --bin chassis-server

# Stage 3: minimal runtime image
FROM debian:trixie-slim AS runtime
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/* && \
    groupadd -g 1001 appgroup && \
    useradd -u 1001 -g appgroup -m -s /bin/bash appuser

WORKDIR /app
COPY --from=builder /app/target/release/chassis-server /usr/local/bin/chassis-server

USER appuser
EXPOSE 3000 9090
ENTRYPOINT ["/usr/local/bin/chassis-server"]