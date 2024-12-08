-- Add up migration script here
CREATE TABLE IF NOT EXISTS "user"(
    id BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    surname VARCHAR(255) NOT NULL,
    gender boolean NOT NULL,
    birth_date DATE NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    role_id BIGSERIAL NOT NULL REFERENCES "role"(id),
    creation_time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS "permission_user"(
    role_id BIGSERIAL PRIMARY KEY NOT NULL UNIQUE REFERENCES "role"(id),
    create_self BOOLEAN NOT NULL,
    read_self BOOLEAN NOT NULL,
    update_self BOOLEAN NOT NULL,
    delete_self BOOLEAN NOT NULL,
    create_other BOOLEAN NOT NULL,
    read_other BOOLEAN NOT NULL,
    update_other BOOLEAN NOT NULL,
    delete_other BOOLEAN NOT NULL,
    create_lower BOOLEAN NOT NULL,
    read_lower BOOLEAN NOT NULL,
    update_lower BOOLEAN NOT NULL,
    delete_lower BOOLEAN NOT NULL
);

INSERT INTO "permission_user"(role_id, create_self, read_self, update_self, delete_self, create_other, read_other, update_other, delete_other, create_lower, read_lower, update_lower, delete_lower) 
VALUES (0, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE) 
ON CONFLICT(role_id) DO UPDATE SET 
"create_self" = TRUE, "read_self" = TRUE, "update_self" = TRUE, "delete_self" = TRUE, 
"create_other" = TRUE, "read_other" = TRUE, "update_other" = TRUE, "delete_other" = TRUE, 
"create_lower" = TRUE, "read_lower" = TRUE, "update_lower" = TRUE, "delete_lower" = TRUE;

INSERT INTO "permission_user"(role_id, create_self, read_self, update_self, delete_self, create_other, read_other, update_other, delete_other, create_lower, read_lower, update_lower, delete_lower) 
VALUES (1, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE) 
ON CONFLICT(role_id) DO UPDATE SET 
"create_self" = TRUE, "read_self" = TRUE, "update_self" = TRUE, "delete_self" = TRUE, 
"create_other" = TRUE, "read_other" = TRUE, "update_other" = TRUE, "delete_other" = TRUE, 
"create_lower" = TRUE, "read_lower" = TRUE, "update_lower" = TRUE, "delete_lower" = TRUE;

INSERT INTO "permission_role"(role_id, create_self, read_self, update_self, delete_self, create_other, read_other, update_other, delete_other, create_lower, read_lower, update_lower, delete_lower) 
VALUES (2, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE) 
ON CONFLICT(role_id) DO UPDATE SET 
"create_self" = TRUE, "read_self" = TRUE, "update_self" = TRUE, "delete_self" = TRUE, 
"create_other" = TRUE, "read_other" = TRUE, "update_other" = TRUE, "delete_other" = TRUE, 
"create_lower" = TRUE, "read_lower" = TRUE, "update_lower" = TRUE, "delete_lower" = TRUE;

INSERT INTO "permission_user"(role_id, create_self, read_self, update_self, delete_self, create_other, read_other, update_other, delete_other, create_lower, read_lower, update_lower, delete_lower) 
VALUES (3, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE) 
ON CONFLICT(role_id) DO UPDATE SET 
"create_self" = TRUE, "read_self" = TRUE, "update_self" = TRUE, "delete_self" = TRUE, 
"create_other" = TRUE, "read_other" = TRUE, "update_other" = TRUE, "delete_other" = TRUE, 
"create_lower" = TRUE, "read_lower" = TRUE, "update_lower" = TRUE, "delete_lower" = TRUE;

INSERT INTO "permission_user"(role_id, create_self, read_self, update_self, delete_self, create_other, read_other, update_other, delete_other, create_lower, read_lower, update_lower, delete_lower) 
VALUES (10, FALSE, TRUE, FALSE, FALSE, FALSE, TRUE, FALSE, FALSE, FALSE, TRUE, FALSE, FALSE) 
ON CONFLICT(role_id) DO UPDATE SET 
"create_self" = FALSE, "read_self" = TRUE, "update_self" = FALSE, "delete_self" = FALSE, 
"create_other" = FALSE, "read_other" = TRUE, "update_other" = FALSE, "delete_other" = FALSE, 
"create_lower" = FALSE, "read_lower" = TRUE, "update_lower" = FALSE, "delete_lower" = FALSE;

INSERT INTO "permission_user"(role_id, create_self, read_self, update_self, delete_self, create_other, read_other, update_other, delete_other, create_lower, read_lower, update_lower, delete_lower) 
VALUES (-1, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE) 
ON CONFLICT(role_id) DO UPDATE SET 
"create_self" = FALSE, "read_self" = FALSE, "update_self" = FALSE, "delete_self" = FALSE, 
"create_other" = FALSE, "read_other" = FALSE, "update_other" = FALSE, "delete_other" = FALSE, 
"create_lower" = FALSE, "read_lower" = FALSE, "update_lower" = FALSE, "delete_lower" = FALSE;