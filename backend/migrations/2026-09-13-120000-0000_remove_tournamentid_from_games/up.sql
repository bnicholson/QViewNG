-- Remove the redundant games.tournamentid column. A game's tournament is now derived via the
-- lookup path game -> pool_bracket -> division_session -> division -> tournament, keeping tournament
-- data consistent when a game (or its pool bracket) is updated.
--
-- Dropping the column also drops its FK (games_tournamentid_fkey) and the unique constraint that
-- included it (games_org_tournament_room_round_clientkey_key).
ALTER TABLE games DROP COLUMN tournamentid;

-- Re-establish game uniqueness without the tournament column. (A room and round already belong to a
-- single tournament, so this stays effectively tournament-scoped.)
ALTER TABLE games
    ADD CONSTRAINT games_org_room_round_clientkey_key
    UNIQUE (org, roomid, roundid, clientkey);
