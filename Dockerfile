FROM rust:1.50 as builder

RUN apt-get update && apt-get install -y libpq-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --bin server

FROM debian:buster-slim

RUN apt-get update \
 && apt-get install -y libpq-dev \
 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/server /usr/local/bin/server

CMD ["server"]
