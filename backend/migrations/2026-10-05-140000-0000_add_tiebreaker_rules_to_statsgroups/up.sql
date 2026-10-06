-- How a stats group breaks ties between teams in its standings.
--
-- Stored as the ordered, comma-separated list of criterion keys, each optionally prefixed with
-- '-' when the user has unchecked it — keeping a disabled criterion in place means re-enabling it
-- restores its original position. An empty string means "use the built-in default order", so
-- existing stats groups keep today's behaviour until someone saves a configuration.
--
--   e.g.  'team_wl,-head_to_head,total_points,avg_points,olympic,mod_olympic'
ALTER TABLE statsgroups
    ADD COLUMN tiebreaker_rules TEXT NOT NULL DEFAULT '';
