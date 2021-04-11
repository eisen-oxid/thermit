ALTER TABLE users
    ADD created timestamp NOT NULL default current_timestamp,
    ADD updated timestamp NOT NULL default current_timestamp;

ALTER TABLE rooms
    ADD created timestamp NOT NULL default current_timestamp,
    ADD updated timestamp NOT NULL default current_timestamp;

ALTER TABLE rooms_users
    ADD created timestamp NOT NULL default current_timestamp,
    ADD updated timestamp NOT NULL default current_timestamp;

CREATE FUNCTION last_upd_trig() RETURNS trigger
    LANGUAGE plpgsql AS
$$BEGIN
    NEW.updated := current_timestamp;
    RETURN NEW;
END;$$;

CREATE TRIGGER user_update_date_trigger
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE PROCEDURE last_upd_trig();

CREATE TRIGGER room_update_date_trigger
    BEFORE UPDATE ON rooms
    FOR EACH ROW
    EXECUTE PROCEDURE last_upd_trig();

CREATE TRIGGER rooms_users_updates_update_date_trigger
    BEFORE UPDATE ON rooms_users
    FOR EACH ROW
    EXECUTE PROCEDURE last_upd_trig();
