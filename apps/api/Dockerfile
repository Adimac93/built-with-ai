FROM docker.io/rust:1-slim AS builder

WORKDIR /app

# Workspace manifests first — dependency-caching layer builds dummy mains so
# source edits don't invalidate the compiled dependency layer. All member
# manifests are needed for the workspace to parse.
COPY Cargo.toml Cargo.lock ./
COPY apps/api/Cargo.toml apps/api/
COPY apps/agent/Cargo.toml apps/agent/
COPY apps/frontend/Cargo.toml apps/frontend/
RUN mkdir -p apps/api/src apps/agent/src apps/frontend/src \
  && echo 'fn main() {}' > apps/api/src/main.rs \
  && echo 'fn main() {}' > apps/agent/src/main.rs \
  && echo 'fn main() {}' > apps/frontend/src/main.rs \
  && cargo build --release -p api \
  && rm -rf apps/api/src target/release/api target/release/deps/api-*

COPY apps/api/src apps/api/src
RUN cargo build --release -p api

FROM docker.io/debian:bookworm-slim

RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/api /usr/local/bin/api

ENV PORT=3000
EXPOSE 3000

CMD ["api"]
