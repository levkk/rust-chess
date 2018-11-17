-- This file should undo anything in `up.sql`

ALTER TABLE games ADD COLUMN client_id BIGINT NOT NULL REFERENCES clients(id);