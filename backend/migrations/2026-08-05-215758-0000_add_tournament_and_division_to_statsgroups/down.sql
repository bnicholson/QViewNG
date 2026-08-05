-- This file should undo anything in `up.sql`
ALTER TABLE statsgroups
    DROP COLUMN division_id,
    DROP COLUMN tournament_id;
