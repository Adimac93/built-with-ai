FROM docker.io/debian:bookworm-slim

RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*

ENV IP=0.0.0.0
ENV PORT=8080

WORKDIR /app
COPY apps/frontend/dist/web/server /app/server
COPY apps/frontend/dist/web/public /app/public

CMD ["/app/server"]
