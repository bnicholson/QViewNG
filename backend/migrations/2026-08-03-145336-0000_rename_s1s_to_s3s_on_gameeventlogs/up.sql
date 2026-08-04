-- Rename the s1s (SHA1 checksum) column to s3s (SHA3-512 checksum) on gameeventlogs.
ALTER TABLE gameeventlogs RENAME COLUMN s1s TO s3s;
