use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::messages;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Queryable, Insertable, PartialEq, Debug)]
#[table_name = "messages"]
pub struct Message {
    pub id: Uuid,
    pub room_id: Uuid,
    pub author: Uuid,
    pub content: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

// decode request data
#[derive(Clone, Deserialize, Insertable, AsChangeset, Debug)]
#[table_name = "messages"]
pub struct MessageData {
    pub content: String,
    pub room_id: Uuid,
    pub author: Uuid,
}

#[derive(Debug)]
pub enum MessageError {
    MessageNotFound,
    DatabaseError,
    GenericError,
}

impl Message {
    pub fn find(message_id: Uuid, conn: &PgConnection) -> Result<Option<Message>, MessageError> {
        use crate::schema::messages::dsl::*;

        Ok(messages
            .find(message_id)
            .get_result::<Message>(conn)
            .optional()?)
    }

    pub fn find_all_by_room(room: Uuid, conn: &PgConnection) -> Result<Vec<Message>, MessageError> {
        use crate::schema::messages::dsl::*;
        Ok(messages.filter(room_id.eq(room)).load::<Message>(conn)?)
    }

    pub fn create(message_data: MessageData, conn: &PgConnection) -> Result<Message, MessageError> {
        use crate::schema::messages::dsl::*;

        let new_message: Message = diesel::insert_into(messages)
            .values(&message_data)
            .get_result(conn)?;
        Ok(new_message)
    }

    pub fn update(
        message_id: Uuid,
        message_data: MessageData,
        conn: &PgConnection,
    ) -> Result<Message, MessageError> {
        use crate::schema::messages::dsl::*;

        let message: Message = diesel::update(messages.find(message_id))
            .set(message_data)
            .get_result(conn)?;
        Ok(message)
    }

    pub fn destroy(message_id: Uuid, conn: &PgConnection) -> Result<usize, MessageError> {
        use crate::schema::messages::dsl::*;

        let count = diesel::delete(messages.find(message_id)).execute(conn)?;
        Ok(count)
    }
}

impl From<DieselError> for MessageError {
    fn from(error: DieselError) -> MessageError {
        match error {
            DieselError::DatabaseError(_, _) => MessageError::DatabaseError,
            DieselError::NotFound => MessageError::MessageNotFound,
            _ => MessageError::GenericError,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;

    fn setup_hello_thermit_message(conn: &PgConnection) -> Result<Message, MessageError> {
        let room = setup_room(conn);
        let user = setup_user(conn);
        let message_data = create_message_data("Hello thermit!", room.id, user.id);
        Message::create(message_data, conn)
    }

    #[test]
    fn create_returns_new_message() {
        let conn = connection();

        let message = setup_hello_thermit_message(&conn).unwrap();
        assert_eq!(message.content, "Hello thermit!");
    }

    #[test]
    fn create_returns_error_when_user_and_room_do_not_exist() {
        let conn = connection();

        let message_data = create_message_data("Hello thermit!", Uuid::new_v4(), Uuid::new_v4());
        let message_result = Message::create(message_data, &conn);

        assert!(matches!(message_result, Err(MessageError::DatabaseError)));
    }

    #[test]
    fn find_returns_message() {
        let conn = connection();

        let message = setup_hello_thermit_message(&conn).unwrap();
        let found_message = Message::find(message.id, &conn).unwrap().unwrap();
        assert_eq!(found_message.content, "Hello thermit!");
    }

    #[test]
    fn find_returns_no_message_when_message_does_not_exist() {
        let conn = connection();

        let _message = setup_hello_thermit_message(&conn).unwrap();
        let found_message = Message::find(Uuid::new_v4(), &conn).unwrap();
        assert!(matches!(found_message, None));
    }

    #[test]
    fn find_all_by_room_returns_messages_for_room() {
        let conn = connection();

        let room1 = setup_room(&conn);
        let room2 = setup_room(&conn);
        let user1 = setup_user_with_username(&conn, "user1");
        let user2 = setup_user_with_username(&conn, "user2");
        let message_data1 = create_message_data("How are you?", room1.id, user1.id);
        let message_data2 = create_message_data("I'm great!", room1.id, user2.id);
        let message_data3 = create_message_data("Hello thermit!", room2.id, user1.id);
        Message::create(message_data1, &conn).unwrap();
        Message::create(message_data2, &conn).unwrap();
        Message::create(message_data3, &conn).unwrap();

        let messages_for_room1 = Message::find_all_by_room(room1.id, &conn).unwrap();
        let messages_for_room2 = Message::find_all_by_room(room2.id, &conn).unwrap();
        assert_eq!(messages_for_room1.len(), 2);
        assert_eq!(messages_for_room2.len(), 1);
    }

    #[test]
    fn update_updates_message() {
        let conn = connection();

        let message = setup_hello_thermit_message(&conn).unwrap();
        let updated_message_data =
            create_message_data("Bye thermit!", message.room_id, message.author);
        let updated_message = Message::update(message.id, updated_message_data, &conn).unwrap();
        assert_eq!(updated_message.id, message.id);
        assert_eq!(updated_message.content, "Bye thermit!");
    }

    #[test]
    fn update_returns_error_when_message_does_not_exist() {
        let conn = connection();

        let message = setup_hello_thermit_message(&conn).unwrap();
        let updated_message_data =
            create_message_data("Bye thermit!", message.room_id, message.author);
        let updated_message = Message::update(Uuid::new_v4(), updated_message_data, &conn);
        assert!(matches!(
            updated_message,
            Err(MessageError::MessageNotFound)
        ));
    }

    #[test]
    fn destroy_deletes_message() {
        let conn = connection();

        let message = setup_hello_thermit_message(&conn).unwrap();
        let deleted_count = Message::destroy(message.id, &conn).unwrap();
        assert_eq!(deleted_count, 1);
        let found_message = Message::find(message.id, &conn).unwrap();
        assert!(matches!(found_message, None));
    }

    #[test]
    fn destroy_returns_zero_when_message_does_not_exist() {
        let conn = connection();

        setup_hello_thermit_message(&conn).unwrap();
        let deletion_result = Message::destroy(Uuid::new_v4(), &conn).unwrap();
        assert_eq!(deletion_result, 0);
    }
}
