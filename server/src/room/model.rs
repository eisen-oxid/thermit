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

#[derive(Clone, Deserialize, Insertable, AsChangeset, Debug)]
#[table_name = "rooms"]
pub struct RoomData {
    pub name: Option<String>,
}

#[derive(Identifiable, Queryable, Associations, Insertable, PartialEq, Debug)]
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
        existing_room_id: Uuid,
        user_ids: Vec<Uuid>,
    ) -> Result<usize, RoomError> {
        use crate::schema::rooms_users::dsl::*;

        for user_id_to_add in user_ids.iter() {
            let room_user = RoomUser {
                user_id: *user_id_to_add,
                room_id: existing_room_id,
                status: None,
            };
            diesel::insert_into(rooms_users)
                .values(room_user)
                .execute(conn)?;
        }
        Ok(user_ids.len())
    }

    pub fn remove_users(
        conn: &PgConnection,
        room_id: Uuid,
        user_ids: Vec<Uuid>,
    ) -> Result<usize, RoomError> {
        let mut count = 0;
        let room = Room::find(conn, room_id)?.unwrap();
        let rooms_users_pairs: Vec<RoomUser> = Room::get_room_users(conn, &room)?
            .into_iter()
            .filter(|rooms_users| user_ids.contains(&rooms_users.user_id))
            .collect::<Vec<RoomUser>>();
        for r_u in rooms_users_pairs.iter() {
            count += diesel::delete(r_u).execute(conn)?;
        }
        Ok(count)
    }

    pub fn create(room_data: RoomData, conn: &PgConnection) -> Result<Room, RoomError> {
        use crate::schema::rooms::dsl::*;

        let new_room: Room = diesel::insert_into(rooms)
            .values(&room_data)
            .get_result(conn)?;
        Ok(new_room)
    }

    pub fn destroy(conn: &PgConnection, room_id: Uuid) -> Result<usize, RoomError> {
        use crate::schema::rooms::dsl::*;

        let count = diesel::delete(rooms.find(room_id)).execute(conn)?;
        Ok(count)
    }

    pub fn update(
        room_id: Uuid,
        room_data: RoomData,
        conn: &PgConnection,
    ) -> Result<Room, RoomError> {
        use crate::schema::rooms::dsl::*;

        let room: Room = diesel::update(rooms.find(room_id))
            .set(room_data)
            .get_result(conn)?;
        Ok(room)
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
