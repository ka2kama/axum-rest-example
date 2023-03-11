# axum-rest-example

## Setup Local Environment

### Install tools

- [rustup](https://www.rust-lang.org/ja/tools/install)
- [npm (for install formatters locally)](https://nodejs.org/ja/download/)

### Execute only the first time

```shell
# install cargo-make (https://github.com/sagiegurari/cargo-make)
cargo install --force cargo-make

makers install-tools
```

## Run Locally

```shell
# run and initialize dynamodb-local if not yet
makers setup-local

# run application with hot reload
makers run
```

## Release Build (not working yet)

```shell
makers publish-local
docker container run -p 9000:9000 --env-file .env bookshelf
```
