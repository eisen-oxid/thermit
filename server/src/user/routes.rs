use crate::errors::ServiceError;
use crate::user::model::{User, UserData};
use crate::Pool;
use actix_web::error::BlockingError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use uuid::Uuid;

#[get("/users")]
pub async fn list(pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    // use web::block to offload blocking Diesel code without blocking server thread
    let users = web::block(move || User::find_all(&conn))
        .await
        .map_err(ServiceError::from)?;
    Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{id}")]
pub async fn find(
    pool: web::Data<Pool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let user = web::block(move || User::find(&conn, id.into_inner()))
        .await
        .map_err(ServiceError::from)?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        Err(ServiceError::NotFound)
    }
}

#[post("/users")]
async fn create(
    pool: web::Data<Pool>,
    user_data: web::Json<UserData>,
) -> Result<HttpResponse, ServiceError> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || User::create(user_data.into_inner(), &conn))
        .await
        .map_err(ServiceError::from)?;

    Ok(HttpResponse::Ok().json(user))
}

#[put("/users/{id}")]
pub async fn update(
    pool: web::Data<Pool>,
    id: web::Path<Uuid>,
    user_data: web::Json<UserData>,
) -> Result<HttpResponse, ServiceError> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let user = web::block(move || User::update(id.into_inner(), user_data.into_inner(), &conn))
        .await
        .map_err(ServiceError::from)?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{id}")]
pub async fn delete(
    pool: web::Data<Pool>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let count = web::block(move || User::destroy(&conn, user_id.into_inner()))
        .await
        .map_err(ServiceError::from)?;

    if count == 0 {
        Err(ServiceError::NotFound)
    } else {
        Ok(HttpResponse::NoContent().finish())
    }
}

#[post("/auth")]
pub async fn authenticate(
    pool: web::Data<Pool>,
    user_data: web::Json<UserData>,
) -> Result<HttpResponse, ServiceError> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let auth_token = web::block(move || User::authenticate(&conn, user_data.into_inner())).await;
    match auth_token {
        Ok(token) => Ok(HttpResponse::Ok().json(json!({ "token": token }))),
        Err(e) => match e {
            BlockingError::Error(e) => Err(ServiceError::from(e)),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(list);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
    config.service(authenticate);
}
