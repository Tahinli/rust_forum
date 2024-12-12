-- Add up migration script here
CREATE TABLE IF NOT EXISTS "role"(
    id BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
    name VARCHAR(16) NOT NULL UNIQUE
);

INSERT INTO "role"(id, name) VALUES (0, 'Ahmet Kaan Gümüş') ON CONFLICT(id) DO UPDATE SET "name" = 'Ahmet Kaan Gümüş';
INSERT INTO "role"(id, name) VALUES (1, 'Founder') ON CONFLICT(id) DO UPDATE SET "name" = 'Founder';
INSERT INTO "role"(id, name) VALUES (2, 'Maintainer') ON CONFLICT(id) DO UPDATE SET "name" = 'Founder';
INSERT INTO "role"(id, name) VALUES (3, 'Admin') ON CONFLICT(id) DO UPDATE SET "name" = 'Admin';
INSERT INTO "role"(id, name) VALUES (10, 'Normal') ON CONFLICT(id) DO UPDATE SET "name" = 'Normal';
INSERT INTO "role"(id, name) VALUES (-1, 'Banned') ON CONFLICT(id) DO UPDATE SET "name" = 'Banned';