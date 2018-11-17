-- This file should undo anything in `up.sql`

DROP TABLE IF EXISTS clients;

CREATE TABLE clients (
    id BIGINT NOT NULL PRIMARY KEY,
    name VARCHAR(255) UNIQUE,
    rank BIGINT
);