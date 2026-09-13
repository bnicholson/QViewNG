-- Remove the redundant games.divisionid column. A game's division is now derived via the lookup
-- path game -> pool_bracket -> division_session -> division, keeping division data consistent when
-- a game (or its pool bracket) is updated.
--
-- Dropping the column also drops its FK (games_divisionid_fkey) and the unique constraint that
-- included it (games_org_tournament_division_room_round_clientkey_key).
ALTER TABLE games DROP COLUMN divisionid;

-- Re-establish game uniqueness without the division column.
ALTER TABLE games
    ADD CONSTRAINT games_org_tournament_room_round_clientkey_key
    UNIQUE (org, tournamentid, roomid, roundid, clientkey);
