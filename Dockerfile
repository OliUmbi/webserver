# ---- build stage ----
FROM rust:alpine3.23 AS builder
RUN apk add --no-cache musl-dev

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY crates ./crates
RUN cargo build --package webserver --release --target x86_64-unknown-linux-musl

# ---- runtime stage ----
FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/webserver /webserver
ENTRYPOINT ["/webserver"]
