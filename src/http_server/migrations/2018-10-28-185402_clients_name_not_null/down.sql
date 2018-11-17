-- This file should undo anything in `up.sql`

ALTER TABLE clients ALTER COLUMN name DROP NOT NULL;