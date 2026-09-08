-- Soft-delete flag: when true, the room is treated as deleted and excluded from reads.
-- "delete" sets this flag; "purge" removes the row entirely.
ALTER TABLE rooms ADD COLUMN del_fl BOOLEAN NOT NULL DEFAULT false;
