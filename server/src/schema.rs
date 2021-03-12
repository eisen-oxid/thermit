table! {
    rooms (id) {
        id -> Uuid,
        name -> Nullable<Varchar>,
    }
}

table! {
    rooms_users (room_id, user_id) {
        user_id -> Uuid,
        room_id -> Uuid,
        status -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        password -> Varchar,
    }
}

joinable!(rooms_users -> rooms (room_id));
joinable!(rooms_users -> users (user_id));

allow_tables_to_appear_in_same_query!(rooms, rooms_users, users,);
