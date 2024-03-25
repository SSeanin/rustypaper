<p align="center">
    <img alt="Rusty Paper" src="./docs/logo.png" width="25%">
</p>

<h1 align="center">
The Rusty Paper
</h1>

<p align="center">
    A ready out of the box RESTful API for creating and sharing text content that you can host yourself, in Rust.
</p>

## Table of Content
- [Development Run](#development-run)
    - [Running Using Docker Compose](#running-using-docker-compose)
    - [Running Locally Using Cargo and SQLx CLI](#running-locally-using-cargo-and-sqlx-cli)
- [Production Run](#production-run)
- [Configuration](#configuration)
    - [`_FILE` Format](#_file-format)

## Development Run
You can either run the application locally in a docker container or run it using Cargo and [SQLx CLI](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli). Either way will be OK if you want to work in a development environment.

### Run Using Docker Compose
If you have Docker and [Docker Compose](https://docs.docker.com/compose/) installed, the process for a development run is straight forward.

Simply rename the `.env.example` file to `.env` and fill in the [environment variables](#configuration).

Docker Compose uses the database related configuration that you provide in `.env` to create a PostgreSQL instance. The application container then tries to connect to the database and set it up automatically.

### Run Using Cargo and SQLx CLI
Make sure you have [SQLx CLI](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli#install) installed and an instance of the [PostgreSQL](https://www.postgresql.org/download) database running.

Again rename the `.env.example` file to `.env` and fill in the [environment variables](#configuration).

You can then setup the database using SQLx

```bash
cargo sqlx database setup
```

then run using cargo

```bash
cargo run
```

or you can do a release run

```bash
cargo run --release
```

## Production Run
Running in a production environment needs some sort of secret delivery strategy. To keep things simple for a more casual use, a basic [`docker-compose.prod.yaml`](https://github.com/SSeanin/rustypaper/blob/main/docker-compose.prod.yaml) has been provided for usage with [Docker Swarm mode](https://docs.docker.com/engine/swarm/). You need to [provide you own secrets](https://docs.docker.com/compose/use-secrets/) using the [`_FILE` format of environment variables](#_file-format).

## Configuration
The app allows for some configuration mostly database related:

- `DATABASE_URL`: The standard database specific connection string, used by SQLx to setup and migrate the database. Something like
```dotenv
DATABASE_URL=postgres://user:password@host:port/database-name
```
should work for PostgreSQL.

- `RUSTYPAPER_DATABASE_USER`: PostgreSQL Database user for authentication.

- `RUSTYPAPER_DATABASE_HOST`: The host in which PostgreSQL instance is running.

- `RUSTYPAPER_DATABASE_PORT`: The app tries to connect to PostgreSQL on this port. If you are running the application using [Docker Compose](https://docs.docker.com/compose/) leave the port on [5432](https://github.com/docker-library/postgres/blob/ab6925051ca097d415816928a50c483ecc370c00/15/bullseye/Dockerfile#L225).

- `RUSTYPAPER_DATABASE_PASSWORD`: Password for the provided PostgreSQL user.

- `RUSTYPAPER_DATABASE_NAME`: Name of the app database.

- `RUSTYPAPER_AUTH_SHARED_SECRET_KEY`: This is the Base64 encoded secret that application uses to encrypt the shared secrets. It is recommended to use a key of length 512 bits (64 bytes) or more. If you are unsure of to how to generate this key, you can use [OpenSSL Toolkit](https://www.openssl.org/source/) to help you generate 64 random bytes and Base64 encode them:

```bash
openssl rand -base64 64 | tr -d '\n'
```

- `RUSTYPAPER_COOKIE_KEY`: The key to use with private cookies. This key too must be at lease 64 bytes long. You can generate the key in a similar way to generating `RUSTYPAPER_AUTH_SHARED_SECRET_KEY`.

### `_FILE` Format
Currently this is only supported by 
- `RUSTYPAPER_DATABASE_USER`
- `RUSTYPAPER_DATABASE_PASSWORD`
- `RUSTYPAPER_AUTH_SHARED_SECRET_KEY`
- `RUSTYPAPER_COOKIE_KEY` 

By appending a `_FILE` to the end of these environment variables, a file is expected to hold its value.