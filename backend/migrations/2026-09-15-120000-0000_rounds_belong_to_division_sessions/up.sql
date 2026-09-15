-- Rounds are time-bound and now belong to a (time-bound) division session instead of a division.
-- Backfill each existing round to its division's earliest session; rounds whose division has no
-- session cannot be mapped and are removed (safe in this environment: no games reference them).
ALTER TABLE rounds ADD COLUMN division_session_id UUID REFERENCES division_sessions(division_session_id);

UPDATE rounds r
SET division_session_id = (
    SELECT ds.division_session_id
    FROM division_sessions ds
    WHERE ds.did = r.did
    ORDER BY ds.created_date ASC, ds.division_session_id ASC
    LIMIT 1
);

DELETE FROM rounds WHERE division_session_id IS NULL;

ALTER TABLE rounds ALTER COLUMN division_session_id SET NOT NULL;

-- Dropping `did` removes rounds_did_fkey and the old UNIQUE(name, did). Round names are unique within
-- their session among active (non-deleted) rounds.
ALTER TABLE rounds DROP COLUMN did;
CREATE UNIQUE INDEX uq_rounds_division_session_id_name_active
    ON rounds (division_session_id, name) WHERE del_fl = false;
