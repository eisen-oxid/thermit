use crate::errors::ServiceError;
use crate::room::{Room, RoomData};
use crate::Pool;
use actix_web::{delete, get, post, put, web, HttpResponse};
use uuid::Uuid;

#[get("/rooms")]
pub async fn list(pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let rooms = web::block(move || Room::find_all(&conn))
        .await
        .map_err(ServiceError::from)?;
    if rooms.is_empty() {
        Err(ServiceError::NoContent)
    } else {
        Ok(HttpResponse::Ok().json(rooms))
    }
}

#[get("/rooms/{id}")]
pub async fn find(
    pool: web::Data<Pool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let room = web::block(move || Room::find(&conn, id.into_inner()))
        .await
        .map_err(ServiceError::from)?;

    if let Some(room) = room {
        Ok(HttpResponse::Ok().json(room))
    } else {
        Err(ServiceError::NotFound)
    }
}

#[post("/rooms")]
async fn create(
    pool: web::Data<Pool>,
    room_data: web::Json<RoomData>,
) -> Result<HttpResponse, ServiceError> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let room = web::block(move || Room::create(room_data.into_inner(), &conn))
        .await
        .map_err(ServiceError::from)?;

    Ok(HttpResponse::Ok().json(room))
}

#[put("/rooms/{id}")]
pub async fn update(
    pool: web::Data<Pool>,
    id: web::Path<Uuid>,
    room_data: web::Json<RoomData>,
) -> Result<HttpResponse, ServiceError> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let room = web::block(move || Room::update(&conn, id.into_inner(), room_data.into_inner()))
        .await
        .map_err(ServiceError::from)?;
    Ok(HttpResponse::Ok().json(room))
}

#[delete("/rooms/{id}")]
pub async fn delete(
    pool: web::Data<Pool>,
    room_id: web::Path<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let count = web::block(move || Room::destroy(&conn, room_id.into_inner()))
        .await
        .map_err(ServiceError::from)?;

    if count == 0 {
        Err(ServiceError::NotFound)
    } else {
        Ok(HttpResponse::NoContent().finish())
    }
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(list);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}
