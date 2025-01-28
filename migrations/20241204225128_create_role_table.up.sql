-- Add up migration script here
CREATE TABLE IF NOT EXISTS "role"(
    role_id BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
    name VARCHAR(16) NOT NULL UNIQUE
);

INSERT INTO "role"(role_id, name) VALUES (0, 'Builder') ON CONFLICT(role_id) DO UPDATE SET "name" = 'Builder';
INSERT INTO "role"(role_id, name) VALUES (1, 'Admin') ON CONFLICT(role_id) DO UPDATE SET "name" = 'Admin';
INSERT INTO "role"(role_id, name) VALUES (10, 'Normal') ON CONFLICT(role_id) DO UPDATE SET "name" = 'Normal';
INSERT INTO "role"(role_id, name) VALUES (-1, 'Banned') ON CONFLICT(role_id) DO UPDATE SET "name" = 'Banned';
