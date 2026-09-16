-- Reverse the roundgroups rename back to division_sessions.
ALTER INDEX uq_roundgroups_did_name_active RENAME TO uq_division_sessions_did_name_active;
ALTER TABLE rounds RENAME COLUMN roundgroup_id TO division_session_id;
ALTER TABLE roundgroups RENAME COLUMN roundgroup_id TO division_session_id;
ALTER TABLE roundgroups RENAME TO division_sessions;
