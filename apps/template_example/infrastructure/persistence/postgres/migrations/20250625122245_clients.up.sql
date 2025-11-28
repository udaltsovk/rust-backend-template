CREATE TYPE client_gender AS ENUM ('male', 'female');

CREATE TABLE IF NOT EXISTS clients
(
    id       uuid        NOT NULL PRIMARY KEY,
    login    varchar(32) NOT NULL,
    age      int         NOT NULL CHECK ( age > 0 ),
    gender   client_gender NOT NULL,
    location text        NOT NULL
);