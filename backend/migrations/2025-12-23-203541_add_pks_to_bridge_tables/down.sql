
ALTER TABLE teams_quizzers
    DROP CONSTRAINT pk_teamid_quizzerid;
ALTER TABLE teams_quizzers
    ADD CONSTRAINT teams_quizzers_teamid_quizzerid_key UNIQUE (teamid,quizzerid);

ALTER TABLE tournaments_admins
    DROP CONSTRAINT pk_tournamentid_adminid;
ALTER TABLE tournaments_admins
    ADD CONSTRAINT uk_tournamentid_adminid UNIQUE (tournamentid,adminid);

ALTER TABLE tournamentgroups_tournaments
    DROP CONSTRAINT pk_tournamentgroupid_tournamentid;
ALTER TABLE tournamentgroups_tournaments
    ADD CONSTRAINT uk_tournamentgroupid_tournamentid UNIQUE (tournamentgroupid,tournamentid);

ALTER TABLE games_statsgroups
    DROP CONSTRAINT pk_gid_sgid;
ALTER TABLE games_statsgroups
    ADD CONSTRAINT uk_gid_sgid UNIQUE (gid,sgid);
