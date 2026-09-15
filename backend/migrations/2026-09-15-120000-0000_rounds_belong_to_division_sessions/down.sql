-- Revert: rounds belong to a division again (backfilled from their session's division).
DROP INDEX uq_rounds_division_session_id_name_active;
ALTER TABLE rounds ADD COLUMN did UUID REFERENCES divisions(did);
UPDATE rounds r
SET did = (SELECT ds.did FROM division_sessions ds WHERE ds.division_session_id = r.division_session_id);
ALTER TABLE rounds ALTER COLUMN did SET NOT NULL;
ALTER TABLE rounds ADD CONSTRAINT rounds_name_did_unique UNIQUE (name, did);
ALTER TABLE rounds DROP COLUMN division_session_id;
