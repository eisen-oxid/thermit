use crate::test_db::CustomConnectionManager;
use crate::room::{Room, RoomData};
use crate::user::{User, UserData};
use crate::Pool;
use actix_web::dev::{HttpServiceFactory, ServiceResponse};
use actix_web::test::TestRequest;
use actix_web::App;
use diesel::prelude::*;
use diesel_migrations::*;

pub fn connection_pool() -> Pool {
    let database_url = dotenv::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let manager = CustomConnectionManager::<PgConnection>::new(database_url);
    return r2d2::Pool::builder()
        .max_size(1)
        .build(manager)
        .expect("Failed to create pool.");
}

pub async fn test_request<F: HttpServiceFactory + 'static>(
    pool: Pool,
    service: F,
    req: TestRequest,
) -> ServiceResponse {
    let mut app = actix_web::test::init_service(App::new().data(pool).service(service)).await;
    actix_web::test::call_service(&mut app, req.to_request()).await
}

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

pub(crate) fn setup_user(conn: &PgConnection) -> User {
    setup_user_with_username(conn, "testUser")
}

pub(crate) fn setup_user_with_username(conn: &PgConnection, username: &str) -> User {
    let response = User::create(create_user_data(username), conn).unwrap();
    User::_find(&conn, response.id).unwrap().unwrap()
}

pub(crate) fn setup_room(conn: &PgConnection) -> Room {
    Room::create(create_room_data("testRoom"), conn).unwrap()
}
