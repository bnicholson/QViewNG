use crate::{database::{self, seed_data::system_default_data::default_password}, models::{computer::ComputerBuilder, create_tournament_applicant::CreateTournamentApplicantBuilder, division::DivisionBuilder, division_session::DivisionSessionBuilder, pool_bracket::PoolBracketBuilder, equipmentregistration::{EquipmentRegistrationBuilder, EquipmentRegistrationStatus}, extensioncord::ExtensionCordBuilder, game::GameBuilder, interfacebox::InterfaceBoxBuilder, jumppad::JumpPadBuilder, microphonerecorder::MicrophoneRecorderBuilder, monitor::MonitorBuilder, powerstrip::PowerStripBuilder, projector::ProjectorBuilder, role::AppRole, room::RoomBuilder, roster::RosterBuilder, roster_coach::RosterCoachBuilder, roster_quizzer::RosterQuizzerBuilder, round::RoundBuilder, statsgroup::StatsGroupBuilder, game_statsgroup::GameStatsGroupBuilder, team::{Team, TeamBuilder}, teamgroup::TeamGroupBuilder, team_teamgroup::TeamTeamgroupBuilder, tournament::TournamentBuilder, tournament_admin::TournamentAdminBuilder, tournamentgroup::TournamentGroupBuilder, tournamentgroup_tournament::TournamentGroupTournamentBuilder, user::UserBuilder, users_roles::UsersRolesBuilder}};
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
        .set_lname("Manager")
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
        .set_lname("Member")
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
        .set_lname("Owner")
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
        .set_lname("Admin")
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
        .set_lname("Castillo")
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
        .set_lname("Voss")
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

    // One division-scoped statsgroup per division (games are linked to these below).
    let sg_experienced = StatsGroupBuilder::new_default(&division_experienced.dname, tour.tid)
        .set_division_id(Some(division_experienced.did))
        .build_and_insert(db)
        .unwrap();
    let sg_novice = StatsGroupBuilder::new_default(&division_novice.dname, tour.tid)
        .set_division_id(Some(division_novice.did))
        .build_and_insert(db)
        .unwrap();
    let sg_decades = StatsGroupBuilder::new_default(&division_decades.dname, tour.tid)
        .set_division_id(Some(division_decades.did))
        .build_and_insert(db)
        .unwrap();

    // Division sessions and pools/brackets are created later, inside the `include_scheduling` block
    // (below), together with the team placements and games that depend on them — so an unscheduled
    // tournament stops at registered teams + gear.

    // Quizmasters — one per room, each unique across all rooms
    let qm_1 = UserBuilder::new("Jordan")
        .set_lname("Avery")
        .set_username("javery_qm")
        .set_email("javery@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let qm_2 = UserBuilder::new("Riley")
        .set_lname("Blake")
        .set_username("rblake_qm")
        .set_email("rblake@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let qm_3 = UserBuilder::new("Morgan")
        .set_lname("Casey")
        .set_username("mcasey_qm")
        .set_email("mcasey@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let qm_4 = UserBuilder::new("Quinn")
        .set_lname("Drew")
        .set_username("qdrew_qm")
        .set_email("qdrew@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let qm_5 = UserBuilder::new("Sage")
        .set_lname("Ellis")
        .set_username("sellis_qm")
        .set_email("sellis@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let qm_6 = UserBuilder::new("Avery")
        .set_lname("Flynn")
        .set_username("aflynn_qm")
        .set_email("aflynn@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let qm_7 = UserBuilder::new("Blake")
        .set_lname("Grant")
        .set_username("bgrant_qm")
        .set_email("bgrant@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();

    // Content Judges — one per room, each unique across all rooms
    let cj_1 = UserBuilder::new("Dana")
        .set_lname("Harper")
        .set_username("dharper_cj")
        .set_email("dharper@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let cj_2 = UserBuilder::new("Emery")
        .set_lname("Ingram")
        .set_username("eingram_cj")
        .set_email("eingram@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let cj_3 = UserBuilder::new("Finley")
        .set_lname("Jensen")
        .set_username("fjensen_cj")
        .set_email("fjensen@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let cj_4 = UserBuilder::new("Gray")
        .set_lname("Knox")
        .set_username("gknox_cj")
        .set_email("gknox@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let cj_5 = UserBuilder::new("Hayden")
        .set_lname("Lane")
        .set_username("hlane_cj")
        .set_email("hlane@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let cj_6 = UserBuilder::new("Indigo")
        .set_lname("Marsh")
        .set_username("imarsh_cj")
        .set_email("imarsh@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let cj_7 = UserBuilder::new("Jamie")
        .set_lname("Nash")
        .set_username("jnash_cj")
        .set_email("jnash@fakeemail.com")
        .set_hash_password(&default_password())
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();

    let room_1 = RoomBuilder::new_default("Room 1", tour.tid)
        .set_last_modified_user(tour_owner.id)
        .set_comments("".to_string())
        .set_clientkey(Some("bankdiu".to_string()))
        .set_quizmaster_id(Some(qm_1.id))
        .set_contentjudge_id(Some(cj_1.id))
        .build_and_insert(db)
        .unwrap();
    let room_2 = RoomBuilder::new_default("Room 2", tour.tid)
        .set_last_modified_user(tour_owner.id)
        .set_comments("".to_string())
        .set_clientkey(Some("bbhsth4".to_string()))
        .set_quizmaster_id(Some(qm_2.id))
        .set_contentjudge_id(Some(cj_2.id))
        .build_and_insert(db)
        .unwrap();
    let room_3 = RoomBuilder::new_default("Room 3", tour.tid)
        .set_last_modified_user(tour_owner.id)
        .set_comments("".to_string())
        .set_clientkey(Some("16587397".to_string()))
        .set_quizmaster_id(Some(qm_3.id))
        .set_contentjudge_id(Some(cj_3.id))
        .build_and_insert(db)
        .unwrap();
    let room_4 = RoomBuilder::new_default("Room 4", tour.tid)
        .set_last_modified_user(tour_owner.id)
        .set_comments("".to_string())
        .set_clientkey(Some("aplyhen".to_string()))
        .set_quizmaster_id(Some(qm_4.id))
        .set_contentjudge_id(Some(cj_4.id))
        .build_and_insert(db)
        .unwrap();
    let room_5 = RoomBuilder::new_default("Room 5", tour.tid)
        .set_last_modified_user(tour_owner.id)
        .set_comments("".to_string())
        .set_clientkey(Some("llpjhin".to_string()))
        .set_quizmaster_id(Some(qm_5.id))
        .set_contentjudge_id(Some(cj_5.id))
        .build_and_insert(db)
        .unwrap();
    let room_6 = RoomBuilder::new_default("Room 6", tour.tid)
        .set_last_modified_user(tour_owner.id)
        .set_comments("".to_string())
        .set_clientkey(Some("qwx7bfyh".to_string()))
        .set_quizmaster_id(Some(qm_6.id))
        .set_contentjudge_id(Some(cj_6.id))
        .build_and_insert(db)
        .unwrap();
    let room_7 = RoomBuilder::new_default("Room 7", tour.tid)
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
        .set_last_modified_user(tour_owner.id)
        .set_comments("".to_string())
        .set_clientkey(Some("rm8xk2po".to_string()))
        .build_and_insert(db)
        .unwrap();
    let room_9 = RoomBuilder::new_default("Room 9", tour.tid)
        .set_last_modified_user(tour_owner.id)
        .set_comments("".to_string())
        .set_clientkey(Some("rm9zt5qa".to_string()))
        .build_and_insert(db)
        .unwrap();
    let room_10 = RoomBuilder::new_default("Room 10", tour.tid)
        .set_last_modified_user(tour_owner.id)
        .set_comments("".to_string())
        .set_clientkey(Some("rm10wv3b".to_string()))
        .build_and_insert(db)
        .unwrap();

    // Div: Experienced
    let coach_exp_1 = UserBuilder::new("Coachish")
        .set_lname("Coach")
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
        .set_lname("Park")
        .set_email("apark@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_1_2 = UserBuilder::new("Zoe")
        .set_lname("Nakamura")
        .set_email("znakamura@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_1_3 = UserBuilder::new("Felix")
        .set_lname("Brennan")
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
        .set_lname("Ellison")
        .set_email("hellison@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_r2_2 = UserBuilder::new("Rowan")
        .set_lname("Caldwell")
        .set_email("rcaldwell@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_r2_3 = UserBuilder::new("Nadia")
        .set_lname("Ostrowski")
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
        .set_lname("Osei")
        .set_email("mosei@fakeemail.com")
        .set_username("mosei")
        .set_hash_password("Marc0s!Ose")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_exp_2_1 = UserBuilder::new("Simone")
        .set_lname("Tremblay")
        .set_email("stremblay@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_2_2 = UserBuilder::new("Kwame")
        .set_lname("Asante")
        .set_email("kasante@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_2_3 = UserBuilder::new("Nora")
        .set_lname("Lindqvist")
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
        .set_lname("Nair")
        .set_email("pnair@fakeemail.com")
        .set_username("pnair")
        .set_hash_password("Priy@N4ir")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_exp_3_1 = UserBuilder::new("Dante")
        .set_lname("Moretti")
        .set_email("dmoretti@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_3_2 = UserBuilder::new("Clara")
        .set_lname("Hoffmann")
        .set_email("choffmann@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_3_3 = UserBuilder::new("Ravi")
        .set_lname("Sharma")
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
        .set_lname("Calloway")
        .set_email("dcalloway@fakeemail.com")
        .set_username("dcalloway")
        .set_hash_password("Derek#C4ll")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_exp_4_1 = UserBuilder::new("Yara")
        .set_lname("Hassan")
        .set_email("yhassan@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_4_2 = UserBuilder::new("Owen")
        .set_lname("Fitzgerald")
        .set_email("ofitzgerald@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_4_3 = UserBuilder::new("Mei")
        .set_lname("Lin")
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
        .set_lname("Tanaka")
        .set_email("ytanaka@fakeemail.com")
        .set_username("ytanaka")
        .set_hash_password("Yuk1!Tank")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_exp_5_1 = UserBuilder::new("Ezra")
        .set_lname("Goldberg")
        .set_email("egoldberg@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_5_2 = UserBuilder::new("Amira")
        .set_lname("Seif")
        .set_email("aseif@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_5_3 = UserBuilder::new("Luke")
        .set_lname("Petrov")
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
        .set_lname("Flores")
        .set_email("bflores@fakeemail.com")
        .set_username("bflores")
        .set_hash_password("Bri@Fl0re")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_exp_6_1 = UserBuilder::new("Isla")
        .set_lname("Mackenzie")
        .set_email("imackenzie@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_6_2 = UserBuilder::new("Tomas")
        .set_lname("Vega")
        .set_email("tvega@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_6_3 = UserBuilder::new("Hana")
        .set_lname("Iwata")
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
        .set_lname("Volkov")
        .set_email("nvolkov@fakeemail.com")
        .set_username("nvolkov")
        .set_hash_password("Nad!aV0lk")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_exp_7_1 = UserBuilder::new("Priya")
        .set_lname("Menon")
        .set_email("pmenon@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_7_2 = UserBuilder::new("Diego")
        .set_lname("Ramos")
        .set_email("dramos@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_7_3 = UserBuilder::new("Anya")
        .set_lname("Sokolova")
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
        .set_lname("Karimi")
        .set_email("rkarimi@fakeemail.com")
        .set_username("rkarimi")
        .set_hash_password("Rash!dK4r")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_exp_8_1 = UserBuilder::new("Omar")
        .set_lname("Farouk")
        .set_email("ofarouk@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_8_2 = UserBuilder::new("Lucia")
        .set_lname("Mendez")
        .set_email("lmendez@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_8_3 = UserBuilder::new("Tariq")
        .set_lname("Aziz")
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
        .set_lname("Costa")
        .set_email("ecosta@fakeemail.com")
        .set_username("ecosta")
        .set_hash_password("Elen@C0st")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_exp_9_1 = UserBuilder::new("Freya")
        .set_lname("Nilsson")
        .set_email("fnilsson@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_9_2 = UserBuilder::new("Hugo")
        .set_lname("Bernard")
        .set_email("hbernard@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_9_3 = UserBuilder::new("Meera")
        .set_lname("Iyer")
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
        .set_lname("Johnson")
        .set_email("mjohnson@fakeemail.com")
        .set_username("mjohnson")
        .set_hash_password("Mal!kJ0hn")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_exp_10_1 = UserBuilder::new("Sven")
        .set_lname("Larsen")
        .set_email("slarsen@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_10_2 = UserBuilder::new("Rania")
        .set_lname("Haddad")
        .set_email("rhaddad@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_10_3 = UserBuilder::new("Felix")
        .set_lname("Braun")
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
        .set_lname("Rossi")
        .set_email("srossi@fakeemail.com")
        .set_username("srossi")
        .set_hash_password("Sof!aR0ss")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_exp_11_1 = UserBuilder::new("Ingrid")
        .set_lname("Holm")
        .set_email("iholm@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_11_2 = UserBuilder::new("Carlos")
        .set_lname("Vargas")
        .set_email("cvargas@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_11_3 = UserBuilder::new("Yasmin")
        .set_lname("Ali")
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
        .set_lname("Watanabe")
        .set_email("kwatanabe@fakeemail.com")
        .set_username("kwatanabe")
        .set_hash_password("Kenj!W4ta")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_exp_12_1 = UserBuilder::new("Bjorn")
        .set_lname("Eriksson")
        .set_email("beriksson@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_12_2 = UserBuilder::new("Leila")
        .set_lname("Nasser")
        .set_email("lnasser@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_exp_12_3 = UserBuilder::new("Andre")
        .set_lname("Silva")
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

    // Rounds are created inside the scheduling block below (they belong to division sessions).

    // Div: Novice
    let coach_nov_1 = UserBuilder::new("Samuel")
        .set_lname("Ebert")
        .set_email("sebert@fakeemail.com")
        .set_username("sebert")
        .set_hash_password("Sam#Eb3rt")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_nov_1_1 = UserBuilder::new("Elijah")
        .set_lname("Brooks")
        .set_email("ebrooks@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_nov_1_2 = UserBuilder::new("Sofia")
        .set_lname("Reyes")
        .set_email("sreyes@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_nov_1_3 = UserBuilder::new("Liam")
        .set_lname("Okafor")
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
        .set_lname("Rashid")
        .set_email("frashid@fakeemail.com")
        .set_username("frashid")
        .set_hash_password("Fat!Ra5hid")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_nov_2_1 = UserBuilder::new("Mia")
        .set_lname("Johansson")
        .set_email("mjohansson@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_nov_2_2 = UserBuilder::new("Caleb")
        .set_lname("Patel")
        .set_email("cpatel@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_nov_2_3 = UserBuilder::new("Aria")
        .set_lname("Novak")
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
        .set_lname("Walsh")
        .set_email("cwalsh@fakeemail.com")
        .set_username("cwalsh")
        .set_hash_password("Con@W4lsh")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_nov_3_1 = UserBuilder::new("Noah")
        .set_lname("Ferreira")
        .set_email("nferreira@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_nov_3_2 = UserBuilder::new("Stella")
        .set_lname("Kim")
        .set_email("skim@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_nov_3_3 = UserBuilder::new("James")
        .set_lname("Mbeki")
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
        .set_lname("Eze")
        .set_email("neze@fakeemail.com")
        .set_username("neze")
        .set_hash_password("Ngoz!3Eze")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_nov_4_1 = UserBuilder::new("Leah")
        .set_lname("Christensen")
        .set_email("lchristensen@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_nov_4_2 = UserBuilder::new("Omar")
        .set_lname("Saleh")
        .set_email("osaleh@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_nov_4_3 = UserBuilder::new("Ruby")
        .set_lname("Nguyen")
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

    // (Novice rounds are created inside the scheduling block below.)

    // Div: Decades
    let coach_dec_1 = UserBuilder::new("Pavel")
        .set_lname("Sorokin")
        .set_email("psorokin@fakeemail.com")
        .set_username("psorokin")
        .set_hash_password("Pav@S0rok")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_dec_1_1 = UserBuilder::new("Finn")
        .set_lname("Gallagher")
        .set_email("fgallagher@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_dec_1_2 = UserBuilder::new("Zara")
        .set_lname("Ahmed")
        .set_email("zahmed@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_dec_1_3 = UserBuilder::new("Cole")
        .set_lname("Marchetti")
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
        .set_lname("Diallo")
        .set_email("adiallo@fakeemail.com")
        .set_username("adiallo")
        .set_hash_password("Amar#Di4l")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_dec_2_1 = UserBuilder::new("Ivy")
        .set_lname("Chen")
        .set_email("ichen@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_dec_2_2 = UserBuilder::new("Declan")
        .set_lname("Murphy")
        .set_email("dmurphy@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_dec_2_3 = UserBuilder::new("Layla")
        .set_lname("Espinoza")
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
        .set_lname("Reinhardt")
        .set_email("treinhardt@fakeemail.com")
        .set_username("treinhardt")
        .set_hash_password("Tobi!R3in")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_dec_3_1 = UserBuilder::new("Asher")
        .set_lname("Friedman")
        .set_email("afriedman@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_dec_3_2 = UserBuilder::new("Cleo")
        .set_lname("Papadopoulos")
        .set_email("cpapadopoulos@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_dec_3_3 = UserBuilder::new("Theo")
        .set_lname("Nkosi")
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
        .set_lname("Hollis")
        .set_email("mhollis@fakeemail.com")
        .set_username("mhollis")
        .set_hash_password("Mir@H0lli")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
    let q_dec_4_1 = UserBuilder::new("Vera")
        .set_lname("Kuznetsova")
        .set_email("vkuznetsova@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_dec_4_2 = UserBuilder::new("Miles")
        .set_lname("Oduya")
        .set_email("moduya@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let q_dec_4_3 = UserBuilder::new("Sasha")
        .set_lname("Petersen")
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
    // Everything below this point — division sessions, pools/brackets, team placements, games, and
    // the game events nested under `include_gameevents` — is the tournament's schedule. When
    // `include_scheduling` is false the seed stops here: a tournament with registered teams and gear
    // but no schedule yet, ready for the manager/admins to define one. Because games are only created
    // past this guard, no game events are ever added without a schedule even if `include_gameevents`
    // is true (there would be no games to attach them to).
    if !include_scheduling {
        return;
    }

    // Each division runs two sessions ("Session 1" then "Session 2"). Session 1 for every division
    // happens concurrently, so those games use disjoint rooms (Experienced 1-2, Novice 4-5,
    // Decades 6-7); Session 2 happens afterward and reuses the same rooms.
    //
    // Experienced's Session 1 is split into two pools (its teams divided evenly across them — see the
    // team groups seeded below); every other session is a single pool holding all of that division's
    // teams. Games must reference a real pool bracket — there is no auto-default.
    let session_exp_1 = DivisionSessionBuilder::new(division_experienced.did)
        .set_name("Session 1")
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    let pb_exp_1_a = PoolBracketBuilder::new(division_experienced.did)
        .set_name("Pool A")
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    let pb_exp_1_b = PoolBracketBuilder::new(division_experienced.did)
        .set_name("Pool B")
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    // Experienced runs a single session with two pools (six teams each); no Session 2.

    let session_nov_1 = DivisionSessionBuilder::new(division_novice.did)
        .set_name("Session 1")
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    let pb_nov_1 = PoolBracketBuilder::new(division_novice.did)
        .set_name("Pool A")
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    let session_nov_2 = DivisionSessionBuilder::new(division_novice.did)
        .set_name("Session 2")
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    // Pool brackets are division-scoped now, so the division's pools need distinct names.
    let pb_nov_2 = PoolBracketBuilder::new(division_novice.did)
        .set_name("Pool B")
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();

    let session_dec_1 = DivisionSessionBuilder::new(division_decades.did)
        .set_name("Session 1")
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    let pb_dec_1 = PoolBracketBuilder::new(division_decades.did)
        .set_name("Pool A")
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    let session_dec_2 = DivisionSessionBuilder::new(division_decades.did)
        .set_name("Session 2")
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    let pb_dec_2 = PoolBracketBuilder::new(division_decades.did)
        .set_name("Pool B")
        .set_creator_userid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();

    // Rounds belong to (time-bound) division sessions. Experienced's single session runs 5 rounds
    // (12:00-14:00); Novice and Decades each run 3 rounds per session (Session 1 12:00-13:00,
    // Session 2 13:30-14:30). Both Experienced pools share these same 5 rounds.
    let round_exp_s1_1 = RoundBuilder::new_default(session_exp_1.division_session_id)
        .set_last_modified_user(tour_owner.id).set_name("1")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 12,  0, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_exp_s1_2 = RoundBuilder::new_default(session_exp_1.division_session_id)
        .set_last_modified_user(tour_owner.id).set_name("2")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 12, 30, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_exp_s1_3 = RoundBuilder::new_default(session_exp_1.division_session_id)
        .set_last_modified_user(tour_owner.id).set_name("3")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 13,  0, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_exp_s1_4 = RoundBuilder::new_default(session_exp_1.division_session_id)
        .set_last_modified_user(tour_owner.id).set_name("4")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 13, 30, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_exp_s1_5 = RoundBuilder::new_default(session_exp_1.division_session_id)
        .set_last_modified_user(tour_owner.id).set_name("5")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 14,  0, 0).unwrap())
        .build_and_insert(db).unwrap();

    let round_nov_s1_1 = RoundBuilder::new_default(session_nov_1.division_session_id)
        .set_last_modified_user(tour_owner.id).set_name("1")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 12,  0, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_nov_s1_2 = RoundBuilder::new_default(session_nov_1.division_session_id)
        .set_last_modified_user(tour_owner.id).set_name("2")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 12, 30, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_nov_s1_3 = RoundBuilder::new_default(session_nov_1.division_session_id)
        .set_last_modified_user(tour_owner.id).set_name("3")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 13,  0, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_nov_s2_1 = RoundBuilder::new_default(session_nov_2.division_session_id)
        .set_last_modified_user(tour_owner.id).set_name("4")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 13, 30, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_nov_s2_2 = RoundBuilder::new_default(session_nov_2.division_session_id)
        .set_last_modified_user(tour_owner.id).set_name("5")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 14,  0, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_nov_s2_3 = RoundBuilder::new_default(session_nov_2.division_session_id)
        .set_last_modified_user(tour_owner.id).set_name("6")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 14, 30, 0).unwrap())
        .build_and_insert(db).unwrap();

    let round_dec_s1_1 = RoundBuilder::new_default(session_dec_1.division_session_id)
        .set_last_modified_user(tour_owner.id).set_name("1")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 12,  0, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_dec_s1_2 = RoundBuilder::new_default(session_dec_1.division_session_id)
        .set_last_modified_user(tour_owner.id).set_name("2")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 12, 30, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_dec_s1_3 = RoundBuilder::new_default(session_dec_1.division_session_id)
        .set_last_modified_user(tour_owner.id).set_name("3")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 13,  0, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_dec_s2_1 = RoundBuilder::new_default(session_dec_2.division_session_id)
        .set_last_modified_user(tour_owner.id).set_name("4")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 13, 30, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_dec_s2_2 = RoundBuilder::new_default(session_dec_2.division_session_id)
        .set_last_modified_user(tour_owner.id).set_name("5")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 14,  0, 0).unwrap())
        .build_and_insert(db).unwrap();
    let round_dec_s2_3 = RoundBuilder::new_default(session_dec_2.division_session_id)
        .set_last_modified_user(tour_owner.id).set_name("6")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 14, 30, 0).unwrap())
        .build_and_insert(db).unwrap();

    // Team groups: each pool/bracket owns exactly one team group holding its teams. Experienced's
    // 12 teams are split evenly across its two pools (1-6 in Pool A, 7-12 in Pool B); every other
    // pool holds all of its division's teams.
    place_teams_in_pool(db, pb_exp_1_a.pool_bracket_id, tour_owner.id, &[
        team_1_experienced.teamid, team_2_experienced.teamid, team_3_experienced.teamid,
        team_4_experienced.teamid, team_5_experienced.teamid, team_6_experienced.teamid,
    ]);
    place_teams_in_pool(db, pb_exp_1_b.pool_bracket_id, tour_owner.id, &[
        team_7_experienced.teamid, team_8_experienced.teamid, team_9_experienced.teamid,
        team_10_experienced.teamid, team_11_experienced.teamid, team_12_experienced.teamid,
    ]);
    for pb in [pb_nov_1.pool_bracket_id, pb_nov_2.pool_bracket_id] {
        place_teams_in_pool(db, pb, tour_owner.id, &[
            team_1_novice.teamid, team_2_novice.teamid, team_3_novice.teamid, team_4_novice.teamid,
        ]);
    }
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

    // Div: Experienced — Session 1, Pool A: full 6-team round-robin (teams 1-6), 15 games / 5 rounds
    // in Rooms 1-3 (qm per room: rm1→qm1, rm2→qm2, rm3→qm3). Runs concurrently with Pool B (Rooms 4-6).
    // Round 1: (1,6)→rm1, (2,5)→rm2, (3,4)→rm3
    let game = GameBuilder::new_default(room_1.roomid, round_exp_s1_1.roundid)
        .set_poolbracket_id(pb_exp_1_a.pool_bracket_id)
        .set_leftteamid(team_1_experienced.teamid).set_rightteamid(team_6_experienced.teamid)
        .set_quizmasterid(qm_1.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_1_experienced, &team_6_experienced));
    let game = GameBuilder::new_default(room_2.roomid, round_exp_s1_1.roundid)
        .set_poolbracket_id(pb_exp_1_a.pool_bracket_id)
        .set_leftteamid(team_2_experienced.teamid).set_rightteamid(team_5_experienced.teamid)
        .set_quizmasterid(qm_2.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_2_experienced, &team_5_experienced));
    let game = GameBuilder::new_default(room_3.roomid, round_exp_s1_1.roundid)
        .set_poolbracket_id(pb_exp_1_a.pool_bracket_id)
        .set_leftteamid(team_3_experienced.teamid).set_rightteamid(team_4_experienced.teamid)
        .set_quizmasterid(qm_3.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_3_experienced, &team_4_experienced));
    // Round 2: (1,5)→rm2, (6,4)→rm3, (2,3)→rm1
    let game = GameBuilder::new_default(room_2.roomid, round_exp_s1_2.roundid)
        .set_poolbracket_id(pb_exp_1_a.pool_bracket_id)
        .set_leftteamid(team_1_experienced.teamid).set_rightteamid(team_5_experienced.teamid)
        .set_quizmasterid(qm_2.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_1_experienced, &team_5_experienced));
    let game = GameBuilder::new_default(room_3.roomid, round_exp_s1_2.roundid)
        .set_poolbracket_id(pb_exp_1_a.pool_bracket_id)
        .set_leftteamid(team_6_experienced.teamid).set_rightteamid(team_4_experienced.teamid)
        .set_quizmasterid(qm_3.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_6_experienced, &team_4_experienced));
    let game = GameBuilder::new_default(room_1.roomid, round_exp_s1_2.roundid)
        .set_poolbracket_id(pb_exp_1_a.pool_bracket_id)
        .set_leftteamid(team_2_experienced.teamid).set_rightteamid(team_3_experienced.teamid)
        .set_quizmasterid(qm_1.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_2_experienced, &team_3_experienced));
    // Round 3: (1,4)→rm3, (5,3)→rm1, (6,2)→rm2
    let game = GameBuilder::new_default(room_3.roomid, round_exp_s1_3.roundid)
        .set_poolbracket_id(pb_exp_1_a.pool_bracket_id)
        .set_leftteamid(team_1_experienced.teamid).set_rightteamid(team_4_experienced.teamid)
        .set_quizmasterid(qm_3.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_1_experienced, &team_4_experienced));
    let game = GameBuilder::new_default(room_1.roomid, round_exp_s1_3.roundid)
        .set_poolbracket_id(pb_exp_1_a.pool_bracket_id)
        .set_leftteamid(team_5_experienced.teamid).set_rightteamid(team_3_experienced.teamid)
        .set_quizmasterid(qm_1.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_5_experienced, &team_3_experienced));
    let game = GameBuilder::new_default(room_2.roomid, round_exp_s1_3.roundid)
        .set_poolbracket_id(pb_exp_1_a.pool_bracket_id)
        .set_leftteamid(team_6_experienced.teamid).set_rightteamid(team_2_experienced.teamid)
        .set_quizmasterid(qm_2.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_6_experienced, &team_2_experienced));
    // Round 4: (1,3)→rm1, (4,2)→rm3, (5,6)→rm2
    let game = GameBuilder::new_default(room_1.roomid, round_exp_s1_4.roundid)
        .set_poolbracket_id(pb_exp_1_a.pool_bracket_id)
        .set_leftteamid(team_1_experienced.teamid).set_rightteamid(team_3_experienced.teamid)
        .set_quizmasterid(qm_1.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_1_experienced, &team_3_experienced));
    let game = GameBuilder::new_default(room_3.roomid, round_exp_s1_4.roundid)
        .set_poolbracket_id(pb_exp_1_a.pool_bracket_id)
        .set_leftteamid(team_4_experienced.teamid).set_rightteamid(team_2_experienced.teamid)
        .set_quizmasterid(qm_3.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_4_experienced, &team_2_experienced));
    let game = GameBuilder::new_default(room_2.roomid, round_exp_s1_4.roundid)
        .set_poolbracket_id(pb_exp_1_a.pool_bracket_id)
        .set_leftteamid(team_5_experienced.teamid).set_rightteamid(team_6_experienced.teamid)
        .set_quizmasterid(qm_2.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_5_experienced, &team_6_experienced));
    // Round 5: (1,2)→rm2, (3,6)→rm1, (4,5)→rm3
    let game = GameBuilder::new_default(room_2.roomid, round_exp_s1_5.roundid)
        .set_poolbracket_id(pb_exp_1_a.pool_bracket_id)
        .set_leftteamid(team_1_experienced.teamid).set_rightteamid(team_2_experienced.teamid)
        .set_quizmasterid(qm_2.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_1_experienced, &team_2_experienced));
    let game = GameBuilder::new_default(room_1.roomid, round_exp_s1_5.roundid)
        .set_poolbracket_id(pb_exp_1_a.pool_bracket_id)
        .set_leftteamid(team_3_experienced.teamid).set_rightteamid(team_6_experienced.teamid)
        .set_quizmasterid(qm_1.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_3_experienced, &team_6_experienced));
    let game = GameBuilder::new_default(room_3.roomid, round_exp_s1_5.roundid)
        .set_poolbracket_id(pb_exp_1_a.pool_bracket_id)
        .set_leftteamid(team_4_experienced.teamid).set_rightteamid(team_5_experienced.teamid)
        .set_quizmasterid(qm_3.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_4_experienced, &team_5_experienced));

    // Div: Experienced — Session 1, Pool B: full 6-team round-robin (teams 7-12), 15 games / 5 rounds
    // in Rooms 4-6 (qm per room: rm4→qm4, rm5→qm5, rm6→qm6). Shares the same 5 rounds as Pool A.
    // Round 1: (7,12)→rm4, (8,11)→rm5, (9,10)→rm6
    let game = GameBuilder::new_default(room_4.roomid, round_exp_s1_1.roundid)
        .set_poolbracket_id(pb_exp_1_b.pool_bracket_id)
        .set_leftteamid(team_7_experienced.teamid).set_rightteamid(team_12_experienced.teamid)
        .set_quizmasterid(qm_4.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_7_experienced, &team_12_experienced));
    let game = GameBuilder::new_default(room_5.roomid, round_exp_s1_1.roundid)
        .set_poolbracket_id(pb_exp_1_b.pool_bracket_id)
        .set_leftteamid(team_8_experienced.teamid).set_rightteamid(team_11_experienced.teamid)
        .set_quizmasterid(qm_5.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_8_experienced, &team_11_experienced));
    let game = GameBuilder::new_default(room_6.roomid, round_exp_s1_1.roundid)
        .set_poolbracket_id(pb_exp_1_b.pool_bracket_id)
        .set_leftteamid(team_9_experienced.teamid).set_rightteamid(team_10_experienced.teamid)
        .set_quizmasterid(qm_6.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_9_experienced, &team_10_experienced));
    // Round 2: (7,11)→rm5, (12,10)→rm6, (8,9)→rm4
    let game = GameBuilder::new_default(room_5.roomid, round_exp_s1_2.roundid)
        .set_poolbracket_id(pb_exp_1_b.pool_bracket_id)
        .set_leftteamid(team_7_experienced.teamid).set_rightteamid(team_11_experienced.teamid)
        .set_quizmasterid(qm_5.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_7_experienced, &team_11_experienced));
    let game = GameBuilder::new_default(room_6.roomid, round_exp_s1_2.roundid)
        .set_poolbracket_id(pb_exp_1_b.pool_bracket_id)
        .set_leftteamid(team_12_experienced.teamid).set_rightteamid(team_10_experienced.teamid)
        .set_quizmasterid(qm_6.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_12_experienced, &team_10_experienced));
    let game = GameBuilder::new_default(room_4.roomid, round_exp_s1_2.roundid)
        .set_poolbracket_id(pb_exp_1_b.pool_bracket_id)
        .set_leftteamid(team_8_experienced.teamid).set_rightteamid(team_9_experienced.teamid)
        .set_quizmasterid(qm_4.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_8_experienced, &team_9_experienced));
    // Round 3: (7,10)→rm6, (11,9)→rm4, (12,8)→rm5
    let game = GameBuilder::new_default(room_6.roomid, round_exp_s1_3.roundid)
        .set_poolbracket_id(pb_exp_1_b.pool_bracket_id)
        .set_leftteamid(team_7_experienced.teamid).set_rightteamid(team_10_experienced.teamid)
        .set_quizmasterid(qm_6.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_7_experienced, &team_10_experienced));
    let game = GameBuilder::new_default(room_4.roomid, round_exp_s1_3.roundid)
        .set_poolbracket_id(pb_exp_1_b.pool_bracket_id)
        .set_leftteamid(team_11_experienced.teamid).set_rightteamid(team_9_experienced.teamid)
        .set_quizmasterid(qm_4.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_11_experienced, &team_9_experienced));
    let game = GameBuilder::new_default(room_5.roomid, round_exp_s1_3.roundid)
        .set_poolbracket_id(pb_exp_1_b.pool_bracket_id)
        .set_leftteamid(team_12_experienced.teamid).set_rightteamid(team_8_experienced.teamid)
        .set_quizmasterid(qm_5.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_12_experienced, &team_8_experienced));
    // Round 4: (7,9)→rm4, (10,8)→rm6, (11,12)→rm5
    let game = GameBuilder::new_default(room_4.roomid, round_exp_s1_4.roundid)
        .set_poolbracket_id(pb_exp_1_b.pool_bracket_id)
        .set_leftteamid(team_7_experienced.teamid).set_rightteamid(team_9_experienced.teamid)
        .set_quizmasterid(qm_4.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_7_experienced, &team_9_experienced));
    let game = GameBuilder::new_default(room_6.roomid, round_exp_s1_4.roundid)
        .set_poolbracket_id(pb_exp_1_b.pool_bracket_id)
        .set_leftteamid(team_10_experienced.teamid).set_rightteamid(team_8_experienced.teamid)
        .set_quizmasterid(qm_6.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_10_experienced, &team_8_experienced));
    let game = GameBuilder::new_default(room_5.roomid, round_exp_s1_4.roundid)
        .set_poolbracket_id(pb_exp_1_b.pool_bracket_id)
        .set_leftteamid(team_11_experienced.teamid).set_rightteamid(team_12_experienced.teamid)
        .set_quizmasterid(qm_5.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_11_experienced, &team_12_experienced));
    // Round 5: (7,8)→rm5, (9,12)→rm4, (10,11)→rm6
    let game = GameBuilder::new_default(room_5.roomid, round_exp_s1_5.roundid)
        .set_poolbracket_id(pb_exp_1_b.pool_bracket_id)
        .set_leftteamid(team_7_experienced.teamid).set_rightteamid(team_8_experienced.teamid)
        .set_quizmasterid(qm_5.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_7_experienced, &team_8_experienced));
    let game = GameBuilder::new_default(room_4.roomid, round_exp_s1_5.roundid)
        .set_poolbracket_id(pb_exp_1_b.pool_bracket_id)
        .set_leftteamid(team_9_experienced.teamid).set_rightteamid(team_12_experienced.teamid)
        .set_quizmasterid(qm_4.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_9_experienced, &team_12_experienced));
    let game = GameBuilder::new_default(room_6.roomid, round_exp_s1_5.roundid)
        .set_poolbracket_id(pb_exp_1_b.pool_bracket_id)
        .set_leftteamid(team_10_experienced.teamid).set_rightteamid(team_11_experienced.teamid)
        .set_quizmasterid(qm_6.id).build_and_insert(db).unwrap();
    game_event_specs.push((game.gid, &team_10_experienced, &team_11_experienced));

    // Div: Novice — Session 1: full 4-team round-robin, 6 games / 3 rounds (2 games/round).
    // Fixed team alternates between rm7 and rm8 each round.
    // Round 1: (t1n,t4n)→rm7, (t2n,t3n)→rm8
    let game = GameBuilder::new_default(room_7.roomid, round_nov_s1_1.roundid)
        .set_poolbracket_id(pb_nov_1.pool_bracket_id)
        .set_leftteamid(team_1_novice.teamid)
        .set_rightteamid(team_4_novice.teamid)
        .set_quizmasterid(qm_4.id)
        .build_and_insert(db)
        .unwrap();
    game_event_specs.push((game.gid, &team_1_novice, &team_4_novice));
    let game = GameBuilder::new_default(room_8.roomid, round_nov_s1_1.roundid)
        .set_poolbracket_id(pb_nov_1.pool_bracket_id)
        .set_leftteamid(team_2_novice.teamid)
        .set_rightteamid(team_3_novice.teamid)
        .set_quizmasterid(qm_5.id)
        .build_and_insert(db)
        .unwrap();
    game_event_specs.push((game.gid, &team_2_novice, &team_3_novice));
    // Round 2: (t1n,t3n)→rm8, (t4n,t2n)→rm7
    let game = GameBuilder::new_default(room_8.roomid, round_nov_s1_2.roundid)
        .set_poolbracket_id(pb_nov_1.pool_bracket_id)
        .set_leftteamid(team_1_novice.teamid)
        .set_rightteamid(team_3_novice.teamid)
        .set_quizmasterid(qm_5.id)
        .build_and_insert(db)
        .unwrap();
    game_event_specs.push((game.gid, &team_1_novice, &team_3_novice));
    let game = GameBuilder::new_default(room_7.roomid, round_nov_s1_2.roundid)
        .set_poolbracket_id(pb_nov_1.pool_bracket_id)
        .set_leftteamid(team_4_novice.teamid)
        .set_rightteamid(team_2_novice.teamid)
        .set_quizmasterid(qm_4.id)
        .build_and_insert(db)
        .unwrap();
    game_event_specs.push((game.gid, &team_4_novice, &team_2_novice));
    // Round 3: (t1n,t2n)→rm7, (t3n,t4n)→rm8
    let game = GameBuilder::new_default(room_7.roomid, round_nov_s1_3.roundid)
        .set_poolbracket_id(pb_nov_1.pool_bracket_id)
        .set_leftteamid(team_1_novice.teamid)
        .set_rightteamid(team_2_novice.teamid)
        .set_quizmasterid(qm_4.id)
        .build_and_insert(db)
        .unwrap();
    game_event_specs.push((game.gid, &team_1_novice, &team_2_novice));
    let game = GameBuilder::new_default(room_8.roomid, round_nov_s1_3.roundid)
        .set_poolbracket_id(pb_nov_1.pool_bracket_id)
        .set_leftteamid(team_3_novice.teamid)
        .set_rightteamid(team_4_novice.teamid)
        .set_quizmasterid(qm_5.id)
        .build_and_insert(db)
        .unwrap();
    game_event_specs.push((game.gid, &team_3_novice, &team_4_novice));

    // Div: Novice — Session 2: full 4-team round-robin again, 6 games / 3 rounds. Rooms 7-8.
    // Round 1: (t1n,t4n)→rm7, (t2n,t3n)→rm8
    let game = GameBuilder::new_default(room_7.roomid, round_nov_s2_1.roundid)
        .set_poolbracket_id(pb_nov_2.pool_bracket_id)
        .set_leftteamid(team_1_novice.teamid)
        .set_rightteamid(team_4_novice.teamid)
        .set_quizmasterid(qm_4.id)
        .build_and_insert(db)
        .unwrap();
    game_event_specs.push((game.gid, &team_1_novice, &team_4_novice));
    let game = GameBuilder::new_default(room_8.roomid, round_nov_s2_1.roundid)
        .set_poolbracket_id(pb_nov_2.pool_bracket_id)
        .set_leftteamid(team_2_novice.teamid)
        .set_rightteamid(team_3_novice.teamid)
        .set_quizmasterid(qm_5.id)
        .build_and_insert(db)
        .unwrap();
    game_event_specs.push((game.gid, &team_2_novice, &team_3_novice));
    // Round 2: (t1n,t3n)→rm8, (t4n,t2n)→rm7
    let game = GameBuilder::new_default(room_8.roomid, round_nov_s2_2.roundid)
        .set_poolbracket_id(pb_nov_2.pool_bracket_id)
        .set_leftteamid(team_1_novice.teamid)
        .set_rightteamid(team_3_novice.teamid)
        .set_quizmasterid(qm_5.id)
        .build_and_insert(db)
        .unwrap();
    game_event_specs.push((game.gid, &team_1_novice, &team_3_novice));
    let game = GameBuilder::new_default(room_7.roomid, round_nov_s2_2.roundid)
        .set_poolbracket_id(pb_nov_2.pool_bracket_id)
        .set_leftteamid(team_4_novice.teamid)
        .set_rightteamid(team_2_novice.teamid)
        .set_quizmasterid(qm_4.id)
        .build_and_insert(db)
        .unwrap();
    game_event_specs.push((game.gid, &team_4_novice, &team_2_novice));
    // Round 3: (t1n,t2n)→rm7, (t3n,t4n)→rm8
    let game = GameBuilder::new_default(room_7.roomid, round_nov_s2_3.roundid)
        .set_poolbracket_id(pb_nov_2.pool_bracket_id)
        .set_leftteamid(team_1_novice.teamid)
        .set_rightteamid(team_2_novice.teamid)
        .set_quizmasterid(qm_4.id)
        .build_and_insert(db)
        .unwrap();
    game_event_specs.push((game.gid, &team_1_novice, &team_2_novice));
    let game = GameBuilder::new_default(room_8.roomid, round_nov_s2_3.roundid)
        .set_poolbracket_id(pb_nov_2.pool_bracket_id)
        .set_leftteamid(team_3_novice.teamid)
        .set_rightteamid(team_4_novice.teamid)
        .set_quizmasterid(qm_5.id)
        .build_and_insert(db)
        .unwrap();
    game_event_specs.push((game.gid, &team_3_novice, &team_4_novice));

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

    // Add every game of each division to its division's statsgroup (games_statsgroups).
    // A game's division is derived via its pool bracket (game -> pool_bracket -> division), so
    // read_all_games_of_division groups them correctly.
    let all_games_pagination = crate::models::common::PaginationParams {
        page: 0,
        page_size: crate::models::common::PaginationParams::MAX_PAGE_SIZE as i64,
    };
    for (division_did, statsgroup_sgid) in [
        (division_experienced.did, sg_experienced.sgid),
        (division_novice.did, sg_novice.sgid),
        (division_decades.did, sg_decades.sgid),
    ] {
        let games = crate::models::game::read_all_games_of_division(db, division_did, &all_games_pagination).unwrap();
        for game in games {
            GameStatsGroupBuilder::new(game.gid, statsgroup_sgid)
                .build_and_insert(db)
                .unwrap();
        }
    }

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
        .set_lname("Torres")
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
        .set_lname("Blake")
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
        .set_lname("Morgan")
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
    let team_group = TeamGroupBuilder::new(pool_bracket_id)
        .set_creator_userid(creator)
        .build_and_insert(db)
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

        match next(10) {
            0..=5 => {
                // Toss-up answered correctly.
                insert_game_event(db, gid, q, qev, &quizzer_name, answering_team, seat, GameEventCode::TC, ts); qev += 1; ts = ts + Duration::minutes(1);
            }
            6..=8 => {
                // Toss-up error, then the other team attempts the bonus.
                insert_game_event(db, gid, q, qev, &quizzer_name, answering_team, seat, GameEventCode::TE, ts); qev += 1; ts = ts + Duration::minutes(1);
                let other_team = 1 - answering_team;
                let other_quizzers = if other_team == 0 { &left_quizzers } else { &right_quizzers };
                let other_seat = next(other_quizzers.len() as u64) as i32;
                let other_name = other_quizzers[other_seat as usize].clone();
                let bonus = if next(2) == 0 { GameEventCode::BC } else { GameEventCode::BE };
                insert_game_event(db, gid, q, qev, &other_name, other_team, other_seat, bonus, ts); qev += 1; ts = ts + Duration::minutes(1);
            }
            _ => {
                // No jump on the toss-up.
                insert_game_event(db, gid, q, qev, "No Jump", 0, 0, GameEventCode::NJ, ts); qev += 1; ts = ts + Duration::minutes(1);
            }
        }
        let _ = qev;
    }
}

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
