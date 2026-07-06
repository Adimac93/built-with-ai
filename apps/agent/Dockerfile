FROM docker.io/rust:1-slim AS builder

WORKDIR /app

# Workspace manifests first — dependency-caching layer builds dummy mains so
# source edits don't invalidate the compiled dependency layer. All member
# manifests are needed for the workspace to parse.
COPY Cargo.toml Cargo.lock ./
COPY apps/agent/Cargo.toml apps/agent/
COPY apps/frontend/Cargo.toml apps/frontend/
COPY apps/agent/triz.json apps/agent/triz.json
RUN mkdir -p apps/agent/src apps/frontend/src \
  && echo 'fn main() {}' > apps/agent/src/main.rs \
  && echo 'fn main() {}' > apps/frontend/src/main.rs \
  && cargo build --release -p agent \
  && rm -rf apps/agent/src target/release/agent target/release/deps/agent-*

COPY apps/agent/src apps/agent/src
RUN cargo build --release -p agent

FROM docker.io/debian:bookworm-slim

RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/agent /usr/local/bin/agent

ENV PORT=8080
EXPOSE 8080

CMD ["agent"]
