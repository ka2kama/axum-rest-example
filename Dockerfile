ARG RUST_VERSION
FROM rust:${RUST_VERSION} AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock rust-toolchain ./
RUN mkdir ./src && echo "fn main(){}" > ./src/main.rs
RUN cargo build --release && rm ./target/release/deps/bookshelf*
COPY src ./src
RUN cargo build --release

FROM gcr.io/distroless/cc
WORKDIR /app
EXPOSE 9000
COPY --from=builder /app/target/release/bookshelf /app/bookshelf

ENTRYPOINT ["/app/bookshelf"]
