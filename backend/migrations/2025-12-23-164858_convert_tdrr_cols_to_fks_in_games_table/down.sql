
ALTER TABLE games
    ADD COLUMN tournament VARCHAR(48) NOT NULL,
    ADD COLUMN division VARCHAR(48) NOT NULL,
    ADD COLUMN room VARCHAR(48) NOT NULL,
    ADD COLUMN round VARCHAR(48) NOT NULL;

ALTER TABLE games
    DROP COLUMN tournamentid,
    DROP COLUMN divisionid,
    DROP COLUMN roomid,
    DROP COLUMN roundid;
