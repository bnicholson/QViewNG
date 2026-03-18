
ALTER TABLE users DROP COLUMN username;
ALTER TABLE users ADD COLUMN username character varying(32);

ALTER TABLE users DROP COLUMN hash_password;
ALTER TABLE users ADD COLUMN hash_password text;