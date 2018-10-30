-- Your SQL goes here

CREATE TABLE clients (
    id BIGINT NOT NULL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    rank BIGINT
);