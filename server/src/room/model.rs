use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::BelongingToDsl;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::rooms;
use crate::schema::rooms_users;

#[derive(Serialize, Identifiable, Deserialize, Queryable, Insertable, PartialEq, Debug)]
#[table_name = "rooms"]
pub struct Room {
    pub id: Uuid,
    pub name: Option<String>,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Room)]
#[primary_key(user_id, room_id)]
#[table_name = "rooms_users"]
pub struct RoomUser {
    user_id: Uuid,
    room_id: Uuid,
    status: Option<String>,
}

pub enum RoomError {
    GenericError,
    DatabaseError,
    RoomNotFound,
}

impl Room {
    pub fn find_all(conn: &PgConnection) -> Result<Vec<Room>, RoomError> {
        use crate::schema::rooms::dsl::*;

        Ok(rooms.load::<Room>(conn)?)
    }

    pub fn find(conn: &PgConnection, room_id: Uuid) -> Result<Option<Room>, RoomError> {
        use crate::schema::rooms::dsl::*;

        Ok(rooms.find(room_id).get_result::<Room>(conn).optional()?)
    }

    pub fn get_room_users(conn: &PgConnection, room: &Room) -> Result<Vec<RoomUser>, RoomError> {
        Ok(RoomUser::belonging_to(room).load::<RoomUser>(conn)?)
    }

    pub fn get_user_ids(conn: &PgConnection, room_id: Uuid) -> Result<Vec<Uuid>, RoomError> {
        use crate::schema::rooms::dsl::*;

        let room = rooms.find(room_id).get_result::<Room>(conn)?;
        let room_users = Room::get_room_users(conn, &room)?;
        Ok(room_users
            .into_iter()
            .map(|room_user| room_user.user_id)
            .collect::<Vec<Uuid>>())
    }

    pub fn add_users(
        conn: &PgConnection,
        room_id: Uuid,
        user_ids: Vec<Uuid>,
    ) -> Result<usize, RoomError> {
        Err(RoomError::GenericError)
    }

    pub fn remove_users(
        conn: &PgConnection,
        room_id: Uuid,
        user_ids: Vec<Uuid>,
    ) -> Result<usize, RoomError> {
        Err(RoomError::GenericError)
    }

    pub fn create(room_name: &str, conn: &PgConnection) -> Result<Room, RoomError> {
        Err(RoomError::GenericError)
    }

    pub fn destroy(conn: &PgConnection, room_id: Uuid) -> Result<usize, RoomError> {
        Err(RoomError::GenericError)
    }

    pub fn update(room_id: Uuid, room_name: &str, conn: &PgConnection) -> Result<Room, RoomError> {
        Err(RoomError::GenericError)
    }
}

impl From<DieselError> for RoomError {
    fn from(error: DieselError) -> RoomError {
        match error {
            DieselError::DatabaseError(_, _) => RoomError::DatabaseError,
            DieselError::NotFound => RoomError::RoomNotFound,
            _ => RoomError::GenericError,
        }
    }
}
