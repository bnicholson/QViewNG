
ALTER TABLE tournaments_admins
    ADD CONSTRAINT uk_tournamentid_adminid UNIQUE(tournamentid,adminid);

ALTER TABLE tournamentgroups
    ADD CONSTRAINT uk_tgid_name UNIQUE(tgid,name);

ALTER TABLE tournamentgroups_tournaments
    ADD CONSTRAINT uk_tournamentgroupid_tournamentid UNIQUE(tournamentgroupid,tournamentid);

ALTER TABLE statsgroups
    ADD CONSTRAINT uk_sgid_name UNIQUE(sgid,name);

ALTER TABLE games_statsgroups
    ADD CONSTRAINT uk_gid_sgid UNIQUE(gid,sgid);

ALTER TABLE teams
    ADD CONSTRAINT uk_divisionid_name UNIQUE(divisionid,name);
