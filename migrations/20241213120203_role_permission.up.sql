-- Add up migration script here
CREATE TABLE IF NOT EXISTS "role_permission"(
    role_id BIGSERIAL NOT NULL REFERENCES "role"(id),
    permission_id BIGSERIAL NOT NULL REFERENCES "permission"(id),
    PRIMARY KEY (role_id, permission_id)
);