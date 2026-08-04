-- This file should undo anything in `up.sql`
ALTER TABLE gameeventlogs ALTER COLUMN s3s TYPE VARCHAR(32);
