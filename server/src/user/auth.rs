use crate::user::auth::AuthenticationError::{DatabaseError, UserNotFound};
use crate::user::{User, UserData};
use diesel::PgConnection;
use pwhash::bcrypt;

#[derive(Debug)]
pub enum AuthenticationError {
    IncorrectPassword,
    UserNotFound,
    BcryptError(pwhash::error::Error),
    DatabaseError(diesel::result::Error),
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
