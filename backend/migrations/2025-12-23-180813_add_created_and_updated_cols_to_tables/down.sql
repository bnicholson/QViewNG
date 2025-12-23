
ALTER TABLE statsgroups
    DROP COLUMN created_at,
    DROP COLUMN updated_at;

ALTER TABLE games_statsgroups
    DROP COLUMN created_at;

ALTER TABLE teams
    DROP COLUMN created_at,
    DROP COLUMN updated_at;

ALTER TABLE rooms
    DROP COLUMN created_at,
    DROP COLUMN updated_at;

ALTER TABLE games
    DROP COLUMN created_at,
    DROP COLUMN updated_at;
