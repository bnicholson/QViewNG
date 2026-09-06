ALTER TABLE equipment DROP COLUMN last_modified_user;
ALTER TABLE equipment DROP COLUMN creator_id;
ALTER TABLE rosters DROP COLUMN last_modified_user;
ALTER TABLE games DROP COLUMN creator_id;
ALTER TABLE teams DROP COLUMN creator_id;
