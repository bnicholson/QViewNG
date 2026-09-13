-- Add a soft-delete flag to division_sessions, pool_brackets, and teamgroups.
ALTER TABLE division_sessions ADD COLUMN del_fl BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE pool_brackets ADD COLUMN del_fl BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE teamgroups ADD COLUMN del_fl BOOLEAN NOT NULL DEFAULT FALSE;
