-- This file should undo anything in `up.sql`
ALTER TABLE rooms DROP COLUMN ping_jobspending;
ALTER TABLE rooms DROP COLUMN ping_client_ts;
ALTER TABLE rooms DROP COLUMN ping_qm_version;
ALTER TABLE rooms DROP COLUMN ping_question_number;
