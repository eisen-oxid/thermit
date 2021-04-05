#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{middleware::Logger, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod errors;
mod room;
mod schema;
mod user;

#[cfg(test)]
mod test_helpers;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // load tls
    let mut using_tls = false;
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    if let Ok(_) = std::env::var("USE_TLS") {
        let key_path =
            std::env::var("TLS_KEY_PATH").expect("TLS_KEY_PATH must be set when using TLS");
        let cert_path =
            std::env::var("TLS_CERT_PATH").expect("TLS_CERT_PATH must be set when using TLS");
        builder
            .set_private_key_file(key_path, SslFiletype::PEM)
            .unwrap();
        builder.set_certificate_chain_file(cert_path).unwrap();

        using_tls = true;
    }

    let server_ip = std::env::var("SERVER_IP").expect("SERVER_IP must be set");
    let server_port = std::env::var("SERVER_PORT").expect("SERVER_PORT must be set");

    let server_address = format!("{}:{}", server_ip, server_port);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .service(
                web::scope("/api/v1")
                    .configure(user::init_routes)
                    .configure(room::init_routes),
            )
    });

    if using_tls {
        server.bind_openssl(server_address, builder)?.run().await
    } else {
        server.bind(server_address)?.run().await
    }
}
