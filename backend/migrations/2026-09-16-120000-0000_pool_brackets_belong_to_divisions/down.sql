-- Revert: pool brackets belong to a division session again.
ALTER TABLE pool_brackets ADD COLUMN division_session_id UUID REFERENCES division_sessions(division_session_id);

-- Backfill: attach each bracket to the earliest active session of its division. A bracket whose
-- division has no session cannot be mapped back and is removed.
UPDATE pool_brackets pb
   SET division_session_id = (
       SELECT ds.division_session_id
         FROM division_sessions ds
        WHERE ds.did = pb.divisionid AND ds.del_fl = false
        ORDER BY ds.created_date
        LIMIT 1
   );
DELETE FROM pool_brackets WHERE division_session_id IS NULL;

ALTER TABLE pool_brackets ALTER COLUMN division_session_id SET NOT NULL;

DROP INDEX uq_pool_brackets_divisionid_name_active;
ALTER TABLE pool_brackets DROP COLUMN divisionid;

CREATE UNIQUE INDEX uq_pool_brackets_division_session_id_name_active
    ON pool_brackets (division_session_id, name) WHERE del_fl = false;
