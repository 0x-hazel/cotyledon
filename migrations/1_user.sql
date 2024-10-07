-- Create users table.
CREATE TABLE IF NOT EXISTS users
(
    id integer PRIMARY KEY NOT NULL,
    username text NOT NULL UNIQUE,
    email text NOT NULL UNIQUE,
    password text NOT NULL,
    bio text NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS following
(
    follower integer NOT NULL,
    followee integer NOT NULL,
    is_accepted boolean NOT NULL CHECK (accepted in (0, 1)),
    requested text NOT NULL DEFAULT CURRENT_TIMESTAMP,
    accepted text DEFAULT NULL,
    FOREIGN KEY (follower) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (followee) REFERENCES users (id) ON DELETE CASCADE,
    PRIMARY KEY (follower, followee)
);

-- Insert "ferris" user.
INSERT INTO users (id, username, email, password, bio)
VALUES (1, 'ferris', 'ferris@example.org', '$argon2id$v=19$m=19456,t=2,p=1$VE0e3g7DalWHgDwou3nuRA$uC6TER156UQpk0lNQ5+jHM0l5poVjPA1he/Tyn9J4Zw', 'Lorem ipsum dolor sit amet');