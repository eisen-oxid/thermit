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

joinable!(rooms_users -> rooms (room_id));
joinable!(rooms_users -> users (user_id));

allow_tables_to_appear_in_same_query!(
    rooms,
    rooms_users,
    users,
);
