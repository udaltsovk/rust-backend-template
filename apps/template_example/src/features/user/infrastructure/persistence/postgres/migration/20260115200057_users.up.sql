DO
$$
    BEGIN
        IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'user_target_settings') THEN
            CREATE TYPE user_target_settings AS
            (
                age     smallint,
                country text
            );
        END IF;
    END
$$;


CREATE TABLE IF NOT EXISTS users
(
    id              uuid                 NOT NULL PRIMARY KEY,
    name            text                 NOT NULL,
    surname         text                 NOT NULL,
    email           text                 NOT NULL,
    password_hash   text                 NOT NULL,
    avatar_url      text,
    target_settings user_target_settings NOT NULL
);
