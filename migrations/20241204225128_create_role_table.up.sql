-- Add up migration script here
CREATE TABLE IF NOT EXISTS "role"(
    id BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
    name VARCHAR(16) NOT NULL UNIQUE
);

INSERT INTO "role"(id, name) VALUES (0, 'Builder') ON CONFLICT(id) DO UPDATE SET "name" = 'Builder';
INSERT INTO "role"(id, name) VALUES (1, 'Admin') ON CONFLICT(id) DO UPDATE SET "name" = 'Admin';
INSERT INTO "role"(id, name) VALUES (10, 'Normal') ON CONFLICT(id) DO UPDATE SET "name" = 'Normal';
INSERT INTO "role"(id, name) VALUES (-1, 'Banned') ON CONFLICT(id) DO UPDATE SET "name" = 'Banned';
