use crate::Pool;
use diesel;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::users;
use crate::schema::users::dsl::*;

use actix_web::web;

#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}
// decode request data
#[derive(Deserialize)]
pub struct UserData {
    pub username: String,
}
// this is to insert users to database
#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

impl User {
    pub fn find_all(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
        let conn = pool.get().unwrap();
        let items = users.load::<User>(&conn)?;
        Ok(items)
    }
}
