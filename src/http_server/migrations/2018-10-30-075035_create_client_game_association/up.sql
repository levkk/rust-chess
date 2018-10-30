-- Your SQL goes here

CREATE TABLE client_games (
    id BIGSERIAL PRIMARY KEY,
    client_id BIGINT NOT NULL REFERENCES clients(id),
    game_id BIGINT NOT NULL REFERENCES games(id)
);