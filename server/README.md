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

### Logging

We use the [env_logger](https://docs.rs/env_logger/0.8.3/env_logger/) for logging.
See [Enable logging](https://docs.rs/env_logger/0.8.3/env_logger/#enabling-logging) for configurations.

### TLS

The server can be configured to encrypt connections using TLS, based on openSSL. To enable this option, set the `USE_TLS` option in your .env file.
You have to add the key and certificate in the PEM format and specify the path in the .env file. Restart the server and you'll be able to use HTTPS.

#### Seeding

To create some test data, you can use the script `seed.sh` in the seed directory. Replace the password in the code with the password you set for the database.
This will create some example data to get started. All users have `123456` as password.
