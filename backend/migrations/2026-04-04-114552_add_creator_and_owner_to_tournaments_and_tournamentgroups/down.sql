ALTER TABLE tournaments
    DROP COLUMN creator_id;

ALTER TABLE tournamentgroups
    DROP COLUMN owner_id,
    DROP COLUMN creator_id;
