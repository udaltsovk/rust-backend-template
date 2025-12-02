DO
$$
    BEGIN
        IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'client_gender') THEN
            CREATE TYPE client_gender AS ENUM ('male', 'female');
        END IF;
    END
$$;

CREATE TABLE IF NOT EXISTS clients
(
    id       uuid          NOT NULL PRIMARY KEY,
    login    varchar(32)   NOT NULL CHECK ( length(login) >= 3 ),
    age      int           NOT NULL CHECK ( age >= 0 AND age <= 255 ),
    gender   client_gender NOT NULL,
    location varchar(100)  NOT NULL CHECK ( length(location) >= 5 )
);
