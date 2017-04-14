ALTER TABLE users ADD COLUMN user_id integer;
UPDATE users SET user_id = id;
CREATE UNIQUE INDEX users_user_id_key ON users (user_id);
ALTER TABLE users ALTER COLUMN user_id SET NOT NULL;
CREATE SEQUENCE users_user_id_seq;
ALTER TABLE users ALTER COLUMN user_id SET DEFAULT nextval('users_user_id_seq');

