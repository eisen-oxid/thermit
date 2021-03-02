use crate::user::auth::AuthenticationError::{DatabaseError, UserNotFound};
use crate::user::{User, UserData, UserError};
use diesel::PgConnection;
use pwhash::bcrypt;

#[derive(Debug)]
pub enum AuthenticationError {
    IncorrectPassword,
    UserNotFound,
    BcryptError(pwhash::error::Error),
    DatabaseError(UserError),
}

impl User {
    pub fn generate_password(clear_password: &str) -> String {
        bcrypt::hash(clear_password).unwrap()
    }

    fn check_password(self: &User, clear_password: &str) -> bool {
        bcrypt::verify(clear_password, &*self.password)
    }

    pub fn authenticate(
        conn: &PgConnection,
        user_data: UserData,
    ) -> Result<String, AuthenticationError> {
        let user = User::find_by_username(&conn, &*user_data.username);
        let user = match user {
            Err(e) => return Err(DatabaseError(e)),
            Ok(u) => u,
        };
        let user = match user {
            None => return Err(UserNotFound),
            Some(u) => u,
        };
        if user.check_password(&*user_data.password) && user_data.username == user.username {
            Ok(String::from("AUTH_TOKEN_NOT_IMPLEMENTED"))
        } else {
            Err(AuthenticationError::IncorrectPassword)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_helpers::*;
    use pwhash::bcrypt;

    #[test]
    fn generate_password_hashes_password() {
        let clear_password = "p455w0rd!";
        let actual = User::generate_password(clear_password);
        assert_ne!(clear_password, actual);
        assert!(bcrypt::verify(clear_password, &actual));
    }

    #[test]
    fn correct_password_authenticates_user() {
        let conn = connection();
        setup_user(&conn);
        let user_data = create_user_data("testUser");

        assert!(User::authenticate(&conn, user_data).is_ok());
    }

    #[test]
    fn incorrect_password_gives_incorrect_password() {
        let conn = connection();
        setup_user(&conn);
        let mut user_data = create_user_data("testUser");
        user_data.password = "Wrong password".to_string();

        assert!(matches!(
            User::authenticate(&conn, user_data),
            Err(AuthenticationError::IncorrectPassword)
        ));
    }

    #[test]
    fn authentication_with_unknown_username_gives_user_not_found() {
        let conn = connection();
        setup_user(&conn);
        let user_data = create_user_data("USER_NAME_DOES_NOT_EXIST");

        assert!(matches!(
            User::authenticate(&conn, user_data),
            Err(AuthenticationError::UserNotFound)
        ));
    }
}
