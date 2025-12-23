
ALTER TABLE rooms
    ADD COLUMN quizmaster VARCHAR(64),
    ADD COLUMN contentjudge VARCHAR(64);

ALTER TABLE games
    DROP COLUMN quizmasterid,
    DROP COLUMN contentjudgeid;
