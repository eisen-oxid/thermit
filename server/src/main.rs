use actix_web::{post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

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
    HttpServer::new(|| App::new().route("/", web::get().to(home)).service(home2))
        .bind("0.0.0.0:8000")?
        .run()
        .await
}
