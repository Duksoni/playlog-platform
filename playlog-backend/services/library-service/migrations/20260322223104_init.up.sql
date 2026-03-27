CREATE TYPE game_library_status AS ENUM (
    'OWNED',
    'PLAYING',
    'WISHLIST',
    'COMPLETED',
    'DROPPED'
    );

CREATE TABLE user_games
(
    user_id      UUID           NOT NULL,
    game_id      INT            NOT NULL,
    status       game_library_status NOT NULL DEFAULT 'OWNED',
    added_at     TIMESTAMPTZ    NOT NULL DEFAULT now(),
    last_updated TIMESTAMPTZ    NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id, game_id)
);

CREATE INDEX idx_user_games_user_id ON user_games (user_id);
CREATE INDEX idx_user_games_status ON user_games (status);
