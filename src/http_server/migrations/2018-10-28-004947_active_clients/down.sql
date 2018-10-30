-- This file should undo anything in `up.sql`

ALTER TABLE clients DROP COLUMN online;
ALTER TABLE clients DROP COLUMN last_login;