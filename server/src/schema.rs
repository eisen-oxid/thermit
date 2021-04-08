table! {
    use diesel::sql_types::*;
    use crate::message::Message_encryption;
    messages (id) {
        id -> Uuid,
        room_id -> Uuid,
        author -> Uuid,
        content -> Varchar,
        encryption -> Message_encryption,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    rooms (id) {
        id -> Uuid,
        name -> Nullable<Varchar>,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    rooms_users (user_id, room_id) {
        user_id -> Uuid,
        room_id -> Uuid,
        status -> Nullable<Varchar>,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        password -> Varchar,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

joinable!(messages -> rooms (room_id));
joinable!(messages -> users (author));
joinable!(rooms_users -> rooms (room_id));
joinable!(rooms_users -> users (user_id));

allow_tables_to_appear_in_same_query!(
    messages,
    rooms,
    rooms_users,
    users,
);
