FROM rust:1.50-alpine as builder

RUN apk add --no-cache musl-dev postgresql-dev

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --bin server

FROM alpine:3

RUN apk add --no-cache libpq

COPY --from=builder /app/target/release/server /usr/local/bin/server

CMD ["server"]
