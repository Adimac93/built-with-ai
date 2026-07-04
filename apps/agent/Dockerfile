FROM docker.io/rust:1-slim AS builder

WORKDIR /app

# Dependency-caching layer: build a dummy main against the real manifests so
# source edits don't invalidate the compiled dependency layer.
COPY apps/agent/Cargo.toml apps/agent/Cargo.lock ./
COPY apps/agent/triz.json ./triz.json
RUN mkdir src && echo 'fn main() {}' > src/main.rs \
  && cargo build --release \
  && rm -rf src target/release/agent target/release/deps/agent-*

COPY apps/agent/src ./src
RUN cargo build --release

FROM docker.io/debian:bookworm-slim

RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/agent /usr/local/bin/agent

ENV PORT=8080
EXPOSE 8080

CMD ["agent"]
