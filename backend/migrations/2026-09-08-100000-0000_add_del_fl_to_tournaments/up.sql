-- Soft-delete flag: when true, the tournament is treated as deleted and excluded from reads.
-- Hard deletion ("purge") removes the row entirely; "delete" now just sets this flag.
ALTER TABLE tournaments ADD COLUMN del_fl BOOLEAN NOT NULL DEFAULT false;
