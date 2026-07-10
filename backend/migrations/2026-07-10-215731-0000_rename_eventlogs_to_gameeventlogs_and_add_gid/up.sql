-- Rename the eventlogs table to gameeventlogs and add a VARCHAR column
-- for holding a Rust UUID (stored as its string representation).
ALTER TABLE eventlogs RENAME TO gameeventlogs;
ALTER TABLE gameeventlogs ADD COLUMN gid VARCHAR(64) NOT NULL DEFAULT '';
