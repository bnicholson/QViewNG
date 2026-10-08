-- Which scheduling UI the tournament's Schedule > Create/Edit section presents: Normal mode (false,
-- the default) manages only Rounds and Games and hides Sessions/PoolBracketGroups/Pools, while
-- Advanced mode (true) exposes all of them.
ALTER TABLE tournaments ADD COLUMN use_advanced_scheduling BOOLEAN NOT NULL DEFAULT false;
