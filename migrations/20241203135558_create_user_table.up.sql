-- Add up migration script here
CREATE TABLE IF NOT EXISTS "role"(
    id BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
    name VARCHAR(16) NOT NULL UNIQUE
);

INSERT INTO "role"(id, name) VALUES (0, 'Ahmet Kaan Gümüş');
INSERT INTO "role"(id, name) VALUES (1, 'Founder');
INSERT INTO "role"(id, name) VALUES (2, 'Normal');

CREATE TABLE IF NOT EXISTS "user"(
    id BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    surname VARCHAR(255) NOT NULL,
    gender boolean NOT NULL,
    birth_date DATE NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    role_id BIGSERIAL NOT NULL REFERENCES "role"(id)
);