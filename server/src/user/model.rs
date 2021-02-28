use diesel::prelude::*;
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

    pub fn create(user_data: UserData, conn: &PgConnection) -> Result<Self, diesel::result::Error> {
        use crate::schema::users::dsl::*;

        let new_user = diesel::insert_into(users)
            .values(&user_data)
            .get_result(conn)?;
        Ok(new_user)
    }

    pub fn update(
        user_id: Uuid,
        user_data: UserData,
        conn: &PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::users::dsl::*;

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
