-- This file should undo anything in `up.sql`

ALTER TABLE games DROP COLUMN ended_at;
ALTER TABLE games DROP COLUMN started_at;
