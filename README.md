# axum-rest-example

## Setup Local Environment

### Install tools

- [rustup](https://www.rust-lang.org/ja/tools/install)
- [npm (for install formatters locally)](https://nodejs.org/ja/download/)

### Execute only the first time

```shell
# install cargo-make (https://github.com/sagiegurari/cargo-make)
cargo install --force cargo-make

makers setup-local
```

## Run Locally

```shell
# run and initialize dynamodb-local if not yet
makers setup-db

# run application with hot reload
makers run
```

## Release Build (not working yet)

```shell
docker build --build-arg RUST_VERSION=$(cat ./rust-toolchain) . -t bookshelf --progress=plain
docker container run -p 9000:9000 --env-file .env bookshelf
```
