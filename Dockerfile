FROM rust:1.50 as builder

ENV CARGO_HOME="/cargo"

RUN apt-get update \
 && apt-get install -y libpq-dev
 && cargo install diesel_cli --no-default-features --features "postgres"

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --bin server

FROM debian:buster-slim

WORKDIR /app

RUN apt-get update \
 && apt-get install -y libpq-dev \
 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /cargo/bin/diesel /usr/local/bin/diesel
COPY --from=builder /app/target/release/server /usr/local/bin/server
COPY docker/entrypoint.sh /usr/local/bin/entrypoint.sh
COPY migrations ./migrations

ENTRYPOINT ["entrypoint.sh"]

CMD ["server", "-m"]
