#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod errors;
mod schema;
mod user;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let server_port = std::env::var("SERVER_PORT").expect("SERVER_PORT must be set");
    let mut server_address = "0.0.0.0:".to_string();
    server_address.push_str(&server_port[..]);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(web::scope("/api").service(web::scope("/v1").configure(user::init_routes)))
    })
    .bind(server_address)?
    .run()
    .await
}
