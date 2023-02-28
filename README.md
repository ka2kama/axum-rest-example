# axum-rest-example

## Prerequisites

### Install tools

- [rustup](https://www.rust-lang.org/ja/tools/install)
- [cargo-make (task runner with a simple toml based configuration file)](https://github.com/sagiegurari/cargo-make#installation)
- [taplo (rich TOML toolkit)](https://taplo.tamasfe.dev/cli/installation/cargo.html)
- [dprint (code formatter for json, markdown, Dockerfile, etc.)](https://dprint.dev/install/)

### Execute only the first time

```shell
cp .env-template .env
```

## Run Locally

```shell
# run and initialize dynamodb-local if not yet
makers setup-dynamodb-local

# run application with hot reload
makers run
```

## Release Build (not working yet)

```shell
docker build --build-arg RUST_VERSION=$(cat ./rust-toolchain) . -t bookshelf --progress=plain
docker container run -p 9000:9000 --env-file .env bookshelf
```
