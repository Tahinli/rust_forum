-- Add up migration script here
CREATE TABLE IF NOT EXISTS "user"(
    user_id BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
    name VARCHAR(256) NOT NULL,
    surname VARCHAR(256) NOT NULL,
    gender BOOLEAN NOT NULL,
    birth_date DATE NOT NULL,
    role_id BIGINT NOT NULL REFERENCES "role"(role_id) DEFAULT 10,
    creation_time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO "user"(user_id, name, surname, gender, birth_date, role_id)
VALUES (0, 'Builder', 'Builder', true, NOW(), 0)
ON CONFLICT(user_id) DO UPDATE SET
"name" = 'Builder',
"surname" = 'Builder',
"gender" = true,
"birth_date" = NOW(),
"role_id" = 0;
