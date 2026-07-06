FROM docker.io/rust:1.96.1-bookworm AS chef

WORKDIR /app

RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates cmake clang pkg-config \
  && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-chef --locked

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS cacher
COPY rust-toolchain.toml rust-toolchain.toml
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --locked --package agent --recipe-path recipe.json

FROM chef AS builder
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --locked --package agent --bin agent

FROM docker.io/debian:bookworm-slim

RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/* \
  && useradd --system --uid 10001 --home-dir /nonexistent --shell /usr/sbin/nologin app

COPY --from=builder /app/target/release/agent /usr/local/bin/agent

ENV PORT=8080
EXPOSE 8080

USER app
ENTRYPOINT ["/usr/local/bin/agent"]
