FROM docker.io/rust:1-bookworm AS chef

WORKDIR /app

RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates cmake clang pkg-config \
  && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown \
  && cargo install cargo-chef --locked

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS cacher
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --locked --package frontend --target wasm32-unknown-unknown --recipe-path recipe.json
RUN cargo chef cook --release --locked --package frontend --no-default-features --features server --recipe-path recipe.json

FROM chef AS builder
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo install dioxus-cli --version 0.7.9 --locked

COPY . .
COPY --from=cacher /app/target target
RUN dx build --release --fullstack --force-sequential

FROM docker.io/debian:bookworm-slim

RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/* \
  && useradd --system --uid 10001 --home-dir /nonexistent --shell /usr/sbin/nologin app

ENV IP=0.0.0.0
ENV PORT=8080

WORKDIR /app
COPY --from=builder /app/target/dx/frontend/release/web/server /app/server
COPY --from=builder /app/target/dx/frontend/release/web/public /app/public

USER app
ENTRYPOINT ["/app/server"]
