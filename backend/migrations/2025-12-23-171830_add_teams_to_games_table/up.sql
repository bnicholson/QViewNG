
ALTER TABLE games
    ADD COLUMN leftteamid BIGINT NOT NULL REFERENCES teams(teamid),
    ADD COLUMN centerteamid BIGINT REFERENCES teams(teamid),
    ADD COLUMN rightteamid BIGINT NOT NULL REFERENCES teams(teamid);
