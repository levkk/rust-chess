-- Your SQL goes here

ALTER TABLE clients ADD COLUMN online boolean DEFAULT false;
ALTER TABLE clients ADD COLUMN last_login TIMESTAMP NULL;