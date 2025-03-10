FROM lukemathwalker/cargo-chef:latest-rust-1.85-slim-bookworm AS chef
WORKDIR /social_network

FROM --platform=$BUILDPLATFORM chef AS planner

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

RUN apt-get update -y && \
    apt-get install -y pkg-config make g++ libssl-dev curl && \
    rustup target add x86_64-unknown-linux-gnu

COPY --from=planner /social_network/recipe.json recipe.json

# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
COPY ./.sqlx ./.sqlx
COPY ./migrations ./migrations

# Build our project
RUN cargo build --release

FROM --platform=$BUILDPLATFORM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update && apt-get install -y libssl-dev ca-certificates

COPY --from=builder /social_network/target/release/social_network /usr/local/bin/social_network

EXPOSE 4238/tcp
CMD social_network
