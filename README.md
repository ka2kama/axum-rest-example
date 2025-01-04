# axum-rest-example

## Setup Local Environment

### Requirement tools

- [rust (rustup & cargo)](https://www.rust-lang.org/ja/tools/install)
- [pnpm (for install formatters locally)](https://pnpm.io/installation)
- [docker](https://docs.docker.com)

### Execute only the first time

```shell
./scripts/initial-setup.sh
```

## Run Locally

```shell
# run and initialize dynamodb-local if not yet
./makers setup-local

# run application with hot reload
./makers run
```

## Release Build (not working yet)

```shell
./makers publish-local
docker container run -p 9000:9000 --env-file .env bookshelf
```
