# ---- build stage ----
FROM rust:1.93.0-alpine3.23 AS builder

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates/* ./crates/*

RUN cargo build --release --package webserver --target x86_64-unknown-linux-musl

# ---- runtime stage ----
FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/webserver /webserver
ENTRYPOINT ["/webserver"]
