#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{middleware::Logger, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

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

    let server_ip = std::env::var("SERVER_IP").expect("SERVER_IP must be set");
    let server_port = std::env::var("SERVER_PORT").expect("SERVER_PORT must be set");

    let server_address = format!("{}:{}", server_ip, server_port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .service(web::scope("/api").service(web::scope("/v1").configure(user::init_routes)))
    })
    .bind(server_address)?
    .run()
    .await
}
