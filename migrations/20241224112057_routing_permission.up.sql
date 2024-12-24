-- Add up migration script here
CREATE TABLE IF NOT EXISTS "routing_permission"(
    routing_id BIGSERIAL NOT NULL REFERENCES "routing"(id),
    permission_id BIGSERIAL NOT NULL REFERENCES "permission"(id),
    PRIMARY KEY (routing_id, permission_id)
);