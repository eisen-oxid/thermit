use actix_web::http::StatusCode;
use actix_web::{error::ResponseError, HttpResponse};
use derive_more::{Display, Error};
use serde::Serialize;
use serde_json::json;

use crate::room::RoomError;
use crate::user::auth::AuthenticationError;
use crate::user::UserError;
use actix_web::error::BlockingError;

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

impl From<BlockingError<UserError>> for ServiceError {
    fn from(error: BlockingError<UserError>) -> ServiceError {
        match error {
            BlockingError::Error(e) => ServiceError::from(e),
            BlockingError::Canceled => ServiceError::InternalServerError,
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

impl From<UserError> for ServiceError {
    fn from(error: UserError) -> ServiceError {
        match error {
            UserError::UserNotFound => ServiceError::NotFound,
            UserError::UsernameTaken => ServiceError::Forbidden,
            UserError::DatabaseError => ServiceError::InternalServerError,
            UserError::GenericError => ServiceError::InternalServerError,
        }
    }
}

impl From<BlockingError<RoomError>> for ServiceError {
    fn from(error: BlockingError<RoomError>) -> ServiceError {
        match error {
            BlockingError::Error(e) => ServiceError::from(e),
            BlockingError::Canceled => ServiceError::InternalServerError,
        }
    }
}

impl From<RoomError> for ServiceError {
    fn from(error: RoomError) -> ServiceError {
        match error {
            RoomError::GenericError => ServiceError::InternalServerError,
            RoomError::DatabaseError => ServiceError::InternalServerError,
            RoomError::RoomNotFound => ServiceError::NotFound,
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
