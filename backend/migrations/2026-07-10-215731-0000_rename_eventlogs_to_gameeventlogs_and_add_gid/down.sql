-- This file should undo anything in `up.sql`
ALTER TABLE gameeventlogs DROP COLUMN gid;
ALTER TABLE gameeventlogs RENAME TO eventlogs;
