use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::BelongingToDsl;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::room::RoomError::*;
use crate::schema::rooms;
use crate::schema::rooms_users;
use chrono::{NaiveDateTime, Utc};

#[derive(Serialize, Identifiable, Deserialize, Queryable, Insertable, PartialEq, Debug)]
#[table_name = "rooms"]
pub struct Room {
    pub id: Uuid,
    pub name: Option<String>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
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
    pub(crate) user_id: Uuid,
    room_id: Uuid,
    status: Option<String>,
    created: NaiveDateTime,
    updated: NaiveDateTime,
}

#[derive(Debug)]
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

    pub fn exists(conn: &PgConnection, room_id: Uuid) -> Result<bool, RoomError> {
        let room = Room::find(conn, room_id)?;
        match room {
            None => Ok(false),
            Some(_) => Ok(true),
        }
    }

    pub fn add_users(
        conn: &PgConnection,
        existing_room_id: Uuid,
        user_ids: Vec<Uuid>,
    ) -> Result<Vec<Uuid>, RoomError> {
        use crate::schema::rooms_users::dsl::*;

        if !Room::exists(conn, existing_room_id)? {
            return Err(RoomNotFound);
        };

        let mut added_users = vec![];

        let existing_users = Room::get_user_ids(conn, existing_room_id)?;

        for user_id_to_add in user_ids.into_iter() {
            let room_user = RoomUser {
                user_id: user_id_to_add,
                room_id: existing_room_id,
                status: None,
                created: Utc::now().naive_utc(),
                updated: Utc::now().naive_utc(),
            };

            // Check if the user to add exists
            use crate::user::User;
            match User::exists(conn, user_id_to_add) {
                Err(_) => return Err(DatabaseError),
                Ok(bool) => {
                    if !bool {
                        continue; // Do not add user
                    }
                }
            };

            // Check if the user is already in the room
            if existing_users.contains(&user_id_to_add) {
                continue; // Do not add user
            }

            diesel::insert_into(rooms_users)
                .values(room_user)
                .execute(conn)?;
            added_users.push(user_id_to_add);
        }
        Ok(added_users)
    }

    pub fn remove_users(
        conn: &PgConnection,
        room_id: Uuid,
        user_ids: Vec<Uuid>,
    ) -> Result<usize, RoomError> {
        let mut count = 0;

        let room = match Room::find(conn, room_id)? {
            None => return Err(RoomNotFound),
            Some(r) => r,
        };

        let rooms_users_pairs: Vec<RoomUser> = Room::get_room_users(conn, &room)?
            .into_iter()
            .filter(|rooms_users| user_ids.contains(&rooms_users.user_id))
            .collect::<Vec<RoomUser>>();

        for r_u in rooms_users_pairs.iter() {
            let query = diesel::delete(r_u);
            count += query.execute(conn)?;
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
        conn: &PgConnection,
        room_id: Uuid,
        room_data: RoomData,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;

    #[test]
    fn create_returns_new_room() {
        let conn = connection();

        let room_data = create_room_data("test room");
        let room = Room::create(room_data.clone(), &conn).unwrap();
        assert_eq!(room.name, room_data.name);
    }

    #[test]
    fn find_returns_none_when_no_room_exists() {
        let conn = connection();

        assert!(matches!(Room::find(&conn, Uuid::new_v4()), Ok(None)));
    }

    #[test]
    fn find_returns_room() {
        let conn = connection();
        let room = setup_room(&conn);
        assert_eq!(Room::find(&conn, room.id).unwrap().unwrap().id, room.id);
    }

    #[test]
    fn find_all_returns_empty_list_when_no_rooms_exist() {
        let conn = connection();

        assert_eq!(Room::find_all(&conn).unwrap().len(), 0);
    }

    #[test]
    fn find_all_returns_all_rooms() {
        let conn = connection();

        setup_room(&conn);
        Room::create(create_room_data("anotherRoom"), &conn).unwrap();

        let rooms = Room::find_all(&conn).unwrap();

        assert_eq!(rooms.len(), 2);
        assert_ne!(rooms[0].id, rooms[1].id);
    }

    #[test]
    fn room_can_be_deleted() {
        let conn = connection();

        let room = setup_room(&conn);

        let destroyed_count = Room::destroy(&conn, room.id).unwrap();
        let rooms = Room::find_all(&conn).unwrap();

        assert_eq!(rooms.len(), 0);
        assert_eq!(destroyed_count, 1);
    }

    #[test]
    fn room_can_be_updated() {
        let conn = connection();

        let room = setup_room(&conn);
        let updated_data = RoomData {
            name: Some("newName".to_string()),
        };
        Room::update(&conn, room.id, updated_data).unwrap();

        let updated_room = Room::find(&conn, room.id).unwrap().unwrap();
        assert_eq!(updated_room.name.unwrap(), "newName");
    }

    #[test]
    fn new_room_has_no_users() {
        let conn = connection();

        let room = setup_room(&conn);
        let users = Room::get_room_users(&conn, &room).unwrap();

        assert_eq!(users.len(), 0);
    }

    #[test]
    fn users_can_be_added_to_room() {
        let conn = connection();

        let room = setup_room(&conn);

        let user1 = setup_user_with_username(&conn, "test user 1");
        let user2 = setup_user_with_username(&conn, "test user 2");
        let user3 = setup_user_with_username(&conn, "test user 3");

        let rooms_users = vec![user1.id, user2.id, user3.id];

        Room::add_users(&conn, room.id, rooms_users).unwrap();
        let users = Room::get_room_users(&conn, &room).unwrap();

        assert_eq!(users.len(), 3);
        assert_eq!(users[0].user_id, user1.id);
    }

    #[test]
    fn non_existent_user_can_not_be_added_to_room() {
        let conn = connection();

        let room = setup_room(&conn);

        let user1 = setup_user_with_username(&conn, "test user 1");

        let rooms_users = vec![user1.id, Uuid::new_v4()];

        let added_users = Room::add_users(&conn, room.id, rooms_users).unwrap();

        assert_eq!(added_users.len(), 1);
        assert_eq!(added_users[0], user1.id);

        let users = Room::get_room_users(&conn, &room).unwrap();

        assert_eq!(users.len(), 1);
        assert_eq!(users[0].user_id, user1.id);
    }

    #[test]
    fn user_can_not_be_added_twice_to_room() {
        let conn = connection();

        let room = setup_room(&conn);

        let user1 = setup_user_with_username(&conn, "test user 1");

        let mut added_users = Room::add_users(&conn, room.id, vec![user1.id]).unwrap();

        assert_eq!(added_users.len(), 1);
        assert_eq!(added_users[0], user1.id);

        added_users = Room::add_users(&conn, room.id, vec![user1.id]).unwrap();

        assert_eq!(added_users.len(), 0);

        let users = Room::get_room_users(&conn, &room).unwrap();

        assert_eq!(users.len(), 1);
        assert_eq!(users[0].user_id, user1.id);
    }

    #[test]
    fn users_can_not_be_added_to_not_existing_room() {
        let conn = connection();

        let user1 = setup_user_with_username(&conn, "test user 1");
        let user2 = setup_user_with_username(&conn, "test user 2");

        let rooms_users_to_add = vec![user1.id, user2.id];

        let result = Room::add_users(&conn, Uuid::new_v4(), rooms_users_to_add);

        assert!(matches!(result, Err(RoomError::RoomNotFound)));

        use crate::schema::rooms_users::dsl::*;
        let room_user_list = rooms_users.load::<RoomUser>(&conn).unwrap();
        assert_eq!(room_user_list.len(), 0);
    }

    #[test]
    fn users_can_be_removed_from_room() {
        let conn = connection();

        let room = setup_room(&conn);
        let user1 = setup_user_with_username(&conn, "test user 1");
        let user2 = setup_user_with_username(&conn, "test user 2");
        let user3 = setup_user_with_username(&conn, "test user 3");

        let rooms_users = vec![user1.id, user2.id, user3.id];

        Room::add_users(&conn, room.id, rooms_users).unwrap();
        Room::remove_users(&conn, room.id, vec![user1.id]).unwrap();
        let users = Room::get_room_users(&conn, &room).unwrap();

        assert_eq!(users.len(), 2);
        assert_eq!(users[0].user_id, user2.id);
    }

    #[test]
    fn deleting_user_removes_user_from_room() {
        let conn = connection();

        let room = setup_room(&conn);
        let user1 = setup_user_with_username(&conn, "test user 1");
        let user2 = setup_user_with_username(&conn, "test user 2");

        let rooms_users = vec![user1.id, user2.id];

        Room::add_users(&conn, room.id, rooms_users).unwrap();

        use crate::user::User;
        User::destroy(&conn, user1.id).unwrap();

        let users = Room::get_room_users(&conn, &room).unwrap();

        assert_eq!(users.len(), 1);
    }
}
