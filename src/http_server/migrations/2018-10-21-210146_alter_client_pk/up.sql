-- Your SQL goes here

DROP TABLE IF EXISTS clients;

CREATE TABLE clients (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE,
    rank BIGINT
);