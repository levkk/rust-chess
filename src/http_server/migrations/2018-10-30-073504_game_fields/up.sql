-- Your SQL goes here

ALTER TABLE games ADD COLUMN started_at TIMESTAMP NULL DEFAULT NOW();
ALTER TABLE games ADD COLUMN ended_at TIMESTAMP NULL;
