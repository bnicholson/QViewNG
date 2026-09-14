-- Revert the partial unique indexes back to full UNIQUE constraints. NOTE: this will fail if, while
-- the partial indexes were in effect, duplicate names were created among soft-deleted rows -- a full
-- constraint covers every row regardless of del_fl. Such duplicates must be resolved before rolling
-- back.

DROP INDEX uq_pool_brackets_division_session_id_name_active;
ALTER TABLE pool_brackets
    ADD CONSTRAINT uq_pool_brackets_division_session_id_name UNIQUE (division_session_id, name);

DROP INDEX uq_division_sessions_did_name_active;
ALTER TABLE division_sessions
    ADD CONSTRAINT uq_division_sessions_did_name UNIQUE (did, name);

DROP INDEX uq_divisions_tid_dname_active;
ALTER TABLE divisions
    ADD CONSTRAINT divisions_tid_dname_key UNIQUE (tid, dname);
