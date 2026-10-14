use crate::{database::{self, seed_data::system_default_data::default_password}, models::{computer::ComputerBuilder, create_tournament_applicant::CreateTournamentApplicantBuilder, division::DivisionBuilder, roundgroup::RoundGroupBuilder, pool_bracket::PoolBracketBuilder, poolbracketgroup::PoolBracketGroupBuilder, equipmentregistration::{EquipmentRegistrationBuilder, EquipmentRegistrationStatus}, extensioncord::ExtensionCordBuilder, game::GameBuilder, interfacebox::InterfaceBoxBuilder, jumppad::JumpPadBuilder, microphonerecorder::MicrophoneRecorderBuilder, monitor::MonitorBuilder, powerstrip::PowerStripBuilder, projector::ProjectorBuilder, role::AppRole, room::RoomBuilder, roomgroup::RoomGroupBuilder, roster::RosterBuilder, roster_coach::RosterCoachBuilder, roster_quizzer::RosterQuizzerBuilder, round::RoundBuilder, statsgroup::StatsGroupBuilder, team::{Team, TeamBuilder}, team_teamgroup::TeamTeamgroupBuilder, tournament::TournamentBuilder, tournament_admin::TournamentAdminBuilder, tournamentgroup::TournamentGroupBuilder, tournamentgroup_tournament::TournamentGroupTournamentBuilder, user::UserBuilder, users_roles::UsersRolesBuilder}};
use chrono::{DateTime, Local, NaiveDate, Duration, TimeZone, Utc};
use uuid::Uuid;
use crate::models::gameevent::{GameEventBuilder, GameEventCode};

pub fn insert_seed_data_one(db: &mut database::Connection, include_scheduling: bool, include_gameevents: bool) {
    let start_time_for_db_pop_seed_data = Utc::now();
    println!("Starting DB Data Population for Seed Data");

    add_tour_1_demo(db, include_scheduling, include_gameevents);
    create_tournament_applicants(db);

    let end_time_for_db_pop_seed_data = Utc::now();
    let duration_for_db_pop_seed_data = end_time_for_db_pop_seed_data.naive_utc() - start_time_for_db_pop_seed_data.naive_utc();
    println!("Done. DB Seed Data Population Time Duration: {}\n", duration_for_db_pop_seed_data);
}

