
ALTER TABLE rooms
    DROP COLUMN quizmaster,
    DROP COLUMN contentjudge;

ALTER TABLE games
    ADD COLUMN quizmasterid BIGINT REFERENCES users(id),
    ADD COLUMN contentjudgeid BIGINT REFERENCES users(id);
