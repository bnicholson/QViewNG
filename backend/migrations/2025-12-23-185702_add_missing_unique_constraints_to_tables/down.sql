
ALTER TABLE tournaments_admins
    DROP CONSTRAINT uk_tournamentid_adminid;

ALTER TABLE tournamentgroups
    DROP CONSTRAINT uk_tgid_name;

ALTER TABLE tournamentgroups_tournaments
    DROP CONSTRAINT uk_tournamentgroupid_tournamentid;

ALTER TABLE statsgroups
    DROP CONSTRAINT uk_sgid_name;

ALTER TABLE games_statsgroups
    DROP CONSTRAINT uk_gid_sgid;

ALTER TABLE teams
    DROP CONSTRAINT uk_divisionid_name;
