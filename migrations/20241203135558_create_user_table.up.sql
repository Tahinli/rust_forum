-- Add up migration script here
DROP TYPE IF EXISTS role;
CREATE TYPE role AS ENUM('Zero', 'Hero');
CREATE TABLE IF NOT EXISTS "user"(
            id BIGSERIAL PRIMARY KEY NOT NULL,
            name VARCHAR(255) NOT NULL,
            surname VARCHAR(255) NOT NULL,
            gender boolean NOT NULL,
            birth_date DATE NOT NULL,
            email VARCHAR(255) NOT NULL UNIQUE,
            role ROLE NOT NULL DEFAULT 'Zero'
);