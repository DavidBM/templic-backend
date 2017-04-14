CREATE TABLE authors (
    id              integer NOT NULL PRIMARY KEY,
    author_id       integer NOT NULL,
    user_id         integer NOT NULL REFERENCES users(id)
);

CREATE UNIQUE INDEX authors_author_id_key ON authors (author_id);
CREATE UNIQUE INDEX authors_user_id_key ON authors (user_id);

CREATE SEQUENCE authors_author_id_seq;
ALTER TABLE authors ALTER COLUMN author_id SET DEFAULT nextval('authors_author_id_seq');

ALTER SEQUENCE authors_author_id_seq OWNED BY authors.author_id;