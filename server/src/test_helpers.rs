use crate::user::{User, UserData};
use diesel::prelude::*;

pub fn connection() -> PgConnection {
    let url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&url).unwrap();
    conn.begin_test_transaction().unwrap();
    conn
}

pub fn create_user_data(username: &str) -> UserData {
    UserData {
        username: String::from(username),
        password: String::from("12345678"),
    }
}

pub(crate) fn setup_user(conn: &PgConnection) -> User {
    let response = User::create(create_user_data("testUser"), conn).unwrap();
    User::_find(&conn, response.id).unwrap().unwrap()
}
