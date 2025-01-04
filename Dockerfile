FROM rust:slim AS builder

ARG APP_NAME
WORKDIR /app

# install toolchain
COPY rust-toolchain.toml ./
RUN rustup show

# pre-build only the dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir ./src && echo "fn main(){}" > ./src/main.rs
RUN cargo build --release
# remove application crates
# NOTE: hyphens in project names are replaced with underscores when they become crates.
RUN rm -rf ./src ./target/release/deps/$(echo ${APP_NAME} | tr '-' '_')*

# release build
COPY src ./src
RUN cargo build --release

# executable image
FROM gcr.io/distroless/cc
ARG APP_NAME
WORKDIR /app
COPY --from=builder /app/target/release/${APP_NAME} /app/${APP_NAME}

# NOTE: The executable name must be hardcoded because variables cannot be used within
#       the ENTRYPOINT instruction, and distroless images do not include a shell.
ENTRYPOINT ["/app/bookshelf"]
