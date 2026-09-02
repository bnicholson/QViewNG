-- Retire `region` in favor of `state`: carry existing region values into state (only where
-- state hasn't already been set), then drop the region column.
UPDATE tournaments SET state = region WHERE state = '';
ALTER TABLE tournaments DROP COLUMN region;
