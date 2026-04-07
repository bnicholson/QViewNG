ALTER TABLE rooms
    DROP COLUMN IF EXISTS quizmaster_id,
    DROP COLUMN IF EXISTS contentjudge_id;
