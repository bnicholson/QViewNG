// @generated automatically by Diesel CLI.

diesel::table! {
    apicalllog (apicallid) {
        created_at -> Timestamptz,
        apicallid -> Int8,
        #[max_length = 8]
        method -> Varchar,
        uri -> Text,
        #[max_length = 32]
        version -> Varchar,
        headers -> Text,
    }
}

diesel::table! {
    attachment_blobs (id) {
        id -> Int4,
        key -> Text,
        file_name -> Text,
        content_type -> Nullable<Text>,
        byte_size -> Int8,
        checksum -> Text,
        service_name -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    attachments (id) {
        id -> Int4,
        name -> Text,
        record_type -> Text,
        record_id -> Int4,
        blob_id -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    divisions (did) {
        did -> Int8,
        tid -> Int8,
        #[max_length = 32]
        dname -> Varchar,
        #[max_length = 32]
        breadcrumb -> Varchar,
        hide -> Bool,
        #[max_length = 1024]
        shortinfo -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    eventlogs (evid) {
        evid -> Int8,
        created_at -> Timestamptz,
        #[max_length = 64]
        clientkey -> Varchar,
        #[max_length = 48]
        organization -> Varchar,
        #[max_length = 32]
        bldgroom -> Varchar,
        #[max_length = 48]
        tournament -> Varchar,
        #[max_length = 48]
        division -> Varchar,
        #[max_length = 48]
        room -> Varchar,
        #[max_length = 48]
        round -> Varchar,
        question -> Int4,
        eventnum -> Int4,
        #[max_length = 64]
        name -> Varchar,
        team -> Int4,
        quizzer -> Int4,
        #[max_length = 2]
        event -> Varchar,
        #[max_length = 64]
        parm1 -> Varchar,
        #[max_length = 64]
        parm2 -> Varchar,
        #[max_length = 32]
        ts -> Varchar,
        #[max_length = 32]
        clientip -> Varchar,
        #[max_length = 32]
        md5digest -> Varchar,
        #[max_length = 80]
        nonce -> Varchar,
        #[max_length = 32]
        s1s -> Varchar,
    }
}

diesel::table! {
    games (gid) {
        gid -> Int8,
        #[max_length = 48]
        org -> Varchar,
        #[max_length = 64]
        clientkey -> Varchar,
        ignore -> Bool,
        #[max_length = 32]
        ruleset -> Varchar,
        quizmasterid -> Nullable<Int8>,
        contentjudgeid -> Nullable<Int8>,
        tournamentid -> Nullable<Int8>,
        divisionid -> Nullable<Int8>,
        roomid -> Nullable<Int8>,
        roundid -> Nullable<Int8>,
        leftteamid -> Int8,
        centerteamid -> Nullable<Int8>,
        rightteamid -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    games_statsgroups (gid, sgid) {
        gid -> Int8,
        sgid -> Int8,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    quizevents (gid, question, eventnum) {
        gid -> Int8,
        question -> Int4,
        eventnum -> Int4,
        #[max_length = 64]
        name -> Varchar,
        team -> Int4,
        quizzer -> Int4,
        #[max_length = 2]
        event -> Varchar,
        #[max_length = 8]
        parm1 -> Varchar,
        #[max_length = 8]
        parm2 -> Varchar,
        clientts -> Timestamptz,
        serverts -> Timestamptz,
        #[max_length = 32]
        md5digest -> Varchar,
    }
}

diesel::table! {
    role_permissions (role, permission) {
        role -> Text,
        permission -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    rooms (roomid) {
        roomid -> Int8,
        tid -> Int8,
        #[max_length = 32]
        name -> Varchar,
        #[max_length = 32]
        building -> Varchar,
        comments -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    rosters (rosterid) {
        rosterid -> Int8,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 256]
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    rosters_coaches (coachid, rosterid) {
        coachid -> Int8,
        rosterid -> Int8,
    }
}

diesel::table! {
    rosters_quizzers (quizzerid, rosterid) {
        quizzerid -> Int8,
        rosterid -> Int8,
    }
}

diesel::table! {
    rounds (roundid) {
        roundid -> Int8,
        did -> Nullable<Int8>,
        scheduled_start_time -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    schedules (sid) {
        sid -> Int8,
        tid -> Int8,
        roundtime -> Timestamptz,
        #[max_length = 32]
        tournament -> Varchar,
        #[max_length = 32]
        division -> Varchar,
        #[max_length = 32]
        room -> Varchar,
        #[max_length = 32]
        round -> Varchar,
        #[max_length = 32]
        team1 -> Nullable<Varchar>,
        #[max_length = 32]
        team2 -> Nullable<Varchar>,
        #[max_length = 32]
        team3 -> Nullable<Varchar>,
        #[max_length = 32]
        quizmaster -> Nullable<Varchar>,
        #[max_length = 32]
        contentjudge -> Nullable<Varchar>,
        #[max_length = 32]
        scorekeeper -> Nullable<Varchar>,
        #[max_length = 1024]
        stats -> Nullable<Varchar>,
    }
}

diesel::table! {
    statsgroups (sgid) {
        sgid -> Int8,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 256]
        description -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    teams (teamid) {
        teamid -> Int8,
        divisionid -> Int8,
        coachid -> Int8,
        #[max_length = 128]
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    teams_quizzers (teamid, quizzerid) {
        teamid -> Int8,
        quizzerid -> Int8,
        #[max_length = 64]
        role_description -> Nullable<Varchar>,
        access_lvl -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    tournamentgroups (tgid) {
        tgid -> Int8,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 256]
        description -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    tournamentgroups_tournaments (tournamentgroupid, tournamentid) {
        tournamentid -> Int8,
        tournamentgroupid -> Int8,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    tournaments (tid) {
        tid -> Int8,
        #[max_length = 32]
        organization -> Varchar,
        #[max_length = 32]
        tname -> Varchar,
        #[max_length = 32]
        breadcrumb -> Varchar,
        fromdate -> Date,
        todate -> Date,
        #[max_length = 64]
        venue -> Varchar,
        #[max_length = 64]
        city -> Varchar,
        #[max_length = 64]
        region -> Varchar,
        #[max_length = 32]
        country -> Varchar,
        #[max_length = 64]
        contact -> Varchar,
        #[max_length = 255]
        contactemail -> Varchar,
        hide -> Bool,
        #[max_length = 1024]
        shortinfo -> Varchar,
        info -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    tournaments_admins (tournamentid, adminid) {
        tournamentid -> Int8,
        adminid -> Int8,
        #[max_length = 64]
        role_description -> Nullable<Varchar>,
        access_lvl -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    user_permissions (user_id, permission) {
        user_id -> Int8,
        permission -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    user_roles (user_id, role) {
        user_id -> Int8,
        role -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    user_sessions (id) {
        id -> Int8,
        user_id -> Int8,
        refresh_token -> Text,
        device -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        email -> Text,
        hash_password -> Text,
        activated -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        #[max_length = 64]
        fname -> Varchar,
        #[max_length = 64]
        mname -> Varchar,
        #[max_length = 64]
        lname -> Varchar,
    }
}

diesel::joinable!(attachments -> attachment_blobs (blob_id));
diesel::joinable!(games -> divisions (divisionid));
diesel::joinable!(games -> rooms (roomid));
diesel::joinable!(games -> rounds (roundid));
diesel::joinable!(games -> tournaments (tournamentid));
diesel::joinable!(games_statsgroups -> games (gid));
diesel::joinable!(games_statsgroups -> statsgroups (sgid));
diesel::joinable!(rosters_coaches -> rosters (rosterid));
diesel::joinable!(rosters_coaches -> users (coachid));
diesel::joinable!(rosters_quizzers -> rosters (rosterid));
diesel::joinable!(rosters_quizzers -> users (quizzerid));
diesel::joinable!(rounds -> divisions (did));
diesel::joinable!(teams -> divisions (divisionid));
diesel::joinable!(teams -> users (coachid));
diesel::joinable!(teams_quizzers -> teams (teamid));
diesel::joinable!(teams_quizzers -> users (quizzerid));
diesel::joinable!(tournamentgroups_tournaments -> tournamentgroups (tournamentgroupid));
diesel::joinable!(tournamentgroups_tournaments -> tournaments (tournamentid));
diesel::joinable!(tournaments_admins -> tournaments (tournamentid));
diesel::joinable!(tournaments_admins -> users (adminid));
diesel::joinable!(user_permissions -> users (user_id));
diesel::joinable!(user_roles -> users (user_id));
diesel::joinable!(user_sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    apicalllog,
    attachment_blobs,
    attachments,
    divisions,
    eventlogs,
    games,
    games_statsgroups,
    quizevents,
    role_permissions,
    rooms,
    rosters,
    rosters_coaches,
    rosters_quizzers,
    rounds,
    schedules,
    statsgroups,
    teams,
    teams_quizzers,
    tournamentgroups,
    tournamentgroups_tournaments,
    tournaments,
    tournaments_admins,
    user_permissions,
    user_roles,
    user_sessions,
    users,
);
