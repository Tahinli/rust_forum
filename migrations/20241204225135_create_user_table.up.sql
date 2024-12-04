-- Add up migration script here
CREATE TABLE IF NOT EXISTS "user"(
    id BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    surname VARCHAR(255) NOT NULL,
    gender boolean NOT NULL,
    birth_date DATE NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    role_id BIGSERIAL NOT NULL REFERENCES "role"(id)
);