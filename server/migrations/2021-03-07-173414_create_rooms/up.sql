CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "rooms"
(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR
);

CREATE TABLE "rooms_users"
(
    user_id UUID,
    room_id UUID,
    status VARCHAR,
    PRIMARY KEY (user_id, room_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (room_id) REFERENCES rooms(id) ON DELETE CASCADE
);
