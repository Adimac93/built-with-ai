FROM docker.io/rust:1-slim AS builder

WORKDIR /app

# Dependency-caching layer: build a dummy main against the real manifests so
# source edits don't invalidate the compiled dependency layer.
COPY apps/api/Cargo.toml apps/api/Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs \
  && cargo build --release \
  && rm -rf src target/release/api target/release/deps/api-*

COPY apps/api/src ./src
RUN cargo build --release

FROM docker.io/debian:bookworm-slim

RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/api /usr/local/bin/api

ENV PORT=3000
EXPOSE 3000

CMD ["api"]
