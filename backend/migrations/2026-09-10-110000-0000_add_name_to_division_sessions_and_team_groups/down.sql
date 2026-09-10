ALTER TABLE team_groups DROP CONSTRAINT uq_team_groups_division_session_id_name;
ALTER TABLE team_groups DROP COLUMN name;

ALTER TABLE division_sessions DROP CONSTRAINT uq_division_sessions_did_name;
ALTER TABLE division_sessions DROP COLUMN name;
