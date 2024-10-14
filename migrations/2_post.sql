CREATE TABLE IF NOT EXISTS posts
(
    id integer PRIMARY KEY NOT NULL,
    user_id integer,
    thread text,
    created text NOT NULL DEFAULT CURRENT_TIMESTAMP,
    summary text,
    body text NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE SET NULL
);