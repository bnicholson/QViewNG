
ALTER TABLE teams_quizzers
    DROP CONSTRAINT teams_quizzers_teamid_quizzerid_key;
ALTER TABLE teams_quizzers
    ADD CONSTRAINT pk_teamid_quizzerid PRIMARY KEY (teamid,quizzerid);

ALTER TABLE tournaments_admins
    DROP CONSTRAINT uk_tournamentid_adminid;
ALTER TABLE tournaments_admins
    ADD CONSTRAINT pk_tournamentid_adminid PRIMARY KEY (tournamentid,adminid);

ALTER TABLE tournamentgroups_tournaments
    DROP CONSTRAINT uk_tournamentgroupid_tournamentid;
ALTER TABLE tournamentgroups_tournaments
    ADD CONSTRAINT pk_tournamentgroupid_tournamentid PRIMARY KEY (tournamentgroupid,tournamentid);

ALTER TABLE games_statsgroups
    DROP CONSTRAINT uk_gid_sgid;
ALTER TABLE games_statsgroups
    ADD CONSTRAINT pk_gid_sgid PRIMARY KEY (gid,sgid);
