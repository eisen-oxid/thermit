use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::messages;
use chrono::NaiveDateTime;
use crate::message::MessageError::MessageNotFound;

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
    MessageNotFound
}

impl Message {
    pub fn find(conn: &PgConnection, message_id: Uuid) -> Result<Option<Message>, MessageError> {
        Err(MessageNotFound)
    }

    pub fn create(message_data: MessageData, conn: &PgConnection) -> Result<Message, MessageError> {
        Err(MessageNotFound)
    }

    pub fn update(message_id: Uuid, message_data: MessageData,conn: &PgConnection,) -> Result<Message, MessageError> {
        Err(MessageNotFound)
    }

    pub fn destroy(message_id: Uuid, conn: &PgConnection) -> Result<usize, MessageError> {
        Err(MessageNotFound)
    }
}
