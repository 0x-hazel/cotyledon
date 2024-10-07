CREATE TABLE IF NOT EXISTS posts
(
    id integer PRIMARY KEY NOT NULL,
    user_id integer,
    responding integer,
    created text NOT NULL DEFAULT CURRENT_TIMESTAMP,
    body text NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE SET NULL,
    FOREIGN KEY (responding) REFERENCES posts (id)
);