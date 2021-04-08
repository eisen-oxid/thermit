CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- by default the postgres ENUM values correspond to snake_cased Rust enum variant names
CREATE TYPE message_encryption AS ENUM ('clear');

CREATE TABLE "messages"
(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    room_id UUID NOT NULL,
    author UUID NOT NULL,
    FOREIGN KEY (room_id) REFERENCES rooms(id) ON DELETE CASCADE,
    FOREIGN KEY (author) REFERENCES users(id) ON DELETE CASCADE,
    content VARCHAR NOT NULL,
    encryption message_encryption NOT NULL,
    created timestamp NOT NULL default current_timestamp,
    updated timestamp NOT NULL default current_timestamp
);

CREATE TRIGGER message_update_date_trigger
    BEFORE UPDATE ON messages
    FOR EACH ROW
EXECUTE PROCEDURE last_upd_trig();
