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

pub enum MessageError {
    MessageNotFound,
    DatabaseError,
    GenericError,
}

impl Message {
    pub fn find(conn: &PgConnection, message_id: Uuid) -> Result<Option<Message>, MessageError> {
        use crate::schema::messages::dsl::*;

        Ok(messages
            .find(message_id)
            .get_result::<Message>(conn)
            .optional()?)
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
