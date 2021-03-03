use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;

#[derive(Serialize, Deserialize, Queryable, Insertable, PartialEq, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}
// decode request data
#[derive(Clone, Deserialize, Insertable, AsChangeset, Debug)]
#[table_name = "users"]
pub struct UserData {
    pub username: String,
    pub password: String,
}

// Do not return passwords, write only the data we want to send out in this struct
#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
}

#[derive(Debug)]
pub enum UserError {
    UserNotFound,
    UsernameTaken,
    DatabaseError,
    GenericError,
}

impl User {
    pub fn find_all(conn: &PgConnection) -> Result<Vec<UserResponse>, UserError> {
        use crate::schema::users::dsl::*;

        let all_users = users.load::<User>(conn)?;
        let items = all_users
            .into_iter()
            .map(|u| UserResponse::from(u))
            .collect::<Vec<UserResponse>>();
        Ok(items)
    }

    pub fn find(conn: &PgConnection, user_id: Uuid) -> Result<Option<UserResponse>, UserError> {
        let user = User::_find(&conn, user_id)?;

        if let Some(u) = user {
            Ok(Some(UserResponse::from(u)))
        } else {
            Ok(None)
        }
    }

    // Internal find, do not use in routes, as it returns the password
    pub(crate) fn _find(conn: &PgConnection, user_id: Uuid) -> Result<Option<User>, UserError> {
        use crate::schema::users::dsl::*;

        Ok(users.find(user_id).get_result::<User>(conn).optional()?)
    }

    pub fn find_by_username(
        conn: &PgConnection,
        u: &str,
    ) -> Result<Option<UserResponse>, UserError> {
        match User::_find_by_username(&conn, u) {
            Ok(user) => {
                if let Some(u) = user {
                    return Ok(Some(UserResponse::from(u)))
                }
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }

    // Only for internal use, do not use this in a route, as it returns the password!
    pub fn _find_by_username(conn: &PgConnection, u: &str) -> Result<Option<User>, UserError> {
        use crate::schema::users::dsl::*;

        let user = users
            .filter(username.eq(u))
            .first::<User>(conn)
            .optional()?;

        Ok(user)
    }

    fn username_taken(conn: &PgConnection, u: &str) -> Result<bool, UserError> {
        let user = User::find_by_username(conn, u)?;
        Ok(user.is_some())
    }

    pub fn create(mut user_data: UserData, conn: &PgConnection) -> Result<UserResponse, UserError> {
        use crate::schema::users::dsl::*;

        user_data.password = User::generate_password(&*user_data.password);

        if User::username_taken(&conn, &*user_data.username)? {
            return Err(UserError::UsernameTaken);
        }

        let new_user:User = diesel::insert_into(users)
            .values(&user_data)
            .get_result(conn)?;
        Ok(UserResponse::from(new_user))
    }

    pub fn update(
        user_id: Uuid,
        mut user_data: UserData,
        conn: &PgConnection,
    ) -> Result<UserResponse, UserError> {
        use crate::schema::users::dsl::*;

        if User::username_taken(&conn, &*user_data.username)? {
            return Err(UserError::UsernameTaken);
        }

        // If no password is specified, do not update it
        if !user_data.password.is_empty() {
            user_data.password = User::generate_password(&*user_data.password);
        } else {
            let old_password = User::_find(&conn, user_id).unwrap().unwrap().password;
            user_data.password = old_password;
        }

        let user:User = diesel::update(users.find(user_id))
            .set(user_data)
            .get_result(conn)?;

        Ok(UserResponse::from(user))
    }

    pub fn destroy(conn: &PgConnection, user_id: Uuid) -> Result<usize, UserError> {
        use crate::schema::users::dsl::*;

        let count = diesel::delete(users.find(user_id)).execute(conn)?;
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;

    #[test]
    fn create_returns_new_user() {
        let conn = connection();

        let user_data = create_user_data("testUser");
        let user = User::create(user_data.clone(), &conn).unwrap();
        assert_eq!(user.username, user_data.username);
    }

    #[test]
    fn create_fails_when_username_is_taken() {
        let conn = connection();

        let user_data = create_user_data("testUser");
        User::create(user_data.clone(), &conn).unwrap();

        let user = User::create(user_data.clone(), &conn);
        assert!(matches!(user, Err(UserError::UsernameTaken)));
    }

    #[test]
    fn find_returns_none_when_no_user_exists() {
        let conn = connection();

        assert!(matches!(User::find(&conn, Uuid::new_v4()), Ok(None)));
    }

    #[test]
    fn find_returns_user_when_exists() {
        let conn = connection();

        let expected = setup_user(&conn);
        let user = User::find(&conn, expected.id).unwrap();

        assert_eq!(expected.username, user.unwrap().username);
    }

    #[test]
    fn find_all_returns_empty_list_when_no_users_exist() {
        let conn = connection();

        assert_eq!(User::find_all(&conn).unwrap().len(), 0);
    }

    #[test]
    fn find_all_returns_all_users() {
        let conn = connection();

        setup_user(&conn);
        User::create(create_user_data("user2"), &conn).unwrap();

        let users = User::find_all(&conn).unwrap();

        assert_eq!(users.len(), 2);
        assert_ne!(users[0].id, users[1].id);
    }

    #[test]
    fn update_fails_with_not_found_if_user_does_not_exist() {
        let conn = connection();

        let user = create_user_data("testUser");

        let user = User::update(Uuid::new_v4(), user, &conn);
        assert!(matches!(user, Err(UserError::UserNotFound)));
    }

    #[test]
    fn update_returns_updated_user_if_exists() {
        let conn = connection();

        let mut user = setup_user(&conn);
        let update_user = UserData {
            username: String::from("new_username"),
            password: String::from("new_password"),
        };

        // Update user manually
        user.username = update_user.username.clone();

        let updated_user = User::update(user.id, update_user.clone(), &conn).unwrap();

        assert_eq!(updated_user.username, user.username);
    }

    #[test]
    fn destroy_returns_null_if_user_does_not_exist() {
        let conn = connection();

        let count = User::destroy(&conn, Uuid::new_v4()).unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn destroy_returns_one_if_user_exists() {
        let conn = connection();

        let user = setup_user(&conn);

        let count = User::destroy(&conn, user.id).unwrap();
        assert_eq!(count, 1);
    }
}

impl From<DieselError> for UserError {
    fn from(error: DieselError) -> UserError {
        match error {
            DieselError::DatabaseError(_, _) => UserError::DatabaseError,
            DieselError::NotFound => UserError::UserNotFound,
            _ => UserError::GenericError,
        }
    }
}

impl From<User> for UserResponse {
    fn from(user: User) -> UserResponse {
        UserResponse {
            id: user.id,
            username: user.username,
        }
    }
}
