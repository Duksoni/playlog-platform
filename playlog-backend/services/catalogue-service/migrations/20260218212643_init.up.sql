CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE TABLE games
(
    id          SERIAL PRIMARY KEY,
    rawg_id     INTEGER UNIQUE,
    name        VARCHAR NOT NULL,
    description TEXT    NOT NULL,
    released    DATE, -- nullable: for games that haven't been released yet
    website     TEXT,
    draft       BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE platforms
(
    id      SERIAL PRIMARY KEY,
    rawg_id INTEGER UNIQUE,
    name    VARCHAR NOT NULL
);

CREATE TABLE genres
(
    id      SERIAL PRIMARY KEY,
    rawg_id INTEGER UNIQUE,
    name    VARCHAR NOT NULL
);

CREATE TABLE tags
(
    id      SERIAL PRIMARY KEY,
    rawg_id INTEGER UNIQUE,
    name    VARCHAR NOT NULL
);

CREATE TABLE developers
(
    id      SERIAL PRIMARY KEY,
    rawg_id INTEGER UNIQUE,
    name    VARCHAR NOT NULL
);

CREATE TABLE publishers
(
    id      SERIAL PRIMARY KEY,
    rawg_id INTEGER UNIQUE,
    name    VARCHAR NOT NULL
);

CREATE TABLE game_developers
(
    game_id      INT REFERENCES games (id) ON DELETE CASCADE,
    developer_id INT REFERENCES developers (id),
    PRIMARY KEY (game_id, developer_id)
);

CREATE TABLE game_publishers
(
    game_id      INT REFERENCES games (id) ON DELETE CASCADE,
    publisher_id INT REFERENCES publishers (id),
    PRIMARY KEY (game_id, publisher_id)
);

CREATE TABLE game_genres
(
    game_id  INT REFERENCES games (id) ON DELETE CASCADE,
    genre_id INT REFERENCES genres (id),
    PRIMARY KEY (game_id, genre_id)
);

CREATE TABLE game_platforms
(
    game_id     INT REFERENCES games (id) ON DELETE CASCADE,
    platform_id INT REFERENCES platforms (id),
    PRIMARY KEY (game_id, platform_id)
);

CREATE TABLE game_tags
(
    game_id INT REFERENCES games (id) ON DELETE CASCADE,
    tag_id  INT REFERENCES tags (id),
    PRIMARY KEY (game_id, tag_id)
);

-- Search by name (case-insensitive prefix/contains search) https://www.postgresql.org/docs/current/pgtrgm.html
CREATE INDEX idx_games_name ON games USING gin (name gin_trgm_ops);
CREATE INDEX idx_games_released ON games (released);
CREATE INDEX idx_games_draft ON games (draft);

CREATE INDEX idx_developers_name ON developers USING gin (name gin_trgm_ops);
CREATE INDEX idx_tags_name ON tags USING gin (name gin_trgm_ops);
CREATE INDEX idx_publishers_name ON publishers USING gin (name gin_trgm_ops);

CREATE INDEX idx_game_developers_developer_id ON game_developers (developer_id);
CREATE INDEX idx_game_publishers_publisher_id ON game_publishers (publisher_id);
CREATE INDEX idx_game_genres_genre_id ON game_genres (genre_id);
CREATE INDEX idx_game_platforms_platform_id ON game_platforms (platform_id);
CREATE INDEX idx_game_tags_tag_id ON game_tags (tag_id);