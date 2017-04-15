CREATE TABLE posts (
    id              integer NOT NULL PRIMARY KEY,
    user_id         integer NOT NULL REFERENCES users(id),
    title           text NOT NULL,
    content         text NOT NULL
);

CREATE INDEX posts_user_id_key ON posts (user_id);
