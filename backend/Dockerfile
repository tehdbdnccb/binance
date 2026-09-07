# Build Stage
FROM rust:1.88-slim-bookworm as builder

WORKDIR /app
COPY . .

# Install pkg-config and openssl for SQLx and Request clients
RUN apt-get update && apt-get install -y pkg-config libssl-dev

RUN cargo build --release

# Runtime Stage
FROM debian:bookworm-slim

WORKDIR /app

# Install root certs for Binance API requests
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/sentinel-backend /usr/local/bin/
COPY --from=builder /app/migrations ./migrations

EXPOSE 8080

CMD ["sentinel-backend"]

