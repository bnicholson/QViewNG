ALTER TABLE games DROP CONSTRAINT games_org_tournament_room_round_clientkey_key;

-- Re-add the column (nullable). Original values / FK / unique constraint cannot be reconstructed.
ALTER TABLE games ADD COLUMN divisionid UUID;
