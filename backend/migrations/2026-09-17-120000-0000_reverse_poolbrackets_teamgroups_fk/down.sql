-- Restore the original direction: teamgroups.pool_bracket_id holds the 1:1 FK + UNIQUE.
ALTER TABLE teamgroups ADD COLUMN pool_bracket_id UUID REFERENCES pool_brackets(pool_bracket_id);

UPDATE teamgroups tg
   SET pool_bracket_id = pb.pool_bracket_id
  FROM pool_brackets pb
 WHERE pb.team_group_id = tg.team_group_id;

-- A team group with no owning bracket can't be mapped back (shouldn't happen given the 1:1).
DELETE FROM teamgroups WHERE pool_bracket_id IS NULL;

ALTER TABLE teamgroups ALTER COLUMN pool_bracket_id SET NOT NULL;
ALTER TABLE teamgroups ADD CONSTRAINT teamgroups_pool_bracket_id_key UNIQUE (pool_bracket_id);

ALTER TABLE pool_brackets DROP CONSTRAINT pool_brackets_team_group_id_key;
ALTER TABLE pool_brackets DROP COLUMN team_group_id;
