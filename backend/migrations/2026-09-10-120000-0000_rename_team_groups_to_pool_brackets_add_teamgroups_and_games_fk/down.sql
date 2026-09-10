ALTER TABLE games DROP COLUMN poolbracket_id;

ALTER TABLE team_teamgroups DROP CONSTRAINT team_teamgroups_team_group_id_fkey;
DROP TABLE teamgroups;

ALTER TABLE pool_brackets
    RENAME CONSTRAINT uq_pool_brackets_division_session_id_name TO uq_team_groups_division_session_id_name;
ALTER TABLE pool_brackets RENAME COLUMN pool_bracket_id TO team_group_id;
ALTER TABLE pool_brackets RENAME TO team_groups;

ALTER TABLE team_teamgroups
    ADD CONSTRAINT team_teamgroups_team_group_id_fkey
    FOREIGN KEY (team_group_id) REFERENCES team_groups(team_group_id);
