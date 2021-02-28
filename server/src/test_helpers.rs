use diesel::prelude::*;

pub fn connection() -> PgConnection {
    let url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&url).unwrap();
    conn.begin_test_transaction().unwrap();
    conn
}
