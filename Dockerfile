FROM rust:1.76-bullseye AS planner

WORKDIR /usr/src/app

RUN cargo install cargo-chef

COPY . .

RUN cargo chef prepare --recipe-path recipe.json


FROM rust:1.76-bullseye AS casher

WORKDIR /usr/src/app

RUN cargo install cargo-chef

COPY --from=planner /usr/src/app/recipe.json recipe.json

# Caching the dependencies
RUN cargo chef cook --release --recipe-path recipe.json

# Installing SQLx CLI for database setup
RUN cargo install --version ^0.7 sqlx-cli --no-default-features --features native-tls,postgres


FROM rust:1.76-bullseye AS builder

WORKDIR /usr/src/app

# Copy over the cached dependencies
COPY --from=casher /usr/src/app/target target
COPY --from=casher /usr/local/cargo /usr/local/cargo

COPY . .

ENV SQLX_OFFLINE true
RUN cargo build --release


FROM debian:bullseye-slim AS runtime

WORKDIR /usr/src/app

COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/cargo/bin/sqlx
COPY --from=builder /usr/src/app/target/release/bootstrap axum-backend

ENV CARGO_HOME=/usr/local/cargo
COPY db-setup.sh db-setup.sh
COPY migrations migrations
RUN chmod +x db-setup.sh

ENTRYPOINT [ "sh", "-c", "./db-setup.sh && ./axum-backend" ]