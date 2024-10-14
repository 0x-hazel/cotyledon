CREATE TABLE IF NOT EXISTS tags
(
    id INTEGER PRIMARY KEY NOT NULL,
    tag text
);

CREATE TABLE IF NOT EXISTS postTags
(
    post_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (post_id, tag_id),
    FOREIGN KEY (post_id) REFERENCES posts (id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE -- This shouldn't happen
);