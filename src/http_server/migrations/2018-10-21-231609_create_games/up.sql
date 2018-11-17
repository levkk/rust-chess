-- Your SQL goes here

CREATE TABLE games (
    id BIGSERIAL PRIMARY KEY,
    client_id BIGINT NOT NULL REFERENCES clients(id)
);