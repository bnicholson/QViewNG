-- Add a required "last modified by" user FK to each domain table.
--
-- To stay safe against a database that already holds rows (e.g. a persistent test DB), the
-- column is first added nullable, backfilled from the row's most sensible existing owner, then
-- constrained NOT NULL with the foreign key added last.

-- tournaments: attribute to the tournament owner.
ALTER TABLE tournaments ADD COLUMN last_modified_user UUID;
UPDATE tournaments SET last_modified_user = owner_id WHERE last_modified_user IS NULL;
ALTER TABLE tournaments ALTER COLUMN last_modified_user SET NOT NULL;
ALTER TABLE tournaments ADD CONSTRAINT tournaments_last_modified_user_fkey FOREIGN KEY (last_modified_user) REFERENCES users(id);

-- tournamentgroups: attribute to the group owner.
ALTER TABLE tournamentgroups ADD COLUMN last_modified_user UUID;
UPDATE tournamentgroups SET last_modified_user = owner_id WHERE last_modified_user IS NULL;
ALTER TABLE tournamentgroups ALTER COLUMN last_modified_user SET NOT NULL;
ALTER TABLE tournamentgroups ADD CONSTRAINT tournamentgroups_last_modified_user_fkey FOREIGN KEY (last_modified_user) REFERENCES users(id);

-- teams: attribute to the coach.
ALTER TABLE teams ADD COLUMN last_modified_user UUID;
UPDATE teams SET last_modified_user = coachid WHERE last_modified_user IS NULL;
ALTER TABLE teams ALTER COLUMN last_modified_user SET NOT NULL;
ALTER TABLE teams ADD CONSTRAINT teams_last_modified_user_fkey FOREIGN KEY (last_modified_user) REFERENCES users(id);

-- rooms: no direct user column; attribute to the parent tournament's owner.
ALTER TABLE rooms ADD COLUMN last_modified_user UUID;
UPDATE rooms SET last_modified_user = t.owner_id FROM tournaments t WHERE rooms.tid = t.tid AND rooms.last_modified_user IS NULL;
ALTER TABLE rooms ALTER COLUMN last_modified_user SET NOT NULL;
ALTER TABLE rooms ADD CONSTRAINT rooms_last_modified_user_fkey FOREIGN KEY (last_modified_user) REFERENCES users(id);

-- games: no direct user column; attribute to the parent room's owner (which may/may not have gotten it from the tournament).
ALTER TABLE games ADD COLUMN last_modified_user UUID;
UPDATE games SET last_modified_user = rm.last_modified_user FROM rooms rm WHERE games.roomid = rm.roomid AND games.last_modified_user IS NULL; 
ALTER TABLE games ALTER COLUMN last_modified_user SET NOT NULL;
ALTER TABLE games ADD CONSTRAINT games_last_modified_user_fkey FOREIGN KEY (last_modified_user) REFERENCES users(id);

-- divisions: no direct user column; attribute to the parent tournament's owner.
ALTER TABLE divisions ADD COLUMN last_modified_user UUID;
UPDATE divisions SET last_modified_user = t.owner_id FROM tournaments t WHERE divisions.tid = t.tid AND divisions.last_modified_user IS NULL;
ALTER TABLE divisions ALTER COLUMN last_modified_user SET NOT NULL;
ALTER TABLE divisions ADD CONSTRAINT divisions_last_modified_user_fkey FOREIGN KEY (last_modified_user) REFERENCES users(id);

-- rounds: no direct user column; attribute via division -> parent tournament's owner.
ALTER TABLE rounds ADD COLUMN last_modified_user UUID;
UPDATE rounds SET last_modified_user = t.owner_id FROM divisions d JOIN tournaments t ON d.tid = t.tid WHERE rounds.did = d.did AND rounds.last_modified_user IS NULL;
ALTER TABLE rounds ALTER COLUMN last_modified_user SET NOT NULL;
ALTER TABLE rounds ADD CONSTRAINT rounds_last_modified_user_fkey FOREIGN KEY (last_modified_user) REFERENCES users(id);
