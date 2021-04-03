ALTER TABLE users
    DROP created,
    DROP updated;

ALTER TABLE rooms
    DROP created,
    DROP updated;

DROP FUNCTION last_upd_trig CASCADE; -- Also removes triggers