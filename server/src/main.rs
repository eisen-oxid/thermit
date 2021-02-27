#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod errors;
mod schema;
mod user;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn home(_request: HttpRequest) -> impl Responder {
    println!("Home visited");
    HttpResponse::Ok().body("Hello world!")
}

#[post("/home2")]
async fn home2(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

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

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(home))
            .service(home2)
            .service(web::scope("/api").service(web::scope("/v1").configure(user::init_routes)))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
