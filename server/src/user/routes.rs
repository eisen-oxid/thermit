use crate::errors::ServiceError;
use crate::user::model::User;
use crate::Pool;
use actix_web::{get, web, HttpResponse};

#[get("/users")]
pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    Ok(web::block(move || User::find_all(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| ServiceError::InternalServerError)?)
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(get_users);
}
