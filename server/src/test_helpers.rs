use crate::{
    message::MessageData,
    room::{Room, RoomData},
    user::{User, UserData},
};
use diesel::prelude::*;
use diesel_migrations::*;
use uuid::Uuid;

pub fn connection() -> PgConnection {
    let url = dotenv::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let conn = PgConnection::establish(&url).unwrap();
    run_pending_migrations(&conn).unwrap();
    conn.begin_test_transaction().unwrap();
    conn
}

pub fn create_user_data(username: &str) -> UserData {
    UserData {
        username: String::from(username),
        password: String::from("12345678"),
    }
}

pub fn create_room_data(room_name: &str) -> RoomData {
    RoomData {
        name: Some(String::from(room_name)),
    }
}

pub fn create_message_data(content: &str, room_id: Uuid, author: Uuid) -> MessageData {
    MessageData {
        content: String::from(content),
        room_id,
        author,
    }
}

pub(crate) fn setup_user(conn: &PgConnection) -> User {
    setup_user_with_username(conn, "testUser")
}

pub(crate) fn setup_user_with_username(conn: &PgConnection, username: &str) -> User {
    let response = User::create(create_user_data(username), conn).unwrap();
    User::_find(conn, response.id).unwrap().unwrap()
}

pub(crate) fn setup_room(conn: &PgConnection) -> Room {
    Room::create(create_room_data("testRoom"), conn).unwrap()
}
