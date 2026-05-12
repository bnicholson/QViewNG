ALTER TABLE rounds ADD COLUMN name varchar(64) NOT NULL DEFAULT '';
ALTER TABLE rounds ADD CONSTRAINT rounds_name_did_unique UNIQUE (name, did);
