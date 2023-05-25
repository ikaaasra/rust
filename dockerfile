FROM rust:latest as builder

RUN USER=root cargo new --bin rust
WORKDIR ./rust

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

ADD . .

RUN rm ./target/release/deps/rust*
RUN cargo build --release


FROM debian:bullseye-slim

RUN apt-get update \
    && apt-get install -y \
    && rm -rf /var/lib/apt/lists/*

WORKDIR ./app

COPY --from=builder /app/target/release/rust .

CMD ["rust"]