pub fn add_tour_1_demo(db: &mut database::Connection, include_scheduling: bool, include_gameevents: bool) {

    // Add Touranment Manager (*owner of Tour One):

    let user = UserBuilder::new("Tournament")
        .set_lname("Manager (Demo)")
        .set_username("tournamentmanager")
        .set_hash_password(&default_password())
        .set_email("tmanager@fakeemail.com")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();

    let member_role             = crate::models::role::read_by_name(db, AppRole::Member.as_str()).unwrap();
    let tournament_manager_role = crate::models::role::read_by_name(db, AppRole::TournamentManager.as_str()).unwrap();

    UsersRolesBuilder::new(user.id)
        .assign(member_role.id)
        .assign(tournament_manager_role.id)
        .build_and_insert(db)
        .unwrap();

    // Add member user:

    let member_user = UserBuilder::new("Justa")
        .set_lname("Member (Demo)")
        .set_username("member")
        .set_hash_password(&default_password())
        .set_email("justamember@fakeemail.com")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();

    let member_role = crate::models::role::read_by_name(db, AppRole::Member.as_str()).unwrap();

    UsersRolesBuilder::new(member_user.id)
        .assign(member_role.id)
        .build_and_insert(db)
        .unwrap();

    // Add Tournament One, starting here:
    
    let tour_owner = crate::models::user::UserBuilder::new("Tour")
        .set_mname("One")
        .set_lname("Owner (Demo)")
        .set_username("touroneowner")
        .set_hash_password(&default_password())
        .set_email("touroneowner@fakeemail.com")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();

    let member_role             = crate::models::role::read_by_name(db, AppRole::Member.as_str()).unwrap();
    let tournament_manager_role = crate::models::role::read_by_name(db, AppRole::TournamentManager.as_str()).unwrap();
    let tournament_admin_role   = crate::models::role::read_by_name(db, AppRole::TournamentAdmin.as_str()).unwrap();

    crate::models::users_roles::UsersRolesBuilder::new(tour_owner.id)
        .assign(member_role.id)
        .assign(tournament_manager_role.id)
        .build_and_insert(db)
        .unwrap();

    let today: NaiveDate = Local::now().date_naive();
    let five_days_later: NaiveDate = today + Duration::days(5);
    let tour = TournamentBuilder::new_default("Tournament One (Demo)")
        .set_fromdate(today)
        .set_todate(five_days_later)
        .set_venue("TNU")
        .set_address_line_1("333 Murfreesboro Pike")
        .set_city("Nashville")
        .set_state("TN")
        .set_zip_code("37210")
        .set_country("USA")
        .set_registration_open_date(today - Duration::days(7))
        .set_registration_close_date(five_days_later)
        .set_contact("Skipper Jets")
        .set_contactemail("skippyjets@yahoo.com")
        .set_shortinfo("Display standard data")
        .set_info("This Tournament is intended to show the visitor what a fully data-entered Tournament would look like.")
        .set_owner_id(tour_owner.id)
        .set_is_public(true)
        .build_and_insert(db)
        .unwrap();

    // Tournament group
    let tg_1 = TournamentGroupBuilder::new_default("TG 1 (Demo)")
        .set_creator_id(tour_owner.id)
        .set_owner_id(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    TournamentGroupTournamentBuilder::new_default(tg_1.tgid, tour.tid)
        .build_and_insert(db)
        .unwrap();

    // Tournament admins
    let admin_1 = UserBuilder::new("Tour")
        .set_mname("One")
        .set_lname("Admin (Demo)")
        .set_username("touroneadmin")
        .set_hash_password(&default_password())
        .set_email("touroneadmin@fakeemail.com")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    UsersRolesBuilder::new(admin_1.id)
        .assign(member_role.id)
        .assign(tournament_admin_role.id)
        .build_and_insert(db)
        .unwrap();
    TournamentAdminBuilder::new_default(tour.tid, admin_1.id)
        .build_and_insert(db)
        .unwrap();

    let admin_2 = UserBuilder::new("Ben")
        .set_lname("Castillo (Demo)")
        .set_username("bcastillo")
        .set_hash_password(&default_password())
        .set_email("bcastillo@fakeemail.com")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    UsersRolesBuilder::new(admin_2.id)
        .assign(member_role.id)
        .assign(tournament_admin_role.id)
        .build_and_insert(db)
        .unwrap();
    TournamentAdminBuilder::new_default(tour.tid, admin_2.id)
        .build_and_insert(db)
        .unwrap();

    let admin_3 = UserBuilder::new("Clara")
        .set_lname("Voss (Demo)")
        .set_username("cvoss")
        .set_hash_password(&default_password())
        .set_email("cvoss@fakeemail.com")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    UsersRolesBuilder::new(admin_3.id)
        .assign(member_role.id)
        .assign(tournament_admin_role.id)
        .build_and_insert(db)
        .unwrap();
    TournamentAdminBuilder::new_default(tour.tid, admin_3.id)
        .build_and_insert(db)
        .unwrap();

    let division_experienced = DivisionBuilder::new_default("Experienced", tour.tid)
        .set_last_modified_user(tour_owner.id)
        .set_shortinfo("Been around the block".to_string())
        .set_is_public(true)
        .build_and_insert(db)
        .unwrap();
    let division_novice = DivisionBuilder::new_default("Novice", tour.tid)
        .set_last_modified_user(tour_owner.id)
        .set_shortinfo("New to this".to_string())
        .set_is_public(true)
        .build_and_insert(db)
        .unwrap();
    let division_decades = DivisionBuilder::new_default("Decades", tour.tid)
        .set_last_modified_user(tour_owner.id)
        .set_shortinfo("Young at heart!".to_string())
        .set_is_public(true)
        .build_and_insert(db)
        .unwrap();

    // One division-scoped statsgroup per division. Created before any game, so each game joins
    // its division's group as it is created (see `game::create`).
    StatsGroupBuilder::new_default(&division_experienced.dname, tour.tid)
        .set_division_id(Some(division_experienced.did))
        .build_and_insert(db)
        .unwrap();
    StatsGroupBuilder::new_default(&division_novice.dname, tour.tid)
        .set_division_id(Some(division_novice.did))
        .build_and_insert(db)
        .unwrap();
    StatsGroupBuilder::new_default(&division_decades.dname, tour.tid)
        .set_division_id(Some(division_decades.did))
        .build_and_insert(db)
        .unwrap();

    // Division roundgroups and pools/brackets are created later, inside the `include_scheduling` block
    // (below), together with the team placements and games that depend on them — so an unscheduled
    // tournament stops at registered teams + gear.

    // Quizmasters — one per room, each unique across all rooms
    let qm_1 = UserBuilder::new("Jordan")
        .set_lname("Avery (Demo)")
        .set_username("javery_qm")
        .set_email("javery@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let qm_2 = UserBuilder::new("Riley")
        .set_lname("Blake (Demo)")
        .set_username("rblake_qm")
        .set_email("rblake@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let qm_3 = UserBuilder::new("Morgan")
        .set_lname("Casey (Demo)")
        .set_username("mcasey_qm")
        .set_email("mcasey@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let qm_4 = UserBuilder::new("Quinn")
        .set_lname("Drew (Demo)")
        .set_username("qdrew_qm")
        .set_email("qdrew@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let qm_5 = UserBuilder::new("Sage")
        .set_lname("Ellis (Demo)")
        .set_username("sellis_qm")
        .set_email("sellis@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let qm_6 = UserBuilder::new("Avery")
        .set_lname("Flynn (Demo)")
        .set_username("aflynn_qm")
        .set_email("aflynn@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let qm_7 = UserBuilder::new("Blake")
        .set_lname("Grant (Demo)")
        .set_username("bgrant_qm")
        .set_email("bgrant@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();

    // Content Judges — one per room, each unique across all rooms
    let cj_1 = UserBuilder::new("Dana")
        .set_lname("Harper (Demo)")
        .set_username("dharper_cj")
        .set_email("dharper@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let cj_2 = UserBuilder::new("Emery")
        .set_lname("Ingram (Demo)")
        .set_username("eingram_cj")
        .set_email("eingram@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let cj_3 = UserBuilder::new("Finley")
        .set_lname("Jensen (Demo)")
        .set_username("fjensen_cj")
        .set_email("fjensen@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let cj_4 = UserBuilder::new("Gray")
        .set_lname("Knox (Demo)")
        .set_username("gknox_cj")
        .set_email("gknox@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let cj_5 = UserBuilder::new("Hayden")
        .set_lname("Lane (Demo)")
        .set_username("hlane_cj")
        .set_email("hlane@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let cj_6 = UserBuilder::new("Indigo")
        .set_lname("Marsh (Demo)")
        .set_username("imarsh_cj")
        .set_email("imarsh@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let cj_7 = UserBuilder::new("Jamie")
        .set_lname("Nash (Demo)")
        .set_username("jnash_cj")
        .set_email("jnash@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();

    // Quizmasters and content judges for Rooms 8-10 (added so every room is fully staffed).
    let qm_8 = UserBuilder::new("Cameron")
        .set_lname("Reese (Demo)")
        .set_username("creese_qm")
        .set_email("creese@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let qm_9 = UserBuilder::new("Devon")
        .set_lname("Sloan (Demo)")
        .set_username("dsloan_qm")
        .set_email("dsloan@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let qm_10 = UserBuilder::new("Emerson")
        .set_lname("Tate (Demo)")
        .set_username("etate_qm")
        .set_email("etate@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let cj_8 = UserBuilder::new("Kai")
        .set_lname("Owens (Demo)")
        .set_username("kowens_cj")
        .set_email("kowens@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let cj_9 = UserBuilder::new("Lennon")
        .set_lname("Pierce (Demo)")
        .set_username("lpierce_cj")
        .set_email("lpierce@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let cj_10 = UserBuilder::new("Marlowe")
        .set_lname("Vance (Demo)")
        .set_username("mvance_cj")
        .set_email("mvance@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();

    // Three roomgroups (buildings). Each division quizzes in its own building: Experienced in
    // Building A (rooms 1-6), Novice in Building B (rooms 7-8), Decades in Building C (rooms 9-10).
    let building_a = RoomGroupBuilder::new(tour.tid)
        .set_name("Building A")
        .set_notes("Experienced division")
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    let building_b = RoomGroupBuilder::new(tour.tid)
        .set_name("Building B")
        .set_notes("Novice division")
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    let building_c = RoomGroupBuilder::new(tour.tid)
        .set_name("Building C")
        .set_notes("Decades division")
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();

    let room_1 = RoomBuilder::new_default("Room 1", tour.tid)
        .set_roomgroupid(Some(building_a.roomgroupid))
        .set_last_modified_user(tour_owner.id)
        .set_comments("".to_string())
        .set_clientkey(Some("bankdiu".to_string()))
        .set_quizmaster_id(Some(qm_1.id))
        .set_contentjudge_id(Some(cj_1.id))
        .build_and_insert(db)
        .unwrap();
    let room_2 = RoomBuilder::new_default("Room 2", tour.tid)
        .set_roomgroupid(Some(building_a.roomgroupid))
        .set_last_modified_user(tour_owner.id)
        .set_comments("".to_string())
        .set_clientkey(Some("bbhsth4".to_string()))
        .set_quizmaster_id(Some(qm_2.id))
        .set_contentjudge_id(Some(cj_2.id))
        .build_and_insert(db)
        .unwrap();
    let room_3 = RoomBuilder::new_default("Room 3", tour.tid)
        .set_roomgroupid(Some(building_a.roomgroupid))
        .set_last_modified_user(tour_owner.id)
        .set_comments("".to_string())
        .set_clientkey(Some("16587397".to_string()))
        .set_quizmaster_id(Some(qm_3.id))
        .set_contentjudge_id(Some(cj_3.id))
        .build_and_insert(db)
        .unwrap();
    let room_4 = RoomBuilder::new_default("Room 4", tour.tid)
        .set_roomgroupid(Some(building_a.roomgroupid))
        .set_last_modified_user(tour_owner.id)
        .set_comments("".to_string())
        .set_clientkey(Some("aplyhen".to_string()))
        .set_quizmaster_id(Some(qm_4.id))
        .set_contentjudge_id(Some(cj_4.id))
        .build_and_insert(db)
        .unwrap();
    let room_5 = RoomBuilder::new_default("Room 5", tour.tid)
        .set_roomgroupid(Some(building_a.roomgroupid))
        .set_last_modified_user(tour_owner.id)
        .set_comments("".to_string())
        .set_clientkey(Some("llpjhin".to_string()))
        .set_quizmaster_id(Some(qm_5.id))
        .set_contentjudge_id(Some(cj_5.id))
        .build_and_insert(db)
        .unwrap();
    let room_6 = RoomBuilder::new_default("Room 6", tour.tid)
        .set_roomgroupid(Some(building_a.roomgroupid))
        .set_last_modified_user(tour_owner.id)
        .set_comments("".to_string())
        .set_clientkey(Some("qwx7bfyh".to_string()))
        .set_quizmaster_id(Some(qm_6.id))
        .set_contentjudge_id(Some(cj_6.id))
        .build_and_insert(db)
        .unwrap();
    let room_7 = RoomBuilder::new_default("Room 7", tour.tid)
        .set_roomgroupid(Some(building_b.roomgroupid))
        .set_last_modified_user(tour_owner.id)
        .set_comments("".to_string())
        .set_clientkey(Some("jjkalndi".to_string()))
        .set_quizmaster_id(Some(qm_7.id))
        .set_contentjudge_id(Some(cj_7.id))
        .build_and_insert(db)
        .unwrap();
    // Rooms 8-10 added so the Experienced division's two 6-team pools (3 rooms each) can run
    // concurrently with Novice and Decades in Session 1 without any room being double-booked.
    let room_8 = RoomBuilder::new_default("Room 8", tour.tid)
        .set_roomgroupid(Some(building_b.roomgroupid))
        .set_last_modified_user(tour_owner.id)
        .set_comments("".to_string())
        .set_clientkey(Some("rm8xk2po".to_string()))
        .set_quizmaster_id(Some(qm_8.id))
        .set_contentjudge_id(Some(cj_8.id))
        .build_and_insert(db)
        .unwrap();
    let room_9 = RoomBuilder::new_default("Room 9", tour.tid)
        .set_roomgroupid(Some(building_c.roomgroupid))
        .set_last_modified_user(tour_owner.id)
        .set_comments("".to_string())
        .set_clientkey(Some("rm9zt5qa".to_string()))
        .set_quizmaster_id(Some(qm_9.id))
        .set_contentjudge_id(Some(cj_9.id))
        .build_and_insert(db)
        .unwrap();
    let room_10 = RoomBuilder::new_default("Room 10", tour.tid)
        .set_roomgroupid(Some(building_c.roomgroupid))
        .set_last_modified_user(tour_owner.id)
        .set_comments("".to_string())
        .set_clientkey(Some("rm10wv3b".to_string()))
        .set_quizmaster_id(Some(qm_10.id))
        .set_contentjudge_id(Some(cj_10.id))
        .build_and_insert(db)
        .unwrap();

    // Div: Experienced
    let coach_exp_1 = UserBuilder::new("Coachish")
        .set_lname("Coach (Demo)")
        .set_email("coach@fakeemail.com")
        .set_username("coach")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();

    // Add "Set 1" equipment set for the coach user with one of each gear type
    let coach_set_1 = crate::models::equipmentset::EquipmentSetBuilder::new_default(coach_exp_1.id)
        .set_name("Set 1")
        .set_description(Some("Main GearSet".to_string()))
        .build_and_insert(db)
        .unwrap();

    let eq_s1_computer = ComputerBuilder::new_default(coach_set_1.id)
        .set_brand(Some("Dell".to_string()))
        .set_misc_note(Some("Main quiz machine".to_string()))
        .set_operating_system(Some("Windows 11".to_string()))
        .set_quizmachine_version(Some("6.2.2".to_string()))
        .set_wifi_capabilities(Some("capable".to_string()))
        .set_login_username(Some("Quizzer".to_string()))
        .set_login_password(Some("Password123!".to_string()))
        .set_has_vga_out_port(Some(true))
        .set_has_display_port_out(Some(true))
        .set_has_dvi_out_port(Some(false))
        .set_has_hdmi_out_port(Some(true))
        .set_has_usb_port(Some(true))
        .set_clientkey(Some("liunalfw2k4oiww".to_string()))
        .build_and_insert(db)
        .unwrap();

    let eq_s1_jumppad = JumpPadBuilder::new_default(coach_set_1.id)
        .set_misc_note(Some("Jump pads set, red".to_string()))
        .set_color(Some("red".to_string()))
        .build_and_insert(db)
        .unwrap();

    let eq_s1_interfacebox = InterfaceBoxBuilder::new_default(coach_set_1.id)
        .set_type_(Some("Wired".to_string()))
        .set_serial_number(Some("ahjsunkkdmif67nn87sk".to_string()))
        .build_and_insert(db)
        .unwrap();

    let eq_s1_monitor = MonitorBuilder::new_default(coach_set_1.id)
        .set_size(Some("27".to_string()))
        .set_brand(Some("Dell".to_string()))
        .set_has_vga_out_port(Some(true))
        .set_has_display_port_out(Some(false))
        .set_has_dvi_out_port(Some(false))
        .set_has_hdmi_out_port(Some(false))
        .build_and_insert(db)
        .unwrap();

    let eq_s1_microphone = MicrophoneRecorderBuilder::new_default(coach_set_1.id)
        .set_type_(Some("External".to_string()))
        .set_misc_note(Some("Casio".to_string()))
        .build_and_insert(db)
        .unwrap();

    let eq_s1_projector = ProjectorBuilder::new_default(coach_set_1.id)
        .set_brand(Some("Epson".to_string()))
        .set_has_vga_out_port(Some(true))
        .set_has_display_port_out(Some(true))
        .set_has_dvi_out_port(Some(true))
        .set_has_hdmi_out_port(Some(true))
        .build_and_insert(db)
        .unwrap();

    let eq_s1_powerstrip = PowerStripBuilder::new_default(coach_set_1.id)
        .set_make(Some("Belkin".to_string()))
        .set_model(Some("BE108000-06".to_string()))
        .set_color(Some("white".to_string()))
        .set_num_of_plugs(Some(8))
        .set_misc_note(Some("Old reliable".to_string()))
        .build_and_insert(db)
        .unwrap();

    let eq_s1_extensioncord = ExtensionCordBuilder::new_default(coach_set_1.id)
        .set_length(Some("6ft".to_string()))
        .set_color(Some("Black".to_string()))
        .set_misc_note(Some("The one that came from dad's house".to_string()))
        .build_and_insert(db)
        .unwrap();

    // Add "Set 1" equipment set for the coach user with one of each gear type
    let coach_set_2 = crate::models::equipmentset::EquipmentSetBuilder::new_default(coach_exp_1.id)
        .set_name("Set 2")
        .set_description(Some("Backup GearSet".to_string()))
        .build_and_insert(db)
        .unwrap();

    let eq_s2_computer = ComputerBuilder::new_default(coach_set_2.id)
        .set_brand(Some("Dell".to_string()))
        .set_misc_note(Some("Main quiz machine".to_string()))
        .set_operating_system(Some("Windows 11".to_string()))
        .set_quizmachine_version(Some("6.2.2".to_string()))
        .set_wifi_capabilities(Some("capable".to_string()))
        .set_login_username(Some("Quizzer".to_string()))
        .set_login_password(Some("Password123!".to_string()))
        .set_has_vga_out_port(Some(true))
        .set_has_display_port_out(Some(true))
        .set_has_dvi_out_port(Some(false))
        .set_has_hdmi_out_port(Some(true))
        .set_has_usb_port(Some(true))
        .set_clientkey(Some("liunalfw2k4oiww".to_string()))
        .build_and_insert(db)
        .unwrap();

    let eq_s2_jumppad = JumpPadBuilder::new_default(coach_set_2.id)
        .set_misc_note(Some("Jump pads set, red".to_string()))
        .set_color(Some("red".to_string()))
        .build_and_insert(db)
        .unwrap();

    let eq_s2_interfacebox = InterfaceBoxBuilder::new_default(coach_set_2.id)
        .set_type_(Some("Wired".to_string()))
        .set_serial_number(Some("ahjsunkkdmif67nn87sk".to_string()))
        .build_and_insert(db)
        .unwrap();

    let eq_s2_monitor = MonitorBuilder::new_default(coach_set_2.id)
        .set_size(Some("27".to_string()))
        .set_brand(Some("Dell".to_string()))
        .set_has_vga_out_port(Some(true))
        .set_has_display_port_out(Some(false))
        .set_has_dvi_out_port(Some(false))
        .set_has_hdmi_out_port(Some(false))
        .build_and_insert(db)
        .unwrap();

    let eq_s2_microphone = MicrophoneRecorderBuilder::new_default(coach_set_2.id)
        .set_type_(Some("External".to_string()))
        .set_misc_note(Some("Casio".to_string()))
        .build_and_insert(db)
        .unwrap();

    let eq_s2_projector = ProjectorBuilder::new_default(coach_set_2.id)
        .set_brand(Some("Epson".to_string()))
        .set_has_vga_out_port(Some(true))
        .set_has_display_port_out(Some(true))
        .set_has_dvi_out_port(Some(true))
        .set_has_hdmi_out_port(Some(true))
        .build_and_insert(db)
        .unwrap();

    let eq_s2_powerstrip = PowerStripBuilder::new_default(coach_set_2.id)
        .set_make(Some("Belkin".to_string()))
        .set_model(Some("BE108000-06".to_string()))
        .set_color(Some("white".to_string()))
        .set_num_of_plugs(Some(8))
        .set_misc_note(Some("Old reliable".to_string()))
        .build_and_insert(db)
        .unwrap();

    let eq_s2_extensioncord = ExtensionCordBuilder::new_default(coach_set_2.id)
        .set_length(Some("6ft".to_string()))
        .set_color(Some("Black".to_string()))
        .set_misc_note(Some("The one that came from dad's house".to_string()))
        .build_and_insert(db)
        .unwrap();

    // Register all equipment from both sets with the tournament
    // Set 1 — fully deployed to Room 1; microphone held as spare on standby
    EquipmentRegistrationBuilder::new_default(eq_s1_computer.equipmentid, tour.tid)
        .set_status(Some(EquipmentRegistrationStatus::DeployedToRoom.to_string()))
        .set_roomid(Some(room_1.roomid))
        .build_and_insert(db).unwrap();
    EquipmentRegistrationBuilder::new_default(eq_s1_jumppad.equipmentid, tour.tid)
        .set_status(Some(EquipmentRegistrationStatus::DeployedToRoom.to_string()))
        .set_roomid(Some(room_1.roomid))
        .build_and_insert(db).unwrap();
    EquipmentRegistrationBuilder::new_default(eq_s1_interfacebox.equipmentid, tour.tid)
        .set_status(Some(EquipmentRegistrationStatus::DeployedToRoom.to_string()))
        .set_roomid(Some(room_1.roomid))
        .build_and_insert(db).unwrap();
    EquipmentRegistrationBuilder::new_default(eq_s1_monitor.equipmentid, tour.tid)
        .set_status(Some(EquipmentRegistrationStatus::DeployedToRoom.to_string()))
        .set_roomid(Some(room_1.roomid))
        .build_and_insert(db).unwrap();
    EquipmentRegistrationBuilder::new_default(eq_s1_microphone.equipmentid, tour.tid)
        .set_status(Some(EquipmentRegistrationStatus::OnStandby.to_string()))
        .build_and_insert(db).unwrap();
    EquipmentRegistrationBuilder::new_default(eq_s1_projector.equipmentid, tour.tid)
        .set_status(Some(EquipmentRegistrationStatus::DeployedToRoom.to_string()))
        .set_roomid(Some(room_1.roomid))
        .build_and_insert(db).unwrap();
    EquipmentRegistrationBuilder::new_default(eq_s1_powerstrip.equipmentid, tour.tid)
        .set_status(Some(EquipmentRegistrationStatus::DeployedToRoom.to_string()))
        .set_roomid(Some(room_1.roomid))
        .build_and_insert(db).unwrap();
    EquipmentRegistrationBuilder::new_default(eq_s1_extensioncord.equipmentid, tour.tid)
        .set_status(Some(EquipmentRegistrationStatus::DeployedToRoom.to_string()))
        .set_roomid(Some(room_1.roomid))
        .build_and_insert(db).unwrap();

    // Set 2 — deployed to Room 2; interface box flagged for repair, extension cord returned from room
    EquipmentRegistrationBuilder::new_default(eq_s2_computer.equipmentid, tour.tid)
        .set_status(Some(EquipmentRegistrationStatus::DeployedToRoom.to_string()))
        .set_roomid(Some(room_2.roomid))
        .build_and_insert(db).unwrap();
    EquipmentRegistrationBuilder::new_default(eq_s2_jumppad.equipmentid, tour.tid)
        .set_status(Some(EquipmentRegistrationStatus::DeployedToRoom.to_string()))
        .set_roomid(Some(room_2.roomid))
        .build_and_insert(db).unwrap();
    EquipmentRegistrationBuilder::new_default(eq_s2_interfacebox.equipmentid, tour.tid)
        .set_status(Some(EquipmentRegistrationStatus::NeedsRepair.to_string()))
        .build_and_insert(db).unwrap();
    EquipmentRegistrationBuilder::new_default(eq_s2_monitor.equipmentid, tour.tid)
        .set_status(Some(EquipmentRegistrationStatus::DeployedToRoom.to_string()))
        .set_roomid(Some(room_2.roomid))
        .build_and_insert(db).unwrap();
    EquipmentRegistrationBuilder::new_default(eq_s2_microphone.equipmentid, tour.tid)
        .set_status(Some(EquipmentRegistrationStatus::DeployedToRoom.to_string()))
        .set_roomid(Some(room_2.roomid))
        .build_and_insert(db).unwrap();
    EquipmentRegistrationBuilder::new_default(eq_s2_projector.equipmentid, tour.tid)
        .set_status(Some(EquipmentRegistrationStatus::DeployedToRoom.to_string()))
        .set_roomid(Some(room_2.roomid))
        .build_and_insert(db).unwrap();
    EquipmentRegistrationBuilder::new_default(eq_s2_powerstrip.equipmentid, tour.tid)
        .set_status(Some(EquipmentRegistrationStatus::DeployedToRoom.to_string()))
        .set_roomid(Some(room_2.roomid))
        .build_and_insert(db).unwrap();
    EquipmentRegistrationBuilder::new_default(eq_s2_extensioncord.equipmentid, tour.tid)
        .set_status(Some(EquipmentRegistrationStatus::ReturnedFromRoom.to_string()))
        .build_and_insert(db).unwrap();

    let q_exp_1_1 = UserBuilder::new("Aiden")
        .set_lname("Park (Demo)")
        .set_email("apark@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_1_2 = UserBuilder::new("Zoe")
        .set_lname("Nakamura (Demo)")
        .set_email("znakamura@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_1_3 = UserBuilder::new("Felix")
        .set_lname("Brennan (Demo)")
        .set_email("fbrennan@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();

    // Create Roster 1 for coach_exp_1, add quizzers to roster, then build the team
    let roster_1 = RosterBuilder::new("Roster 1", coach_exp_1.id)
        .set_description(Some("Coach's primary roster".to_string()))
        .build_and_insert(db)
        .unwrap();
    RosterCoachBuilder::new(coach_exp_1.id, roster_1.rosterid)
        .build_and_insert(db)
        .unwrap();
    // Roster is shared also with member user:
    RosterCoachBuilder::new(member_user.id, roster_1.rosterid)
        .build_and_insert(db)
        .unwrap();
    RosterQuizzerBuilder::new(q_exp_1_1.id, roster_1.rosterid)
        .build_and_insert(db)
        .unwrap();
    RosterQuizzerBuilder::new(q_exp_1_2.id, roster_1.rosterid)
        .build_and_insert(db)
        .unwrap();
    RosterQuizzerBuilder::new(tour_owner.id, roster_1.rosterid)
        .build_and_insert(db)
        .unwrap();

    // Create Roster 2 for coach_exp_1 with 3 new quizzers
    let q_r2_1 = UserBuilder::new("Harper")
        .set_lname("Ellison (Demo)")
        .set_email("hellison@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_r2_2 = UserBuilder::new("Rowan")
        .set_lname("Caldwell (Demo)")
        .set_email("rcaldwell@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_r2_3 = UserBuilder::new("Nadia")
        .set_lname("Ostrowski (Demo)")
        .set_email("nostrowski@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let roster_2 = RosterBuilder::new("Roster 2", coach_exp_1.id)
        .set_description(Some("Coach's secondary roster".to_string()))
        .build_and_insert(db)
        .unwrap();
    RosterCoachBuilder::new(coach_exp_1.id, roster_2.rosterid)
        .build_and_insert(db)
        .unwrap();
    RosterQuizzerBuilder::new(q_r2_1.id, roster_2.rosterid)
        .build_and_insert(db)
        .unwrap();
    RosterQuizzerBuilder::new(q_r2_2.id, roster_2.rosterid)
        .build_and_insert(db)
        .unwrap();
    RosterQuizzerBuilder::new(q_r2_3.id, roster_2.rosterid)
        .build_and_insert(db)
        .unwrap();

    let team_1_experienced = TeamBuilder::new_default(division_experienced.did)
        .set_name("Iron Covenant")
        .set_coachid(coach_exp_1.id)
        .set_quizzer_one_id(q_exp_1_1.id)
        .set_quizzer_two_id(q_exp_1_2.id)
        .set_quizzer_three_id(tour_owner.id)
        .build_and_insert(db)
        .unwrap();

    let coach_exp_2 = UserBuilder::new("Marcus")
        .set_lname("Osei (Demo)")
        .set_email("mosei@fakeemail.com")
        .set_username("mosei")
        .set_hash_password("Marc0s!Ose")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_exp_2_1 = UserBuilder::new("Simone")
        .set_lname("Tremblay (Demo)")
        .set_email("stremblay@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_2_2 = UserBuilder::new("Kwame")
        .set_lname("Asante (Demo)")
        .set_email("kasante@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_2_3 = UserBuilder::new("Nora")
        .set_lname("Lindqvist (Demo)")
        .set_email("nlindqvist@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let team_2_experienced = TeamBuilder::new_default(division_experienced.did)
        .set_name("Scarlet Vanguard")
        .set_coachid(coach_exp_2.id)
        .set_quizzer_one_id(q_exp_2_1.id)
        .set_quizzer_two_id(q_exp_2_2.id)
        .set_quizzer_three_id(q_exp_2_3.id)
        .build_and_insert(db)
        .unwrap();

    let coach_exp_3 = UserBuilder::new("Priya")
        .set_lname("Nair (Demo)")
        .set_email("pnair@fakeemail.com")
        .set_username("pnair")
        .set_hash_password("Priy@N4ir")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_exp_3_1 = UserBuilder::new("Dante")
        .set_lname("Moretti (Demo)")
        .set_email("dmoretti@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_3_2 = UserBuilder::new("Clara")
        .set_lname("Hoffmann (Demo)")
        .set_email("choffmann@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_3_3 = UserBuilder::new("Ravi")
        .set_lname("Sharma (Demo)")
        .set_email("rsharma@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let team_3_experienced = TeamBuilder::new_default(division_experienced.did)
        .set_name("Storm Cipher")
        .set_coachid(coach_exp_3.id)
        .set_quizzer_one_id(q_exp_3_1.id)
        .set_quizzer_two_id(q_exp_3_2.id)
        .set_quizzer_three_id(q_exp_3_3.id)
        .build_and_insert(db)
        .unwrap();

    let coach_exp_4 = UserBuilder::new("Derek")
        .set_lname("Calloway (Demo)")
        .set_email("dcalloway@fakeemail.com")
        .set_username("dcalloway")
        .set_hash_password("Derek#C4ll")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_exp_4_1 = UserBuilder::new("Yara")
        .set_lname("Hassan (Demo)")
        .set_email("yhassan@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_4_2 = UserBuilder::new("Owen")
        .set_lname("Fitzgerald (Demo)")
        .set_email("ofitzgerald@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_4_3 = UserBuilder::new("Mei")
        .set_lname("Lin (Demo)")
        .set_email("mlin@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let team_4_experienced = TeamBuilder::new_default(division_experienced.did)
        .set_name("Ember Watch")
        .set_coachid(coach_exp_4.id)
        .set_quizzer_one_id(q_exp_4_1.id)
        .set_quizzer_two_id(q_exp_4_2.id)
        .set_quizzer_three_id(q_exp_4_3.id)
        .build_and_insert(db)
        .unwrap();

    let coach_exp_5 = UserBuilder::new("Yuki")
        .set_lname("Tanaka (Demo)")
        .set_email("ytanaka@fakeemail.com")
        .set_username("ytanaka")
        .set_hash_password("Yuk1!Tank")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_exp_5_1 = UserBuilder::new("Ezra")
        .set_lname("Goldberg (Demo)")
        .set_email("egoldberg@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_5_2 = UserBuilder::new("Amira")
        .set_lname("Seif (Demo)")
        .set_email("aseif@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_5_3 = UserBuilder::new("Luke")
        .set_lname("Petrov (Demo)")
        .set_email("lpetrov@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let team_5_experienced = TeamBuilder::new_default(division_experienced.did)
        .set_name("Obsidian Pact")
        .set_coachid(coach_exp_5.id)
        .set_quizzer_one_id(q_exp_5_1.id)
        .set_quizzer_two_id(q_exp_5_2.id)
        .set_quizzer_three_id(q_exp_5_3.id)
        .build_and_insert(db)
        .unwrap();

    let coach_exp_6 = UserBuilder::new("Brianna")
        .set_lname("Flores (Demo)")
        .set_email("bflores@fakeemail.com")
        .set_username("bflores")
        .set_hash_password("Bri@Fl0re")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_exp_6_1 = UserBuilder::new("Isla")
        .set_lname("Mackenzie (Demo)")
        .set_email("imackenzie@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_6_2 = UserBuilder::new("Tomas")
        .set_lname("Vega (Demo)")
        .set_email("tvega@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_6_3 = UserBuilder::new("Hana")
        .set_lname("Iwata (Demo)")
        .set_email("hiwata@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let team_6_experienced = TeamBuilder::new_default(division_experienced.did)
        .set_name("Hollow Crown")
        .set_coachid(coach_exp_6.id)
        .set_quizzer_one_id(q_exp_6_1.id)
        .set_quizzer_two_id(q_exp_6_2.id)
        .set_quizzer_three_id(q_exp_6_3.id)
        .build_and_insert(db)
        .unwrap();

    let coach_exp_7 = UserBuilder::new("Nadia")
        .set_lname("Volkov (Demo)")
        .set_email("nvolkov@fakeemail.com")
        .set_username("nvolkov")
        .set_hash_password("Nad!aV0lk")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_exp_7_1 = UserBuilder::new("Priya")
        .set_lname("Menon (Demo)")
        .set_email("pmenon@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_7_2 = UserBuilder::new("Diego")
        .set_lname("Ramos (Demo)")
        .set_email("dramos@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_7_3 = UserBuilder::new("Anya")
        .set_lname("Sokolova (Demo)")
        .set_email("asokolova@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let team_7_experienced = TeamBuilder::new_default(division_experienced.did)
        .set_name("Argent Legion")
        .set_coachid(coach_exp_7.id)
        .set_quizzer_one_id(q_exp_7_1.id)
        .set_quizzer_two_id(q_exp_7_2.id)
        .set_quizzer_three_id(q_exp_7_3.id)
        .build_and_insert(db)
        .unwrap();

    let coach_exp_8 = UserBuilder::new("Rashid")
        .set_lname("Karimi (Demo)")
        .set_email("rkarimi@fakeemail.com")
        .set_username("rkarimi")
        .set_hash_password("Rash!dK4r")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_exp_8_1 = UserBuilder::new("Omar")
        .set_lname("Farouk (Demo)")
        .set_email("ofarouk@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_8_2 = UserBuilder::new("Lucia")
        .set_lname("Mendez (Demo)")
        .set_email("lmendez@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_8_3 = UserBuilder::new("Tariq")
        .set_lname("Aziz (Demo)")
        .set_email("taziz@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let team_8_experienced = TeamBuilder::new_default(division_experienced.did)
        .set_name("Crimson Oath")
        .set_coachid(coach_exp_8.id)
        .set_quizzer_one_id(q_exp_8_1.id)
        .set_quizzer_two_id(q_exp_8_2.id)
        .set_quizzer_three_id(q_exp_8_3.id)
        .build_and_insert(db)
        .unwrap();

    let coach_exp_9 = UserBuilder::new("Elena")
        .set_lname("Costa (Demo)")
        .set_email("ecosta@fakeemail.com")
        .set_username("ecosta")
        .set_hash_password("Elen@C0st")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_exp_9_1 = UserBuilder::new("Freya")
        .set_lname("Nilsson (Demo)")
        .set_email("fnilsson@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_9_2 = UserBuilder::new("Hugo")
        .set_lname("Bernard (Demo)")
        .set_email("hbernard@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_9_3 = UserBuilder::new("Meera")
        .set_lname("Iyer (Demo)")
        .set_email("miyer@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let team_9_experienced = TeamBuilder::new_default(division_experienced.did)
        .set_name("Void Sentinels")
        .set_coachid(coach_exp_9.id)
        .set_quizzer_one_id(q_exp_9_1.id)
        .set_quizzer_two_id(q_exp_9_2.id)
        .set_quizzer_three_id(q_exp_9_3.id)
        .build_and_insert(db)
        .unwrap();

    let coach_exp_10 = UserBuilder::new("Malik")
        .set_lname("Johnson (Demo)")
        .set_email("mjohnson@fakeemail.com")
        .set_username("mjohnson")
        .set_hash_password("Mal!kJ0hn")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_exp_10_1 = UserBuilder::new("Sven")
        .set_lname("Larsen (Demo)")
        .set_email("slarsen@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_10_2 = UserBuilder::new("Rania")
        .set_lname("Haddad (Demo)")
        .set_email("rhaddad@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_10_3 = UserBuilder::new("Felix")
        .set_lname("Braun (Demo)")
        .set_email("fbraun@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let team_10_experienced = TeamBuilder::new_default(division_experienced.did)
        .set_name("Golden Reverie")
        .set_coachid(coach_exp_10.id)
        .set_quizzer_one_id(q_exp_10_1.id)
        .set_quizzer_two_id(q_exp_10_2.id)
        .set_quizzer_three_id(q_exp_10_3.id)
        .build_and_insert(db)
        .unwrap();

    let coach_exp_11 = UserBuilder::new("Sofia")
        .set_lname("Rossi (Demo)")
        .set_email("srossi@fakeemail.com")
        .set_username("srossi")
        .set_hash_password("Sof!aR0ss")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_exp_11_1 = UserBuilder::new("Ingrid")
        .set_lname("Holm (Demo)")
        .set_email("iholm@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_11_2 = UserBuilder::new("Carlos")
        .set_lname("Vargas (Demo)")
        .set_email("cvargas@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_11_3 = UserBuilder::new("Yasmin")
        .set_lname("Ali (Demo)")
        .set_email("yali@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let team_11_experienced = TeamBuilder::new_default(division_experienced.did)
        .set_name("Frost Dominion")
        .set_coachid(coach_exp_11.id)
        .set_quizzer_one_id(q_exp_11_1.id)
        .set_quizzer_two_id(q_exp_11_2.id)
        .set_quizzer_three_id(q_exp_11_3.id)
        .build_and_insert(db)
        .unwrap();

    let coach_exp_12 = UserBuilder::new("Kenji")
        .set_lname("Watanabe (Demo)")
        .set_email("kwatanabe@fakeemail.com")
        .set_username("kwatanabe")
        .set_hash_password("Kenj!W4ta")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_exp_12_1 = UserBuilder::new("Bjorn")
        .set_lname("Eriksson (Demo)")
        .set_email("beriksson@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_12_2 = UserBuilder::new("Leila")
        .set_lname("Nasser (Demo)")
        .set_email("lnasser@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_12_3 = UserBuilder::new("Andre")
        .set_lname("Silva (Demo)")
        .set_email("asilva@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let team_12_experienced = TeamBuilder::new_default(division_experienced.did)
        .set_name("Umbral Ascent")
        .set_coachid(coach_exp_12.id)
        .set_quizzer_one_id(q_exp_12_1.id)
        .set_quizzer_two_id(q_exp_12_2.id)
        .set_quizzer_three_id(q_exp_12_3.id)
        .build_and_insert(db)
        .unwrap();

    // Rounds are created inside the scheduling block below (they belong to division roundgroups).

    // Div: Novice
    let coach_nov_1 = UserBuilder::new("Samuel")
        .set_lname("Ebert (Demo)")
        .set_email("sebert@fakeemail.com")
        .set_username("sebert")
        .set_hash_password("Sam#Eb3rt")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_nov_1_1 = UserBuilder::new("Elijah")
        .set_lname("Brooks (Demo)")
        .set_email("ebrooks@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_nov_1_2 = UserBuilder::new("Sofia")
        .set_lname("Reyes (Demo)")
        .set_email("sreyes@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_nov_1_3 = UserBuilder::new("Liam")
        .set_lname("Okafor (Demo)")
        .set_email("lokafor@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let team_1_novice = TeamBuilder::new_default(division_novice.did)
        .set_name("Neon Prophets")
        .set_coachid(tour_owner.id)
        .set_quizzer_one_id(q_nov_1_1.id)
        .set_quizzer_two_id(q_nov_1_2.id)
        .set_quizzer_three_id(q_nov_1_3.id)
        .build_and_insert(db)
        .unwrap();

    let coach_nov_2 = UserBuilder::new("Fatima")
        .set_lname("Rashid (Demo)")
        .set_email("frashid@fakeemail.com")
        .set_username("frashid")
        .set_hash_password("Fat!Ra5hid")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_nov_2_1 = UserBuilder::new("Mia")
        .set_lname("Johansson (Demo)")
        .set_email("mjohansson@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_nov_2_2 = UserBuilder::new("Caleb")
        .set_lname("Patel (Demo)")
        .set_email("cpatel@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_nov_2_3 = UserBuilder::new("Aria")
        .set_lname("Novak (Demo)")
        .set_email("anovak@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let team_2_novice = TeamBuilder::new_default(division_novice.did)
        .set_name("Silver Requiem")
        .set_coachid(coach_nov_2.id)
        .set_quizzer_one_id(q_nov_2_1.id)
        .set_quizzer_two_id(q_nov_2_2.id)
        .set_quizzer_three_id(q_nov_2_3.id)
        .build_and_insert(db)
        .unwrap();

    let coach_nov_3 = UserBuilder::new("Connor")
        .set_lname("Walsh (Demo)")
        .set_email("cwalsh@fakeemail.com")
        .set_username("cwalsh")
        .set_hash_password("Con@W4lsh")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_nov_3_1 = UserBuilder::new("Noah")
        .set_lname("Ferreira (Demo)")
        .set_email("nferreira@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_nov_3_2 = UserBuilder::new("Stella")
        .set_lname("Kim (Demo)")
        .set_email("skim@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_nov_3_3 = UserBuilder::new("James")
        .set_lname("Mbeki (Demo)")
        .set_email("jmbeki@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let team_3_novice = TeamBuilder::new_default(division_novice.did)
        .set_name("Jade Insurgents")
        .set_coachid(coach_nov_3.id)
        .set_quizzer_one_id(q_nov_3_1.id)
        .set_quizzer_two_id(q_nov_3_2.id)
        .set_quizzer_three_id(q_nov_3_3.id)
        .build_and_insert(db)
        .unwrap();

    let coach_nov_4 = UserBuilder::new("Ngozi")
        .set_lname("Eze (Demo)")
        .set_email("neze@fakeemail.com")
        .set_username("neze")
        .set_hash_password("Ngoz!3Eze")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_nov_4_1 = UserBuilder::new("Leah")
        .set_lname("Christensen (Demo)")
        .set_email("lchristensen@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_nov_4_2 = UserBuilder::new("Omar")
        .set_lname("Saleh (Demo)")
        .set_email("osaleh@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_nov_4_3 = UserBuilder::new("Ruby")
        .set_lname("Nguyen (Demo)")
        .set_email("rnguyen@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let team_4_novice = TeamBuilder::new_default(division_novice.did)
        .set_name("Cobalt Rising")
        .set_coachid(coach_nov_4.id)
        .set_quizzer_one_id(q_nov_4_1.id)
        .set_quizzer_two_id(q_nov_4_2.id)
        .set_quizzer_three_id(q_nov_4_3.id)
        .build_and_insert(db)
        .unwrap();

    let coach_nov_5 = UserBuilder::new("Priya")
        .set_lname("Kapoor (Demo)")
        .set_email("pkapoor@fakeemail.com")
        .set_username("pkapoor")
        .set_hash_password("Pr!yaKap5")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_nov_5_1 = UserBuilder::new("Mateo")
        .set_lname("Rossi (Demo)")
        .set_email("mrossi@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_nov_5_2 = UserBuilder::new("Aisha")
        .set_lname("Bello (Demo)")
        .set_email("abello@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_nov_5_3 = UserBuilder::new("Nadia")
        .set_lname("Haddad (Demo)")
        .set_email("nhaddad@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let team_5_novice = TeamBuilder::new_default(division_novice.did)
        .set_name("Amber Vanguard")
        .set_coachid(coach_nov_5.id)
        .set_quizzer_one_id(q_nov_5_1.id)
        .set_quizzer_two_id(q_nov_5_2.id)
        .set_quizzer_three_id(q_nov_5_3.id)
        .build_and_insert(db)
        .unwrap();

    // (Novice rounds are created inside the scheduling block below.)

    // Div: Decades
    let coach_dec_1 = UserBuilder::new("Pavel")
        .set_lname("Sorokin (Demo)")
        .set_email("psorokin@fakeemail.com")
        .set_username("psorokin")
        .set_hash_password("Pav@S0rok")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_dec_1_1 = UserBuilder::new("Finn")
        .set_lname("Gallagher (Demo)")
        .set_email("fgallagher@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_dec_1_2 = UserBuilder::new("Zara")
        .set_lname("Ahmed (Demo)")
        .set_email("zahmed@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_dec_1_3 = UserBuilder::new("Cole")
        .set_lname("Marchetti (Demo)")
        .set_email("cmarchetti@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let team_1_decades = TeamBuilder::new_default(division_decades.did)
        .set_name("Gilt Archive")
        .set_coachid(coach_dec_1.id)
        .set_quizzer_one_id(q_dec_1_1.id)
        .set_quizzer_two_id(q_dec_1_2.id)
        .set_quizzer_three_id(q_dec_1_3.id)
        .build_and_insert(db)
        .unwrap();

    let coach_dec_2 = UserBuilder::new("Amara")
        .set_lname("Diallo (Demo)")
        .set_email("adiallo@fakeemail.com")
        .set_username("adiallo")
        .set_hash_password("Amar#Di4l")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_dec_2_1 = UserBuilder::new("Ivy")
        .set_lname("Chen (Demo)")
        .set_email("ichen@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_dec_2_2 = UserBuilder::new("Declan")
        .set_lname("Murphy (Demo)")
        .set_email("dmurphy@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_dec_2_3 = UserBuilder::new("Layla")
        .set_lname("Espinoza (Demo)")
        .set_email("lespinoza@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let team_2_decades = TeamBuilder::new_default(division_decades.did)
        .set_name("The Relics")
        .set_coachid(coach_dec_2.id)
        .set_quizzer_one_id(q_dec_2_1.id)
        .set_quizzer_two_id(q_dec_2_2.id)
        .set_quizzer_three_id(q_dec_2_3.id)
        .build_and_insert(db)
        .unwrap();

    let coach_dec_3 = UserBuilder::new("Tobias")
        .set_lname("Reinhardt (Demo)")
        .set_email("treinhardt@fakeemail.com")
        .set_username("treinhardt")
        .set_hash_password("Tobi!R3in")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_dec_3_1 = UserBuilder::new("Asher")
        .set_lname("Friedman (Demo)")
        .set_email("afriedman@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_dec_3_2 = UserBuilder::new("Cleo")
        .set_lname("Papadopoulos (Demo)")
        .set_email("cpapadopoulos@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_dec_3_3 = UserBuilder::new("Theo")
        .set_lname("Nkosi (Demo)")
        .set_email("tnkosi@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let team_3_decades = TeamBuilder::new_default(division_decades.did)
        .set_name("Rust & Rime")
        .set_coachid(coach_dec_3.id)
        .set_quizzer_one_id(q_dec_3_1.id)
        .set_quizzer_two_id(q_dec_3_2.id)
        .set_quizzer_three_id(q_dec_3_3.id)
        .build_and_insert(db)
        .unwrap();

    let coach_dec_4 = UserBuilder::new("Miriam")
        .set_lname("Hollis (Demo)")
        .set_email("mhollis@fakeemail.com")
        .set_username("mhollis")
        .set_hash_password("Mir@H0lli")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_dec_4_1 = UserBuilder::new("Vera")
        .set_lname("Kuznetsova (Demo)")
        .set_email("vkuznetsova@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_dec_4_2 = UserBuilder::new("Miles")
        .set_lname("Oduya (Demo)")
        .set_email("moduya@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_dec_4_3 = UserBuilder::new("Sasha")
        .set_lname("Petersen (Demo)")
        .set_email("spetersen@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let team_4_decades = TeamBuilder::new_default(division_decades.did)
        .set_name("Echo Epoch")
        .set_coachid(coach_dec_4.id)
        .set_quizzer_one_id(q_dec_4_1.id)
        .set_quizzer_two_id(q_dec_4_2.id)
        .set_quizzer_three_id(q_dec_4_3.id)
        .build_and_insert(db)
        .unwrap();

    // (Decades rounds are created inside the scheduling block below.)

    // Coaches get the member role regardless of whether a schedule has been defined yet.
    let member_role = crate::models::role::read_by_name(db, "member").unwrap();
    let coaches = [
        coach_exp_1.id, coach_exp_2.id, coach_exp_3.id,
        coach_exp_4.id, coach_exp_5.id, coach_exp_6.id,
        coach_exp_7.id, coach_exp_8.id, coach_exp_9.id,
        coach_exp_10.id, coach_exp_11.id, coach_exp_12.id,
        coach_nov_2.id, coach_nov_3.id, coach_nov_4.id,
        coach_dec_1.id, coach_dec_2.id, coach_dec_3.id, coach_dec_4.id,
    ];
    for coach_id in coaches {
        UsersRolesBuilder::new(coach_id)
            .assign(member_role.id)
            .build_and_insert(db)
            .unwrap();
    }

    // ── Schedule ────────────────────────────────────────────────────────────────
    // Everything below this point — division roundgroups, pools/brackets, team placements, games, and
    // the game events nested under `include_gameevents` — is the tournament's schedule. When
    // `include_scheduling` is false the seed stops here: a tournament with registered teams and gear
    // but no schedule yet, ready for the manager/admins to define one. Because games are only created
    // past this guard, no game events are ever added without a schedule even if `include_gameevents`
    // is true (there would be no games to attach them to).
    if !include_scheduling {
        return;
    }

    // Each division runs two roundgroups ("Session 1" then "Session 2"). Session 1 for every division
    // happens concurrently, so those games use disjoint rooms (Experienced 1-2, Novice 4-5,
    // Decades 6-7); Session 2 happens afterward and reuses the same rooms.
    //
    // Experienced's Session 1 is split into two pools (its teams divided evenly across them — see the
    // team groups seeded below); every other roundgroup is a single pool holding all of that division's
    // teams. Games must reference a real pool bracket — there is no auto-default.
    // Experienced plays each pair twice, split by session: Session 1 is every pair's first meeting,
    // Session 2 the second. Each session has its own poolbracketgroup holding that session's three
    // pools, so a team is placed once per group. Pool names must be unique within the division, so
    // they carry the session number (A1/B1/C1, then A2/B2/C2) — the same convention Novice uses.
    let roundgroup_exp_1 = RoundGroupBuilder::new(division_experienced.did)
        .set_name("Session 1")
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    let roundgroup_exp_2 = RoundGroupBuilder::new(division_experienced.did)
        .set_name("Session 2")
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    let pbg_exp_1 = PoolBracketGroupBuilder::new(division_experienced.did)
        .set_name("Group 1")
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    let pbg_exp_2 = PoolBracketGroupBuilder::new(division_experienced.did)
        .set_name("Group 2")
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    let mut exp_pools_by_session: Vec<Vec<Uuid>> = Vec::new();
    for (group_id, suffix) in [
        (pbg_exp_1.poolbracketgroupid, 1),
        (pbg_exp_2.poolbracketgroupid, 2),
    ] {
        let mut pools = Vec::new();
        for letter in ['A', 'B', 'C'] {
            let pool = PoolBracketBuilder::new(division_experienced.did)
                .set_name(&format!("Pool {}{}", letter, suffix))
                .set_poolbracketgroupid(group_id)
                .set_creator_userid(tour_owner.id)
                .build_and_insert(db)
                .unwrap();
            pools.push(pool.pool_bracket_id);
        }
        exp_pools_by_session.push(pools);
    }

    let roundgroup_nov_1 = RoundGroupBuilder::new(division_novice.did)
        .set_name("Session 1")
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    let pbg_nov_1 = PoolBracketGroupBuilder::new(division_novice.did)
        .set_name("Group 1")
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    // Novice's two round-robins live in separate pools, each in its own pool group (so every team is
    // placed once per group): Session 1 → pool A1 (Group 1), Session 2 → pool A2 (Group 2).
    let pb_nov_1 = PoolBracketBuilder::new(division_novice.did)
        .set_name("A1")
        .set_poolbracketgroupid(pbg_nov_1.poolbracketgroupid)
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    let pbg_nov_2 = PoolBracketGroupBuilder::new(division_novice.did)
        .set_name("Group 2")
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    let pb_nov_2 = PoolBracketBuilder::new(division_novice.did)
        .set_name("A2")
        .set_poolbracketgroupid(pbg_nov_2.poolbracketgroupid)
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    let roundgroup_nov_2 = RoundGroupBuilder::new(division_novice.did)
        .set_name("Session 2")
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();

    let roundgroup_dec_1 = RoundGroupBuilder::new(division_decades.did)
        .set_name("Session 1")
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    let pbg_dec_1 = PoolBracketGroupBuilder::new(division_decades.did)
        .set_name("Group 1")
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    // Decades' two round-robins live in separate pools that share a single pool group: Session 1 →
    // pool A1, Session 2 → pool A2 (both in Group 1, so every team appears in both pools of the group).
    let pb_dec_1 = PoolBracketBuilder::new(division_decades.did)
        .set_name("A1")
        .set_poolbracketgroupid(pbg_dec_1.poolbracketgroupid)
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    let pb_dec_2 = PoolBracketBuilder::new(division_decades.did)
        .set_name("A2")
        .set_poolbracketgroupid(pbg_dec_1.poolbracketgroupid)
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    let roundgroup_dec_2 = RoundGroupBuilder::new(division_decades.did)
        .set_name("Session 2")
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();

    // Rounds belong to (time-bound) division roundgroups. Experienced's single roundgroup runs 5 rounds
    // (12:00-14:00); Novice and Decades each run 3 rounds per roundgroup (Session 1 12:00-13:00,
    // Session 2 13:30-14:30). Both Experienced pools share these same 5 rounds.
    let round_exp_s1_1 = RoundBuilder::new_default(roundgroup_exp_1.roundgroup_id)
        .set_last_modified_user(tour_owner.id).set_name("1")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 12,  0, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_exp_s1_2 = RoundBuilder::new_default(roundgroup_exp_1.roundgroup_id)
        .set_last_modified_user(tour_owner.id).set_name("2")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 12, 30, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_exp_s1_3 = RoundBuilder::new_default(roundgroup_exp_1.roundgroup_id)
        .set_last_modified_user(tour_owner.id).set_name("3")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 13,  0, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_exp_s2_4 = RoundBuilder::new_default(roundgroup_exp_2.roundgroup_id)
        .set_last_modified_user(tour_owner.id).set_name("4")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 13, 30, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_exp_s2_5 = RoundBuilder::new_default(roundgroup_exp_2.roundgroup_id)
        .set_last_modified_user(tour_owner.id).set_name("5")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 14,  0, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_exp_s2_6 = RoundBuilder::new_default(roundgroup_exp_2.roundgroup_id)
        .set_last_modified_user(tour_owner.id).set_name("6")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 14, 30, 0).unwrap())
        .build_and_insert(db).unwrap();

    // Novice has 5 teams, so each session is a full 5-team single round-robin: 5 rounds, 2 games per
    // round with one team on a bye. Both sessions run the same round-robin (teams face each other twice).
    let round_nov_s1_1 = RoundBuilder::new_default(roundgroup_nov_1.roundgroup_id)
        .set_last_modified_user(tour_owner.id).set_name("1")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 12,  0, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_nov_s1_2 = RoundBuilder::new_default(roundgroup_nov_1.roundgroup_id)
        .set_last_modified_user(tour_owner.id).set_name("2")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 12, 30, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_nov_s1_3 = RoundBuilder::new_default(roundgroup_nov_1.roundgroup_id)
        .set_last_modified_user(tour_owner.id).set_name("3")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 13,  0, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_nov_s1_4 = RoundBuilder::new_default(roundgroup_nov_1.roundgroup_id)
        .set_last_modified_user(tour_owner.id).set_name("4")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 13, 30, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_nov_s1_5 = RoundBuilder::new_default(roundgroup_nov_1.roundgroup_id)
        .set_last_modified_user(tour_owner.id).set_name("5")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 14,  0, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_nov_s2_1 = RoundBuilder::new_default(roundgroup_nov_2.roundgroup_id)
        .set_last_modified_user(tour_owner.id).set_name("6")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 14, 30, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_nov_s2_2 = RoundBuilder::new_default(roundgroup_nov_2.roundgroup_id)
        .set_last_modified_user(tour_owner.id).set_name("7")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 15,  0, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_nov_s2_3 = RoundBuilder::new_default(roundgroup_nov_2.roundgroup_id)
        .set_last_modified_user(tour_owner.id).set_name("8")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 15, 30, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_nov_s2_4 = RoundBuilder::new_default(roundgroup_nov_2.roundgroup_id)
        .set_last_modified_user(tour_owner.id).set_name("9")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 16,  0, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_nov_s2_5 = RoundBuilder::new_default(roundgroup_nov_2.roundgroup_id)
        .set_last_modified_user(tour_owner.id).set_name("10")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 16, 30, 0).unwrap())
        .build_and_insert(db).unwrap();

    let round_dec_s1_1 = RoundBuilder::new_default(roundgroup_dec_1.roundgroup_id)
        .set_last_modified_user(tour_owner.id).set_name("1")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 12,  0, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_dec_s1_2 = RoundBuilder::new_default(roundgroup_dec_1.roundgroup_id)
        .set_last_modified_user(tour_owner.id).set_name("2")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 12, 30, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_dec_s1_3 = RoundBuilder::new_default(roundgroup_dec_1.roundgroup_id)
        .set_last_modified_user(tour_owner.id).set_name("3")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 13,  0, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_dec_s2_1 = RoundBuilder::new_default(roundgroup_dec_2.roundgroup_id)
        .set_last_modified_user(tour_owner.id).set_name("4")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 13, 30, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_dec_s2_2 = RoundBuilder::new_default(roundgroup_dec_2.roundgroup_id)
        .set_last_modified_user(tour_owner.id).set_name("5")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 14,  0, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_dec_s2_3 = RoundBuilder::new_default(roundgroup_dec_2.roundgroup_id)
        .set_last_modified_user(tour_owner.id).set_name("6")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 14, 30, 0).unwrap())
        .build_and_insert(db).unwrap();

    // Team groups: each pool/bracket owns exactly one team group holding its teams. Experienced's
    // 12 teams are split evenly across three pools (1-4 in A, 5-8 in B, 9-12 in C), and each
    // session's group repeats that split — so a team is placed once per pool group. Every other
    // division's pool holds all of its teams.
    let exp_pool_rosters: [[Uuid; 4]; 3] = [
        [team_1_experienced.teamid, team_2_experienced.teamid,
         team_3_experienced.teamid, team_4_experienced.teamid],
        [team_5_experienced.teamid, team_6_experienced.teamid,
         team_7_experienced.teamid, team_8_experienced.teamid],
        [team_9_experienced.teamid, team_10_experienced.teamid,
         team_11_experienced.teamid, team_12_experienced.teamid],
    ];
    for session_pools in &exp_pools_by_session {
        for (pool_id, roster) in session_pools.iter().zip(exp_pool_rosters.iter()) {
            place_teams_in_pool(db, *pool_id, tour_owner.id, roster);
        }
    }
    // Novice: all five teams play in each pool (one pool per group), so place them in A1 and A2.
    for pb in [pb_nov_1.pool_bracket_id, pb_nov_2.pool_bracket_id] {
        place_teams_in_pool(db, pb, tour_owner.id, &[
            team_1_novice.teamid, team_2_novice.teamid, team_3_novice.teamid, team_4_novice.teamid,
            team_5_novice.teamid,
        ]);
    }
    // Decades: all four teams play in both pools of the single group, so place them in A1 and A2.
    for pb in [pb_dec_1.pool_bracket_id, pb_dec_2.pool_bracket_id] {
        place_teams_in_pool(db, pb, tour_owner.id, &[
            team_1_decades.teamid, team_2_decades.teamid, team_3_decades.teamid, team_4_decades.teamid,
        ]);
    }

    // Games — round-robin schedule. Session 1 games use disjoint rooms across divisions (they run
    // concurrently); Session 2 reuses those rooms afterward. Each game records a (gid, left team,
    // right team) spec here as it's built; the actual game-event streams are seeded together, in one
    // timed pass, after all games exist (see the `include_gameevents` block below).
    let mut game_event_specs: Vec<(Uuid, &Team, &Team)> = Vec::new();

    // Div: Experienced — Session 1, three pools of four teams. Each pool plays a double
    // round-robin, so every team faces each of its three pool-mates twice: 6 rounds x 2 games per
    // pool = 12 games per pool, 36 in all. The three pools run concurrently, each owning two of
    // Building A's rooms (Pool A -> rooms 1-2, Pool B -> 3-4, Pool C -> 5-6; qm per room, rm1 ->
    // qm1 ... rm6 -> qm6), so no room or quizmaster is ever double-booked within a round.
    // Rounds 1-3 sit in Session 1 and 4-6 in Session 2, which lines up with the pairing table: its
    // first three rounds are each pair's first meeting and its last three are the rematches. Each
    // game is filed under its own session's pool.
    let exp_rounds = [
        &round_exp_s1_1, &round_exp_s1_2, &round_exp_s1_3,
        &round_exp_s2_4, &round_exp_s2_5, &round_exp_s2_6,
    ];
    let exp_pool_members: [([&Team; 4], [(Uuid, Uuid); 2]); 3] = [
        (
            [&team_1_experienced, &team_2_experienced, &team_3_experienced, &team_4_experienced],
            [(room_1.roomid, qm_1.id), (room_2.roomid, qm_2.id)],
        ),
        (
            [&team_5_experienced, &team_6_experienced, &team_7_experienced, &team_8_experienced],
            [(room_3.roomid, qm_3.id), (room_4.roomid, qm_4.id)],
        ),
        (
            [&team_9_experienced, &team_10_experienced, &team_11_experienced, &team_12_experienced],
            [(room_5.roomid, qm_5.id), (room_6.roomid, qm_6.id)],
        ),
    ];
    let rounds_per_session = exp_rounds.len() / exp_pools_by_session.len();
    for (round_idx, round) in exp_rounds.iter().enumerate() {
        let session_pools = &exp_pools_by_session[round_idx / rounds_per_session];
        for (pool_idx, (teams, rooms)) in exp_pool_members.iter().enumerate() {
            let pool_id = session_pools[pool_idx];
            for (slot, (left_idx, right_idx)) in FOUR_TEAM_DOUBLE_ROUND_ROBIN[round_idx].iter().enumerate() {
                let (room_id, quizmaster_id) = rooms[slot];
                let left = teams[*left_idx];
                let right = teams[*right_idx];
                let game = GameBuilder::new_default(room_id, round.roundid)
                    .set_poolbracket_id(pool_id)
                    .set_leftteamid(left.teamid).set_rightteamid(right.teamid)
                    .set_quizmasterid(quizmaster_id).build_and_insert(db).unwrap();
                game_event_specs.push((game.gid, left, right));
            }
        }
    }

    // Div: Novice — Session 1: full 5-team round-robin, 10 games / 5 rounds (2 games/round, one bye
    // each round). rm7 → qm4, rm8 → qm5. Byes: R1→t1, R2→t2, R3→t3, R4→t4, R5→t5.
    // Round 1: (t2n,t5n)→rm7, (t3n,t4n)→rm8   [t1 bye]
    let game = GameBuilder::new_default(room_7.roomid, round_nov_s1_1.roundid)
        .set_poolbracket_id(pb_nov_1.pool_bracket_id)
        .set_leftteamid(team_2_novice.teamid).set_rightteamid(team_5_novice.teamid)
        .set_quizmasterid(qm_4.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_2_novice, &team_5_novice));
    let game = GameBuilder::new_default(room_8.roomid, round_nov_s1_1.roundid)
        .set_poolbracket_id(pb_nov_1.pool_bracket_id)
        .set_leftteamid(team_3_novice.teamid).set_rightteamid(team_4_novice.teamid)
        .set_quizmasterid(qm_5.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_3_novice, &team_4_novice));
    // Round 2: (t1n,t3n)→rm7, (t4n,t5n)→rm8   [t2 bye]
    let game = GameBuilder::new_default(room_7.roomid, round_nov_s1_2.roundid)
        .set_poolbracket_id(pb_nov_1.pool_bracket_id)
        .set_leftteamid(team_1_novice.teamid).set_rightteamid(team_3_novice.teamid)
        .set_quizmasterid(qm_4.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_1_novice, &team_3_novice));
    let game = GameBuilder::new_default(room_8.roomid, round_nov_s1_2.roundid)
        .set_poolbracket_id(pb_nov_1.pool_bracket_id)
        .set_leftteamid(team_4_novice.teamid).set_rightteamid(team_5_novice.teamid)
        .set_quizmasterid(qm_5.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_4_novice, &team_5_novice));
    // Round 3: (t2n,t4n)→rm7, (t1n,t5n)→rm8   [t3 bye]
    let game = GameBuilder::new_default(room_7.roomid, round_nov_s1_3.roundid)
        .set_poolbracket_id(pb_nov_1.pool_bracket_id)
        .set_leftteamid(team_2_novice.teamid).set_rightteamid(team_4_novice.teamid)
        .set_quizmasterid(qm_4.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_2_novice, &team_4_novice));
    let game = GameBuilder::new_default(room_8.roomid, round_nov_s1_3.roundid)
        .set_poolbracket_id(pb_nov_1.pool_bracket_id)
        .set_leftteamid(team_1_novice.teamid).set_rightteamid(team_5_novice.teamid)
        .set_quizmasterid(qm_5.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_1_novice, &team_5_novice));
    // Round 4: (t3n,t5n)→rm7, (t1n,t2n)→rm8   [t4 bye]
    let game = GameBuilder::new_default(room_7.roomid, round_nov_s1_4.roundid)
        .set_poolbracket_id(pb_nov_1.pool_bracket_id)
        .set_leftteamid(team_3_novice.teamid).set_rightteamid(team_5_novice.teamid)
        .set_quizmasterid(qm_4.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_3_novice, &team_5_novice));
    let game = GameBuilder::new_default(room_8.roomid, round_nov_s1_4.roundid)
        .set_poolbracket_id(pb_nov_1.pool_bracket_id)
        .set_leftteamid(team_1_novice.teamid).set_rightteamid(team_2_novice.teamid)
        .set_quizmasterid(qm_5.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_1_novice, &team_2_novice));
    // Round 5: (t1n,t4n)→rm7, (t2n,t3n)→rm8   [t5 bye]
    let game = GameBuilder::new_default(room_7.roomid, round_nov_s1_5.roundid)
        .set_poolbracket_id(pb_nov_1.pool_bracket_id)
        .set_leftteamid(team_1_novice.teamid).set_rightteamid(team_4_novice.teamid)
        .set_quizmasterid(qm_4.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_1_novice, &team_4_novice));
    let game = GameBuilder::new_default(room_8.roomid, round_nov_s1_5.roundid)
        .set_poolbracket_id(pb_nov_1.pool_bracket_id)
        .set_leftteamid(team_2_novice.teamid).set_rightteamid(team_3_novice.teamid)
        .set_quizmasterid(qm_5.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_2_novice, &team_3_novice));

    // Div: Novice — Session 2: the same full 5-team round-robin again (teams face each other twice),
    // 10 games / 5 rounds in rooms 7-8. rm7 → qm4, rm8 → qm5.
    // Round 1: (t2n,t5n)→rm7, (t3n,t4n)→rm8   [t1 bye]
    let game = GameBuilder::new_default(room_7.roomid, round_nov_s2_1.roundid)
        .set_poolbracket_id(pb_nov_2.pool_bracket_id)
        .set_leftteamid(team_2_novice.teamid).set_rightteamid(team_5_novice.teamid)
        .set_quizmasterid(qm_4.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_2_novice, &team_5_novice));
    let game = GameBuilder::new_default(room_8.roomid, round_nov_s2_1.roundid)
        .set_poolbracket_id(pb_nov_2.pool_bracket_id)
        .set_leftteamid(team_3_novice.teamid).set_rightteamid(team_4_novice.teamid)
        .set_quizmasterid(qm_5.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_3_novice, &team_4_novice));
    // Round 2: (t1n,t3n)→rm7, (t4n,t5n)→rm8   [t2 bye]
    let game = GameBuilder::new_default(room_7.roomid, round_nov_s2_2.roundid)
        .set_poolbracket_id(pb_nov_2.pool_bracket_id)
        .set_leftteamid(team_1_novice.teamid).set_rightteamid(team_3_novice.teamid)
        .set_quizmasterid(qm_4.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_1_novice, &team_3_novice));
    let game = GameBuilder::new_default(room_8.roomid, round_nov_s2_2.roundid)
        .set_poolbracket_id(pb_nov_2.pool_bracket_id)
        .set_leftteamid(team_4_novice.teamid).set_rightteamid(team_5_novice.teamid)
        .set_quizmasterid(qm_5.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_4_novice, &team_5_novice));
    // Round 3: (t2n,t4n)→rm7, (t1n,t5n)→rm8   [t3 bye]
    let game = GameBuilder::new_default(room_7.roomid, round_nov_s2_3.roundid)
        .set_poolbracket_id(pb_nov_2.pool_bracket_id)
        .set_leftteamid(team_2_novice.teamid).set_rightteamid(team_4_novice.teamid)
        .set_quizmasterid(qm_4.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_2_novice, &team_4_novice));
    let game = GameBuilder::new_default(room_8.roomid, round_nov_s2_3.roundid)
        .set_poolbracket_id(pb_nov_2.pool_bracket_id)
        .set_leftteamid(team_1_novice.teamid).set_rightteamid(team_5_novice.teamid)
        .set_quizmasterid(qm_5.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_1_novice, &team_5_novice));
    // Round 4: (t3n,t5n)→rm7, (t1n,t2n)→rm8   [t4 bye]
    let game = GameBuilder::new_default(room_7.roomid, round_nov_s2_4.roundid)
        .set_poolbracket_id(pb_nov_2.pool_bracket_id)
        .set_leftteamid(team_3_novice.teamid).set_rightteamid(team_5_novice.teamid)
        .set_quizmasterid(qm_4.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_3_novice, &team_5_novice));
    let game = GameBuilder::new_default(room_8.roomid, round_nov_s2_4.roundid)
        .set_poolbracket_id(pb_nov_2.pool_bracket_id)
        .set_leftteamid(team_1_novice.teamid).set_rightteamid(team_2_novice.teamid)
        .set_quizmasterid(qm_5.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_1_novice, &team_2_novice));
    // Round 5: (t1n,t4n)→rm7, (t2n,t3n)→rm8   [t5 bye]
    let game = GameBuilder::new_default(room_7.roomid, round_nov_s2_5.roundid)
        .set_poolbracket_id(pb_nov_2.pool_bracket_id)
        .set_leftteamid(team_1_novice.teamid).set_rightteamid(team_4_novice.teamid)
        .set_quizmasterid(qm_4.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_1_novice, &team_4_novice));
    let game = GameBuilder::new_default(room_8.roomid, round_nov_s2_5.roundid)
        .set_poolbracket_id(pb_nov_2.pool_bracket_id)
        .set_leftteamid(team_2_novice.teamid).set_rightteamid(team_3_novice.teamid)
        .set_quizmasterid(qm_5.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_2_novice, &team_3_novice));

    // Div: Decades — Session 1: full 4-team round-robin, 6 games / 3 rounds (2 games/round).
    // Fixed team alternates between rm9 and rm10 each round.
    // Round 1: (t1d,t4d)→rm9, (t2d,t3d)→rm10
    let game = GameBuilder::new_default(room_9.roomid, round_dec_s1_1.roundid)
        .set_poolbracket_id(pb_dec_1.pool_bracket_id)
        .set_leftteamid(team_1_decades.teamid)
        .set_rightteamid(team_4_decades.teamid)
        .set_quizmasterid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    game_event_specs.push((game.gid, &team_1_decades, &team_4_decades));
    let game = GameBuilder::new_default(room_10.roomid, round_dec_s1_1.roundid)
        .set_poolbracket_id(pb_dec_1.pool_bracket_id)
        .set_leftteamid(team_2_decades.teamid)
        .set_rightteamid(team_3_decades.teamid)
        .set_quizmasterid(qm_7.id)
        .set_contentjudgeid(Some(tour_owner.id))
        .build_and_insert(db)
        .unwrap();
    game_event_specs.push((game.gid, &team_2_decades, &team_3_decades));
    // Round 2: (t1d,t3d)→rm10, (t4d,t2d)→rm9
    let game = GameBuilder::new_default(room_10.roomid, round_dec_s1_2.roundid)
        .set_poolbracket_id(pb_dec_1.pool_bracket_id)
        .set_leftteamid(team_1_decades.teamid)
        .set_rightteamid(team_3_decades.teamid)
        .set_quizmasterid(qm_7.id)
        .set_contentjudgeid(Some(tour_owner.id))
        .build_and_insert(db)
        .unwrap();
    game_event_specs.push((game.gid, &team_1_decades, &team_3_decades));
    let game = GameBuilder::new_default(room_9.roomid, round_dec_s1_2.roundid)
        .set_poolbracket_id(pb_dec_1.pool_bracket_id)
        .set_leftteamid(team_4_decades.teamid)
        .set_rightteamid(team_2_decades.teamid)
        .set_quizmasterid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    game_event_specs.push((game.gid, &team_4_decades, &team_2_decades));
    // Round 3: (t1d,t2d)→rm9, (t3d,t4d)→rm10
    let game = GameBuilder::new_default(room_9.roomid, round_dec_s1_3.roundid)
        .set_poolbracket_id(pb_dec_1.pool_bracket_id)
        .set_leftteamid(team_1_decades.teamid)
        .set_rightteamid(team_2_decades.teamid)
        .set_quizmasterid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    game_event_specs.push((game.gid, &team_1_decades, &team_2_decades));
    let game = GameBuilder::new_default(room_10.roomid, round_dec_s1_3.roundid)
        .set_poolbracket_id(pb_dec_1.pool_bracket_id)
        .set_leftteamid(team_3_decades.teamid)
        .set_rightteamid(team_4_decades.teamid)
        .set_quizmasterid(qm_7.id)
        .set_contentjudgeid(Some(tour_owner.id))
        .build_and_insert(db)
        .unwrap();
    game_event_specs.push((game.gid, &team_3_decades, &team_4_decades));

    // Div: Decades — Session 2: full 4-team round-robin again, 6 games / 3 rounds. Rooms 9-10.
    // Round 1: (t1d,t4d)→rm9, (t2d,t3d)→rm10
    let game = GameBuilder::new_default(room_9.roomid, round_dec_s2_1.roundid)
        .set_poolbracket_id(pb_dec_2.pool_bracket_id)
        .set_leftteamid(team_1_decades.teamid)
        .set_rightteamid(team_4_decades.teamid)
        .set_quizmasterid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    game_event_specs.push((game.gid, &team_1_decades, &team_4_decades));
    let game = GameBuilder::new_default(room_10.roomid, round_dec_s2_1.roundid)
        .set_poolbracket_id(pb_dec_2.pool_bracket_id)
        .set_leftteamid(team_2_decades.teamid)
        .set_rightteamid(team_3_decades.teamid)
        .set_quizmasterid(qm_7.id)
        .set_contentjudgeid(Some(tour_owner.id))
        .build_and_insert(db)
        .unwrap();
    game_event_specs.push((game.gid, &team_2_decades, &team_3_decades));
    // Round 2: (t1d,t3d)→rm10, (t4d,t2d)→rm9
    let game = GameBuilder::new_default(room_10.roomid, round_dec_s2_2.roundid)
        .set_poolbracket_id(pb_dec_2.pool_bracket_id)
        .set_leftteamid(team_1_decades.teamid)
        .set_rightteamid(team_3_decades.teamid)
        .set_quizmasterid(qm_7.id)
        .set_contentjudgeid(Some(tour_owner.id))
        .build_and_insert(db)
        .unwrap();
    game_event_specs.push((game.gid, &team_1_decades, &team_3_decades));
    let game = GameBuilder::new_default(room_9.roomid, round_dec_s2_2.roundid)
        .set_poolbracket_id(pb_dec_2.pool_bracket_id)
        .set_leftteamid(team_4_decades.teamid)
        .set_rightteamid(team_2_decades.teamid)
        .set_quizmasterid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    game_event_specs.push((game.gid, &team_4_decades, &team_2_decades));
    // Round 3: (t1d,t2d)→rm9, (t3d,t4d)→rm10
    let game = GameBuilder::new_default(room_9.roomid, round_dec_s2_3.roundid)
        .set_poolbracket_id(pb_dec_2.pool_bracket_id)
        .set_leftteamid(team_1_decades.teamid)
        .set_rightteamid(team_2_decades.teamid)
        .set_quizmasterid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    game_event_specs.push((game.gid, &team_1_decades, &team_2_decades));
    let game = GameBuilder::new_default(room_10.roomid, round_dec_s2_3.roundid)
        .set_poolbracket_id(pb_dec_2.pool_bracket_id)
        .set_leftteamid(team_3_decades.teamid)
        .set_rightteamid(team_4_decades.teamid)
        .set_quizmasterid(qm_7.id)
        .set_contentjudgeid(Some(tour_owner.id))
        .build_and_insert(db)
        .unwrap();
    game_event_specs.push((game.gid, &team_3_decades, &team_4_decades));

    // Every game joined its division's statsgroup as it was created (see `game::create`), since
    // the statsgroups above exist before any game does.

    // Seed the game-event stream for every game in one place, and time just this portion — it is by
    // far the most expensive part of the seed, so it is gated behind `include_gameevents` as a unit.
    if !include_gameevents {
        return;
    }

    let start_time_for_game_events = Utc::now();
    println!("Starting DB Data Population for Game Events ({} games)", game_event_specs.len());
    for &(gid, left_team, right_team) in &game_event_specs {
        seed_game_events(db, gid, left_team, right_team);
    }
    let end_time_for_game_events = Utc::now();
    let duration_for_game_events = end_time_for_game_events.naive_utc() - start_time_for_game_events.naive_utc();
    println!("Done. DB Game Events Population Time Duration: {}\n", duration_for_game_events);
}

pub fn create_tournament_applicants(db: &mut database::Connection) {

    let member_role             = crate::models::role::read_by_name(db, AppRole::Member.as_str()).unwrap();
    let tournament_manager_role = crate::models::role::read_by_name(db, AppRole::TournamentManager.as_str()).unwrap();

    // Applicant 1 — status: pending ("Applied")
    let applicant_1 = UserBuilder::new("Alex")
        .set_lname("Torres (Demo)")
        .set_username("alex.torres")
        .set_hash_password(&default_password())
        .set_email("alex.torres@fakeemail.com")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();

    UsersRolesBuilder::new(applicant_1.id)
        .assign(member_role.id)
        .build_and_insert(db)
        .unwrap();

    CreateTournamentApplicantBuilder::new(applicant_1.id, "pending", applicant_1.id)
        .set_request_context(Some("I would like to host a regional tournament for my area.".to_string()))
        .build_and_insert(db)
        .unwrap();

    // Applicant 2 — status: declined
    let applicant_2 = UserBuilder::new("Jordan")
        .set_lname("Blake (Demo)")
        .set_username("jordan.blake")
        .set_hash_password(&default_password())
        .set_email("jordan.blake@fakeemail.com")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();

    UsersRolesBuilder::new(applicant_2.id)
        .assign(member_role.id)
        .build_and_insert(db)
        .unwrap();

    CreateTournamentApplicantBuilder::new(applicant_2.id, "declined", applicant_2.id)
        .set_request_context(Some("Requesting to run a small invitational tournament.".to_string()))
        .build_and_insert(db)
        .unwrap();

    // Applicant 3 — status: approved; user gets tournament_owner + tournament_manager roles
    let applicant_3 = UserBuilder::new("Casey")
        .set_lname("Morgan (Demo)")
        .set_username("casey.morgan")
        .set_hash_password(&default_password())
        .set_email("casey.morgan@fakeemail.com")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();

    UsersRolesBuilder::new(applicant_3.id)
        .assign(member_role.id)
        .assign(tournament_manager_role.id)
        .build_and_insert(db)
        .unwrap();

    CreateTournamentApplicantBuilder::new(applicant_3.id, "approved", applicant_3.id)
        .set_request_context(Some("Looking to organize a national qualifier event for our division.".to_string()))
        .build_and_insert(db)
        .unwrap();
}

/// Creates the single team group that a pool/bracket owns (they are one-to-one) and adds each of
/// the given teams to it, so the teams show up as "placed" in that pool/bracket.
fn place_teams_in_pool(db: &mut database::Connection, pool_bracket_id: Uuid, creator: Uuid, team_ids: &[Uuid]) {
    // The bracket owns the 1-to-1 FK now; resolve_or_create makes the team group and attaches it.
    let team_group = crate::models::teamgroup::resolve_or_create_for_pool_bracket(db, pool_bracket_id, creator)
        .unwrap();
    for &teamid in team_ids {
        TeamTeamgroupBuilder::new(teamid, team_group.team_group_id)
            .set_creator_userid(creator)
            .build_and_insert(db)
            .unwrap();
    }
}

/// Seeds a plausible stream of dummy game events for a single (two-team) game, following the
/// QuizMachine event-sequence patterns seen in exported data:
///   - a Question 1 initialization block: RM (room/rules), QT (quiz type), then for each team a
///     TN (team name), one QN per real quizzer (seated in order), an SC (captain) and an SS
///     (co-captain);
///   - a run of toss-up questions, each of which is either a TC (toss-up correct), a TE (toss-up
///     error) followed by the other team's bonus attempt (BC/BE), or an NJ (no jump).
///
/// Quizzer names come from the actual Users associated with each team (quizzer_one_id..six_id).
/// Each successive event is stamped one minute after the previous. The generated data is
/// deterministic and reproducible: game-to-game variance comes from a PRNG seeded by the game id.
fn seed_game_events(
    db: &mut database::Connection,
    gid: Uuid,
    left_team: &Team,
    right_team: &Team,
) {
    // Real quizzer names for each team, pulled from the associated User records.
    let left_quizzers = team_quizzer_names(db, left_team);
    let right_quizzers = team_quizzer_names(db, right_team);
    // A game needs at least one quizzer per team to generate a valid event stream.
    if left_quizzers.is_empty() || right_quizzers.is_empty() {
        return;
    }

    // Each successive game event is stamped one minute after the previous.
    let mut ts = Utc::now();

    // Tiny deterministic PRNG (LCG) seeded from the game id, so each game varies reproducibly.
    let raw = gid.as_u128();
    let mut rng: u64 = (raw as u64) ^ ((raw >> 64) as u64);
    let mut next = move |n: u64| -> u64 {
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (rng >> 33) % n.max(1)
    };

    let teams: [(i32, &str, &Vec<String>); 2] = [
        (0, left_team.name.as_str(), &left_quizzers),
        (1, right_team.name.as_str(), &right_quizzers),
    ];

    // ── Question 1: initialization block ──
    let mut ev = 0i32;
    insert_game_event(db, gid, 1, ev, "Tournament", 0, 0, GameEventCode::RM, ts); ev += 1; ts = ts + Duration::minutes(1);
    insert_game_event(db, gid, 1, ev, "N", 0, 0, GameEventCode::QT, ts); ev += 1; ts = ts + Duration::minutes(1);
    for (team_num, team_name, quizzers) in teams {
        insert_game_event(db, gid, 1, ev, team_name, team_num, team_num, GameEventCode::TN, ts); ev += 1; ts = ts + Duration::minutes(1);
        for (seat, quizzer_name) in quizzers.iter().enumerate() {
            insert_game_event(db, gid, 1, ev, quizzer_name, team_num, seat as i32, GameEventCode::QN, ts); ev += 1; ts = ts + Duration::minutes(1);
        }
        // Captain in seat 0; co-captain in seat 1 when the team has a second quizzer.
        insert_game_event(db, gid, 1, ev, &quizzers[0], team_num, 0, GameEventCode::SC, ts); ev += 1; ts = ts + Duration::minutes(1);
        if quizzers.len() >= 2 {
            insert_game_event(db, gid, 1, ev, &quizzers[1], team_num, 1, GameEventCode::SS, ts); ev += 1; ts = ts + Duration::minutes(1);
        }
    }

    // ── Toss-up questions ──
    let total_questions: i32 = 20;
    for q in 1..=total_questions {
        // Question 1 continues after the init block above; later questions start at eventnum 0.
        let mut qev = if q == 1 { ev } else { 0 };

        let answering_team = next(2) as i32;
        let answering_quizzers = if answering_team == 0 { &left_quizzers } else { &right_quizzers };
        let seat = next(answering_quizzers.len() as u64) as i32;
        let quizzer_name = answering_quizzers[seat as usize].clone();

        // Every question is answered — a seeded game is a complete round, so no question is left
        // on a no-jump. Either the toss-up is taken, or it is missed and the other team attempts
        // the bonus; both outcomes resolve the question.
        match next(10) {
            0..=6 => {
                // Toss-up answered correctly.
                insert_game_event(db, gid, q, qev, &quizzer_name, answering_team, seat, GameEventCode::TC, ts); qev += 1; ts = ts + Duration::minutes(1);
            }
            _ => {
                // Toss-up error, then the other team attempts the bonus.
                insert_game_event(db, gid, q, qev, &quizzer_name, answering_team, seat, GameEventCode::TE, ts); qev += 1; ts = ts + Duration::minutes(1);
                let other_team = 1 - answering_team;
                let other_quizzers = if other_team == 0 { &left_quizzers } else { &right_quizzers };
                let other_seat = next(other_quizzers.len() as u64) as i32;
                let other_name = other_quizzers[other_seat as usize].clone();
                let bonus = if next(2) == 0 { GameEventCode::BC } else { GameEventCode::BE };
                insert_game_event(db, gid, q, qev, &other_name, other_team, other_seat, bonus, ts); qev += 1; ts = ts + Duration::minutes(1);
            }
        }
        let _ = qev;
    }

    // ── Overtime ──
    //
    // A quiz may not end level, so if regulation finished tied, play overtime until one team is
    // ahead. Rather than re-implement the scoring rules here (and risk drifting from them), ask
    // the real calculator what the standing is and react to its answer.
    //
    // An overtime toss-up is a TC on a question past regulation. The calculator only accepts one
    // while a tie exists, and scores it by awarding the points, re-ranking, then taking the points
    // back — so the displayed score stays level while the rank records who won.
    let mut overtime_question = total_questions + 1;
    while overtime_question <= total_questions + MAX_OVERTIME_QUESTIONS {
        let tied_teams = match tied_team_indices(db, gid) {
            Some(teams) if teams.len() > 1 => teams,
            // Not tied (or the stream can't be scored) — nothing to break.
            _ => break,
        };

        // Pick the overtime winner deterministically from among the tied teams.
        let winner = tied_teams[next(tied_teams.len() as u64) as usize];
        let winning_quizzers = if winner == 0 { &left_quizzers } else { &right_quizzers };
        let seat = next(winning_quizzers.len() as u64) as i32;
        let quizzer_name = winning_quizzers[seat as usize].clone();

        insert_game_event(
            db, gid, overtime_question, 0, &quizzer_name, winner, seat, GameEventCode::TC, ts,
        );
        ts = ts + Duration::minutes(1);
        overtime_question += 1;
    }
}

/// How many overtime questions the seed will play before giving up on separating the teams. A
/// single correct toss-up settles it, so this is only a guard against looping forever.
const MAX_OVERTIME_QUESTIONS: i32 = 5;

/// The team indices currently sharing the top rank of a game, via the real score calculator.
/// None when the event stream cannot be scored.
fn tied_team_indices(db: &mut database::Connection, gid: Uuid) -> Option<Vec<i32>> {
    let pagination = crate::models::common::PaginationParams {
        page: 0,
        page_size: crate::models::common::PaginationParams::MAX_PAGE_SIZE as i64,
    };
    let events = crate::models::gameevent::read_all_gameevents_of_game(db, gid, &pagination).ok()?;
    let results = crate::models::gameevent::calculate_team_results_for_game(gid, events).ok()?;
    let best = results.iter().map(|r| r.rank).min()?;
    Some(results.iter().filter(|r| r.rank == best).map(|r| r.team).collect())
}

/// Pairings for a four-team double round-robin, as (left, right) index pairs per round.
///
/// Rounds 1-3 are the circle-method single round-robin; rounds 4-6 repeat them with the sides
/// swapped, so each pair meets exactly twice and each team plays once per round.
const FOUR_TEAM_DOUBLE_ROUND_ROBIN: [[(usize, usize); 2]; 6] = [
    [(0, 3), (1, 2)],
    [(0, 2), (3, 1)],
    [(0, 1), (2, 3)],
    [(3, 0), (2, 1)],
    [(2, 0), (1, 3)],
    [(1, 0), (3, 2)],
];

/// Returns the distinct display names of the Users seated on a team (quizzer_one_id..six_id),
/// in slot order, skipping empty slots and any that can't be read.
fn team_quizzer_names(db: &mut database::Connection, team: &Team) -> Vec<String> {
    let ids = [
        team.quizzer_one_id, team.quizzer_two_id, team.quizzer_three_id,
        team.quizzer_four_id, team.quizzer_five_id, team.quizzer_six_id,
    ];
    let mut names: Vec<String> = Vec::new();
    for id in ids.into_iter().flatten() {
        if let Ok(user) = crate::models::user::read(db, id) {
            let name = format!("{} {}", user.fname, user.lname).trim().to_string();
            // Skip blanks and de-duplicate so each seat maps to a distinct quizzer name.
            if !name.is_empty() && !names.contains(&name) {
                names.push(name);
            }
        }
    }
    names
}

/// Inserts a single game event; a thin wrapper over GameEventBuilder used by seed_game_events.
fn insert_game_event(
    db: &mut database::Connection,
    gid: Uuid,
    question: i32,
    eventnum: i32,
    name: &str,
    team: i32,
    quizzer: i32,
    event: GameEventCode,
    ts: DateTime<Utc>,
) {
    GameEventBuilder::new_default(gid)
        .set_question(Some(question))
        .set_eventnum(Some(eventnum))
        .set_name(Some(name.to_string()))
        .set_team(Some(team))
        .set_quizzer(Some(quizzer))
        .set_event(Some(event))
        .set_clientts(Some(ts))
        .set_source(Some("seed data".to_string()))
        .build_and_insert(db)
        .unwrap();
}
