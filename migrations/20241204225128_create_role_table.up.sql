-- Add up migration script here
CREATE TABLE IF NOT EXISTS "role"(
    id BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
    name VARCHAR(16) NOT NULL UNIQUE
);

INSERT INTO "role"(id, name) VALUES (0, 'Ahmet Kaan Gümüş');
INSERT INTO "role"(id, name) VALUES (1, 'Founder');
INSERT INTO "role"(id, name) VALUES (2, 'Normal');