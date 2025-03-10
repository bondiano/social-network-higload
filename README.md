# Social Network

Social Network - social network service for Otus High Load course.

## Requirements

- Cargo
- Docker

## Project Structure

### Installing `sqlx-cli`

SQLx provides a command-line tool for creating and managing databases as well as migrations. It is published
on the Cargo crates registry as `sqlx-cli` and can be installed like so:

```shell
cargo install sqlx-cli --features postgres
```

### Setting Up the Application Database

With `sqlx-cli` installed and your `.env` file set up, you only need to run the following command to get the
Postgres database ready for use:

```
sqlx db setup
```

### Starting the Application

With everything else set up, all you should have to do at this point is:

```
make build
make run
```

## Contributing

- please run [.pre-commit.sh](./.pre-commit.sh) before sending a PR, it will check everything
