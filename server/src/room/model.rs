use diesel::prelude::*;
use diesel::BelongingToDsl;
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::room;

#[derive(Serialize, Identifiable, Deserialize, Queryable, Insertable, PartialEq, Debug)]
#[table_name = "rooms"]
pub struct Room {
    pub id: Uuid,
    pub name: String
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Room)]
#[primary_key(user_id, room_id)]
#[table_name = "rooms_users"]
pub struct RoomUser {
    user_id : Uuid,
    room_id : Uuid,
    status: String
}

enum RoomError {
    GenericError
}

impl Room {

    pub fn find_all(conn: &PgConnection) -> Result<Vec<Room>, RoomError>{
        Err(RoomError::GenericError)
    }

    pub fn find(conn: &PgConnection, room_id : Uuid) -> Result<Option<Room>, RoomError>{
        Err(RoomError::GenericError)
    }

    pub fn get_users(conn: &PgConnection, room_id: Uuid) -> Result<Vec<UserData>, RoomError>{
        Err(RoomError::GenericError)
    }

    pub fn add_users(conn: &PgConnection, room_id: Uuid, user_ids: Vec<Uuid>) -> Result<usize, RoomError>{
        Err(RoomError::GenericError)
    }

    pub fn remove_users(conn: &PgConnection, room_id: Uuid , user_ids: Vec<Uuid>) -> Result<usize, RoomError>{
        Err(RoomError::GenericError)
    }

    pub fn create(room_name: &str, conn: &PgConnection) -> Result<Room, RoomError> {
        Err(RoomError)
    }

    pub fn destroy(conn: &PgConnection, room_id : Uuid) -> Result<usize, RoomError> {
        Err(RoomError::GenericError)
    }

    pub fn update(
        room_id: Uuid,
        room_name: &str,
        conn: &PgConnection,
    ) -> Result<Room, RoomError> {
        Err(RoomError::GenericError)
    }
}
