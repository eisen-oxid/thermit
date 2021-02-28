use diesel::prelude::*;
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::ServiceError;
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

impl User {
    fn generate_password(clear_password: &str) -> String {
        bcrypt::hash(clear_password).unwrap()
    }

    fn check_password(self: &User, clear_password: &str) -> bool {
        bcrypt::verify(clear_password, &*self.password)
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

    pub fn authenticate(
        conn: &PgConnection,
        user_data: UserData,
        user_id: Uuid,
    ) -> Result<String, ServiceError> {
        let user = User::find(&conn, user_id).unwrap();
        match user {
            None => Err(ServiceError::NotFound),
            Some(u) => {
                if u.check_password(&*user_data.password) && user_data.username == u.username {
                    Ok(String::from("AUTH_TOKEN_NOT_IMPLEMENTED"))
                } else {
                    Err(ServiceError::Forbidden)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;

    fn create_user_data() -> UserData {
        UserData {
            username: String::from("testUser"),
            password: String::from("12345678"),
        }
    }

    fn setup_user(conn: &PgConnection) -> User {
        User::create(create_user_data(), conn).unwrap()
    }

    #[test]
    fn generate_password_hashes_password() {
        let clear_password = "p455w0rd!";
        let actual = User::generate_password(clear_password);
        assert_ne!(clear_password, actual);
        assert!(bcrypt::verify(clear_password, &actual));
    }

    #[test]
    fn create_returns_new_user() {
        let conn = connection();

        let user_data = create_user_data();
        let user = User::create(user_data.clone(), &conn).unwrap();
        assert_eq!(user.username, user_data.username);
        assert!(bcrypt::verify(user_data.password, &user.password));
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

        assert_eq!(Some(expected), user);
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
        setup_user(&conn);

        let users = User::find_all(&conn).unwrap();

        assert_eq!(users.len(), 2);
        assert_ne!(users[0].id, users[1].id);
    }

    #[test]
    fn update_fails_with_not_found_if_user_does_not_exist() {
        let conn = connection();

        let user = create_user_data();

        let user = User::update(Uuid::new_v4(), user, &conn);
        assert!(matches!(user, Err(diesel::result::Error::NotFound)));
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
        assert!(bcrypt::verify(update_user.password, &updated_user.password));
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
