# Build Stage
FROM rust:1.68.2 AS builder
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new producer
WORKDIR /usr/src/producer
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Bundle Stage
FROM scratch
COPY --from=builder /usr/local/cargo/bin/producer .
USER 1000
CMD ["./producer"]

