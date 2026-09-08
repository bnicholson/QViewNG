-- Soft-delete flag: when true, the round is treated as deleted and excluded from reads.
-- "delete" sets this flag; "purge" removes the row entirely.
ALTER TABLE rounds ADD COLUMN del_fl BOOLEAN NOT NULL DEFAULT false;
