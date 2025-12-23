
ALTER TABLE games
    DROP COLUMN tournament,
    DROP COLUMN division,
    DROP COLUMN room,
    DROP COLUMN round;

ALTER TABLE games
    ADD COLUMN tournamentid BIGINT REFERENCES tournaments(tid),
    ADD COLUMN divisionid BIGINT REFERENCES divisions(did),
    ADD COLUMN roomid BIGINT REFERENCES rooms(roomid),
    ADD COLUMN roundid BIGINT REFERENCES rounds(roundid);
