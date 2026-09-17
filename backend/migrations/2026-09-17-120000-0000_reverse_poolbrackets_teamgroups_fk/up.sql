-- Reverse the 1:1 between pool_brackets and teamgroups. The FK (and its UNIQUE constraint) moves
-- from teamgroups.pool_bracket_id onto pool_brackets.team_group_id. A pool bracket's team group is
-- created lazily (on first team placement), so the new column is nullable; UNIQUE still enforces the
-- 1:1 (each team group belongs to at most one bracket; multiple NULLs are allowed).
-- ON DELETE SET NULL: destroying a team group simply leaves its bracket with none (the column is
-- nullable), rather than blocking the delete.
ALTER TABLE pool_brackets ADD COLUMN team_group_id UUID REFERENCES teamgroups(team_group_id) ON DELETE SET NULL;

-- Backfill from the existing teamgroups -> pool_bracket mapping.
UPDATE pool_brackets pb
   SET team_group_id = tg.team_group_id
  FROM teamgroups tg
 WHERE tg.pool_bracket_id = pb.pool_bracket_id;

ALTER TABLE pool_brackets
    ADD CONSTRAINT pool_brackets_team_group_id_key UNIQUE (team_group_id);

-- Drop the old direction. Dropping the column also drops its FK, its UNIQUE and its NOT NULL.
ALTER TABLE teamgroups DROP COLUMN pool_bracket_id;
