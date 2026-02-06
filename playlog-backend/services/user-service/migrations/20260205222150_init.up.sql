CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS roles;
DROP TYPE IF EXISTS account_status;

CREATE TYPE account_status AS ENUM ('ACTIVE', 'BLOCKED', 'DEACTIVATED');

CREATE TABLE roles
(
    id   UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR NOT NULL UNIQUE
);

INSERT INTO roles (name)
VALUES ('USER'),
       ('MODERATOR'),
       ('ADMIN')
;

CREATE TABLE users
(
    id             UUID PRIMARY KEY        DEFAULT uuid_generate_v4(),
    username       VARCHAR        NOT NULL UNIQUE,
    email          VARCHAR        NOT NULL UNIQUE,
    password       VARCHAR        NOT NULL,
    account_status ACCOUNT_STATUS NOT NULL DEFAULT 'ACTIVE'
);

CREATE TABLE user_profiles
(
    user_id    UUID PRIMARY KEY REFERENCES users (id) ON DELETE CASCADE,
    first_name VARCHAR,
    last_name  VARCHAR,
    birthdate  DATE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE user_roles
(
    user_id UUID REFERENCES users (id) ON DELETE CASCADE,
    role_id UUID REFERENCES roles (id),
    PRIMARY KEY (user_id, role_id)
);

CREATE TABLE user_tokens
(
    id         UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id    UUID REFERENCES users (id) ON DELETE CASCADE,
    token      TEXT        NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL
);
