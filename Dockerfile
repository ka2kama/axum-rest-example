ARG RUST_VERSION
FROM rust:${RUST_VERSION} as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock rust-toolchain ./
RUN mkdir ./src && echo "fn main(){}" > ./src/main.rs
RUN cargo build --release && rm ./target/release/deps/bookshelf*
COPY src ./src
RUN cargo build --release

FROM debian:bullseye-slim
ARG APP=/usr/src/app
EXPOSE 9000
ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

WORKDIR ${APP}
RUN groupadd $APP_USER && useradd -g $APP_USER $APP_USER
RUN chown -R $APP_USER:$APP_USER ${APP}
COPY --from=builder /app/target/release/bookshelf ${APP}/bookshelf

USER $APP_USER
CMD ["./bookshelf"]