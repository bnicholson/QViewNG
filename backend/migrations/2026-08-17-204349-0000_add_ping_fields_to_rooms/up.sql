-- Latest ping data reported by a room's QuizMachine client. All nullable (blank by default).
ALTER TABLE rooms ADD COLUMN ping_question_number INTEGER;
ALTER TABLE rooms ADD COLUMN ping_qm_version VARCHAR(32);
ALTER TABLE rooms ADD COLUMN ping_client_ts TIMESTAMPTZ;
ALTER TABLE rooms ADD COLUMN ping_jobspending INTEGER;
