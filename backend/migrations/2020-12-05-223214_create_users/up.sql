-- Your SQL goes here
CREATE TABLE users
(
    id         SERIAL PRIMARY KEY,
    eth_key    CHAR(42) UNIQUE NOT NULL,
    first_name TEXT,
    last_name  TEXT
)