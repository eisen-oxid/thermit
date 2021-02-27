use actix_web::http::StatusCode;
use actix_web::{error::ResponseError, HttpResponse};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
        }
    }
}
