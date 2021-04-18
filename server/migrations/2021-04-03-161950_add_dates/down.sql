ALTER TABLE users
    DROP created,
    DROP updated;

ALTER TABLE rooms
    DROP created,
    DROP updated;

ALTER TABLE rooms_users
    DROP COLUMN created,
    DROP COLUMN updated;

DROP FUNCTION last_upd_trig CASCADE; -- Also removes triggers
