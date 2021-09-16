use crate::errors::ServiceError;
use crate::message::{Message, MessageData};
use crate::Pool;
use actix_web::{get, post, web, HttpResponse};
use uuid::Uuid;

#[get("/messages/{id}")]
pub async fn find(
    pool: web::Data<Pool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let message = web::block(move || Message::find(id.into_inner(), &conn))
        .await
        .map_err(ServiceError::from)?;

    if let Some(message) = message {
        Ok(HttpResponse::Ok().json(message))
    } else {
        Err(ServiceError::NotFound)
    }
}

#[post("/messages")]
async fn create(
    pool: web::Data<Pool>,
    message_data: web::Json<MessageData>,
) -> Result<HttpResponse, ServiceError> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let message = web::block(move || Message::create(message_data.into_inner(), &conn))
        .await
        .map_err(ServiceError::from)?;

    Ok(HttpResponse::Ok().json(message))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find);
    config.service(create);
}
