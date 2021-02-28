# Server

This is the thermit server.

## Development

The server can be build with `cargo build`.

To run the server in watch mode, first install `cargo-watch` using `cargo install cargo-watch`. Then run `cargo watch -x run`.
Now with every change made in the code, the server will automatically recompile and continue running.

We use a postgres database. You can find a configuration in the docker/postgres.sh script.
To interact with the database, use Diesel.
To install use `cargo install diesel_cli --no-default-features --features postgres`.

With Diesel installed, you can run migrations with `diesel migration run`. This will create the needed tables.

To start the server, a `.env` file must be created that contains some settings. You can find an example in `.env.example`.
