-- This file should undo anything in `up.sql`

ALTER TABLE clients ALTER COLUMN online DROP NOT NULL;