use actix_web::http::StatusCode;
use actix_web::{error::ResponseError, HttpResponse};
use derive_more::{Display, Error};
use diesel::result::Error as DieselError;
use serde::Serialize;
use serde_json::json;

use crate::user::auth::AuthenticationError;

#[derive(Debug, Display, Error, Serialize)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "Internal Server Error")]
    NotFound,

    #[display(fmt = "Internal Server Error")]
    NoContent,

    #[display(fmt = "Internal Server Error")]
    Forbidden,
}
impl ServiceError {
    pub fn json_message(msg: &str) -> serde_json::Value {
        json!({ "msg": msg })
    }
}

impl From<DieselError> for ServiceError {
    fn from(error: DieselError) -> ServiceError {
        match error {
            DieselError::DatabaseError(_, _) => ServiceError::InternalServerError,
            DieselError::NotFound => ServiceError::NotFound,
            _ => ServiceError::InternalServerError,
        }
    }
}

impl From<AuthenticationError> for ServiceError {
    fn from(error: AuthenticationError) -> ServiceError {
        match error {
            AuthenticationError::IncorrectPassword => ServiceError::Forbidden,
            AuthenticationError::UserNotFound => ServiceError::NotFound,
            AuthenticationError::BcryptError(_) => ServiceError::InternalServerError,
            AuthenticationError::DatabaseError(_) => ServiceError::InternalServerError,
        }
    }
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::NotFound => StatusCode::NOT_FOUND,
            ServiceError::NoContent => StatusCode::NO_CONTENT,
            ServiceError::Forbidden => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::NotFound => {
                HttpResponse::NotFound().json(ServiceError::json_message("Not found"))
            }
            ServiceError::NoContent => {
                HttpResponse::NoContent().json(ServiceError::json_message("No content"))
            }
            ServiceError::Forbidden => {
                HttpResponse::Forbidden().json(ServiceError::json_message("Access forbidden"))
            }
        }
    }
}
