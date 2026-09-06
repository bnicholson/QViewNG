ALTER TABLE tournamentgroups DROP COLUMN last_modified_user;
ALTER TABLE games            DROP COLUMN last_modified_user;
ALTER TABLE teams            DROP COLUMN last_modified_user;
ALTER TABLE rounds           DROP COLUMN last_modified_user;
ALTER TABLE rooms            DROP COLUMN last_modified_user;
ALTER TABLE divisions        DROP COLUMN last_modified_user;
ALTER TABLE tournaments      DROP COLUMN last_modified_user;
