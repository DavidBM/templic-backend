DROP INDEX users_user_id_key;
ALTER TABLE users DROP COLUMN user_id;
DROP SEQUENCE users_user_id_seq;