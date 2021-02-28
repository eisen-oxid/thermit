use diesel::prelude::*;
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}
// decode request data
#[derive(Deserialize, Insertable, AsChangeset, Debug)]
#[table_name = "users"]
pub struct UserData {
    pub username: String,
    pub password: String,
}

impl User {
    fn generate_password(clear_password: &str) -> String {
        bcrypt::hash(clear_password).unwrap()
    }

    pub fn find_all(conn: &PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::users::dsl::*;

        let items = users.load::<User>(conn)?;
        Ok(items)
    }

    pub fn find(conn: &PgConnection, user_id: Uuid) -> Result<Option<Self>, diesel::result::Error> {
        use crate::schema::users::dsl::*;

        let user = users.find(user_id).get_result::<User>(conn).optional()?;

        Ok(user)
    }

    pub fn create(
        mut user_data: UserData,
        conn: &PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::users::dsl::*;

        user_data.password = User::generate_password(&*user_data.password);

        let new_user = diesel::insert_into(users)
            .values(&user_data)
            .get_result(conn)?;
        Ok(new_user)
    }

    pub fn update(
        user_id: Uuid,
        mut user_data: UserData,
        conn: &PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::users::dsl::*;

        // If no password is specified, do not update it
        if !user_data.password.is_empty() {
            user_data.password = User::generate_password(&*user_data.password);
        } else {
            let old_password = User::find(&conn, user_id).unwrap().unwrap().password;
            user_data.password = old_password;
        }

        let user = diesel::update(users.find(user_id))
            .set(user_data)
            .get_result(conn)?;

        Ok(user)
    }

    pub fn destroy(conn: &PgConnection, user_id: Uuid) -> Result<usize, diesel::result::Error> {
        use crate::schema::users::dsl::*;

        let count = diesel::delete(users.find(user_id)).execute(conn)?;
        Ok(count)
    }
}
