use crate::errors::ServiceError;
use crate::room::{Room, RoomData};
use crate::Pool;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

#[get("/rooms")]
pub async fn list(pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let rooms = web::block(move || Room::find_all(&conn))
        .await
        .map_err(ServiceError::from)?;

    Ok(HttpResponse::Ok().json(json!({ "rooms": rooms })))
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
        get_json_response(&pool, room)
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

    get_json_response(&pool, room)
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

    get_json_response(&pool, room)
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

#[derive(Deserialize, Debug)]
pub struct RoomUserData {
    id: Uuid,
}

#[post("/rooms/{id}/users")]
pub async fn add_user(
    pool: web::Data<Pool>,
    room_id: web::Path<Uuid>,
    user_data: web::Json<RoomUserData>,
) -> Result<HttpResponse, ServiceError> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    Room::add_users(&conn, room_id.into_inner(), vec![user_data.into_inner().id])?;

    Ok(HttpResponse::NoContent().finish())
}

#[get("/rooms/{id}/users")]
pub async fn get_users(
    pool: web::Data<Pool>,
    room_id: web::Path<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let possible_room = Room::find(&conn, room_id.into_inner())?;
    let room = match possible_room {
        None => return Err(ServiceError::NotFound),
        Some(room) => room,
    };

    let room_user_ids: Vec<Uuid> = Room::get_room_users(&conn, &room)?
        .into_iter()
        .map(|room_user| room_user.user_id)
        .collect();

    Ok(HttpResponse::Ok().json(json!({ "users": room_user_ids })))
}

#[delete("/rooms/{room_id}/users/{user_id}")]
pub async fn remove_user(
    pool: web::Data<Pool>,
    ids: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ServiceError> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let ids_content = ids.into_inner();
    let room_id = ids_content.0;
    let user_id = ids_content.1;

    let count = Room::remove_users(&conn, room_id, vec![user_id])?;

    if count == 0 {
        Err(ServiceError::NotFound)
    } else {
        Ok(HttpResponse::NoContent().finish())
    }
}

fn get_json_response(pool: &web::Data<Pool>, room: Room) -> Result<HttpResponse, ServiceError> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let room_user_ids: Vec<Uuid> = Room::get_room_users(&conn, &room)?
        .into_iter()
        .map(|room_user| room_user.user_id)
        .collect();

    let room_json = json!({
        "id": room.id,
        "name": room.name,
        "users": room_user_ids
    });
    Ok(HttpResponse::Ok().json(room_json))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(list);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);

    config.service(add_user);
    config.service(get_users);
    config.service(remove_user);
}
