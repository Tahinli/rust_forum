-- Add up migration script here
CREATE TABLE IF NOT EXISTS "contact"(
    contact_id BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
    name VARCHAR(32) NOT NULL UNIQUE
);

INSERT INTO "contact"(contact_id, name) VALUES (0, 'Email') ON CONFLICT(contact_id) DO UPDATE SET "name" = 'Email';
