-- Widen the s3s (SHA3-512 checksum) column to hold up to 128 characters.
ALTER TABLE gameeventlogs ALTER COLUMN s3s TYPE VARCHAR(128);
