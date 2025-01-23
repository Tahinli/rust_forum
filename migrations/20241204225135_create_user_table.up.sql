-- Add up migration script here
CREATE TABLE IF NOT EXISTS "user"(
    user_id BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
    name VARCHAR(256) NOT NULL,
    surname VARCHAR(256) NOT NULL,
    gender BOOLEAN NOT NULL,
    birth_date DATE NOT NULL,
    role_id BIGINT NOT NULL REFERENCES "role" DEFAULT 10,
    creation_time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
