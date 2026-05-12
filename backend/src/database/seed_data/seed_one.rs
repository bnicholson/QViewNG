use crate::{database::{self, seed_data::system_default_data::default_password}, models::{computer::ComputerBuilder, create_tournament_applicant::CreateTournamentApplicantBuilder, division::DivisionBuilder, equipmentregistration::{EquipmentRegistrationBuilder, EquipmentRegistrationStatus}, extensioncord::ExtensionCordBuilder, game::GameBuilder, interfacebox::InterfaceBoxBuilder, jumppad::JumpPadBuilder, microphonerecorder::MicrophoneRecorderBuilder, monitor::MonitorBuilder, powerstrip::PowerStripBuilder, projector::ProjectorBuilder, role::AppRole, room::RoomBuilder, roster::RosterBuilder, roster_coach::RosterCoachBuilder, roster_quizzer::RosterQuizzerBuilder, round::RoundBuilder, team::TeamBuilder, tournament::TournamentBuilder, tournament_admin::TournamentAdminBuilder, tournamentgroup::TournamentGroupBuilder, tournamentgroup_tournament::TournamentGroupTournamentBuilder, user::UserBuilder, users_roles::UsersRolesBuilder}};
use chrono::{Local, NaiveDate, Duration, TimeZone, Utc};

pub fn insert_seed_data_one(db: &mut database::Connection) {
    add_tour_1_demo(db);
    create_tournament_applicants(db);
}

pub fn add_tour_1_demo(db: &mut database::Connection) {

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
        .set_city("Nashville")
        .set_country("USA")
        .set_contact("Skipper Jets")
        .set_contactemail("skippyjets@yahoo.com")
        .set_shortinfo("Display standard data")
        .set_info("This Tournament is intended to show the visitor what a fully data-entered Tournament would look like.")
        .set_owner_id(tour_owner.id)
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
        .set_shortinfo("Been around the block".to_string())
        .set_is_public(true)
        .build_and_insert(db)
        .unwrap();
    let division_novice = DivisionBuilder::new_default("Novice", tour.tid)
        .set_shortinfo("New to this".to_string())
        .set_is_public(true)
        .build_and_insert(db)
        .unwrap();
    let division_decades = DivisionBuilder::new_default("Decades", tour.tid)
        .set_shortinfo("Young at heart!".to_string())
        .set_is_public(true)
        .build_and_insert(db)
        .unwrap();

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
        .set_comments("".to_string())
        .set_clientkey(Some("bankdiu".to_string()))
        .set_quizmaster_id(Some(qm_1.id))
        .set_contentjudge_id(Some(cj_1.id))
        .build_and_insert(db)
        .unwrap();
    let room_2 = RoomBuilder::new_default("Room 2", tour.tid)
        .set_comments("".to_string())
        .set_clientkey(Some("bbhsth4".to_string()))
        .set_quizmaster_id(Some(qm_2.id))
        .set_contentjudge_id(Some(cj_2.id))
        .build_and_insert(db)
        .unwrap();
    let room_3 = RoomBuilder::new_default("Room 3", tour.tid)
        .set_comments("".to_string())
        .set_clientkey(Some("16587397".to_string()))
        .set_quizmaster_id(Some(qm_3.id))
        .set_contentjudge_id(Some(cj_3.id))
        .build_and_insert(db)
        .unwrap();
    let room_4 = RoomBuilder::new_default("Room 4", tour.tid)
        .set_comments("".to_string())
        .set_clientkey(Some("aplyhen".to_string()))
        .set_quizmaster_id(Some(qm_4.id))
        .set_contentjudge_id(Some(cj_4.id))
        .build_and_insert(db)
        .unwrap();
    let room_5 = RoomBuilder::new_default("Room 5", tour.tid)
        .set_comments("".to_string())
        .set_clientkey(Some("llpjhin".to_string()))
        .set_quizmaster_id(Some(qm_5.id))
        .set_contentjudge_id(Some(cj_5.id))
        .build_and_insert(db)
        .unwrap();
    let room_6 = RoomBuilder::new_default("Room 6", tour.tid)
        .set_comments("".to_string())
        .set_clientkey(Some("qwx7bfyh".to_string()))
        .set_quizmaster_id(Some(qm_6.id))
        .set_contentjudge_id(Some(cj_6.id))
        .build_and_insert(db)
        .unwrap();
    let room_7 = RoomBuilder::new_default("Room 7", tour.tid)
        .set_comments("".to_string())
        .set_clientkey(Some("jjkalndi".to_string()))
        .set_quizmaster_id(Some(qm_7.id))
        .set_contentjudge_id(Some(cj_7.id))
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

    let round_1_experienced = RoundBuilder::new_default(division_experienced.did)
        .set_name("1")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 12,  0, 0).unwrap())
        .build_and_insert(db)
        .unwrap();
    let round_2_experienced = RoundBuilder::new_default(division_experienced.did)
        .set_name("2")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 12, 30, 0).unwrap())
        .build_and_insert(db)
        .unwrap();
    let round_3_experienced = RoundBuilder::new_default(division_experienced.did)
        .set_name("3")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 13,  0, 0).unwrap())
        .build_and_insert(db)
        .unwrap();
    let round_4_experienced = RoundBuilder::new_default(division_experienced.did)
        .set_name("4")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 13, 30, 0).unwrap())
        .build_and_insert(db)
        .unwrap();
    let round_5_experienced = RoundBuilder::new_default(division_experienced.did)
        .set_name("5")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 14,  0, 0).unwrap())
        .build_and_insert(db)
        .unwrap();

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

    let round_1_novice = RoundBuilder::new_default(division_novice.did)
        .set_name("1")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 12,  0, 0).unwrap())
        .build_and_insert(db)
        .unwrap();
    let round_2_novice = RoundBuilder::new_default(division_novice.did)
        .set_name("2")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 12, 30, 0).unwrap())
        .build_and_insert(db)
        .unwrap();
    let round_3_novice = RoundBuilder::new_default(division_novice.did)
        .set_name("3")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 13,  0, 0).unwrap())
        .build_and_insert(db)
        .unwrap();

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

    let round_1_decades = RoundBuilder::new_default(division_decades.did)
        .set_name("1")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 12,  0, 0).unwrap())
        .build_and_insert(db)
        .unwrap();
    let round_2_decades = RoundBuilder::new_default(division_decades.did)
        .set_name("2")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 12, 30, 0).unwrap())
        .build_and_insert(db)
        .unwrap();
    let round_3_decades = RoundBuilder::new_default(division_decades.did)
        .set_name("3")
        .set_scheduled_start_time(Utc.with_ymd_and_hms(2055, 5, 23, 13,  0, 0).unwrap())
        .build_and_insert(db)
        .unwrap();

    // Games — round-robin schedule (polygon rotation, team 1 fixed)
    // Experienced rooms 1-3, Novice rooms 4-5, Decades rooms 6-7

    // Div: Experienced — 15 games across 5 rounds (3 games/round)
    // Room rotation per round: R1→[1,2,3], R2→[2,3,1], R3→[3,1,2], R4→[1,3,2], R5→[2,1,3]
    // Round 1: (t1e,t6e)→rm1, (t2e,t5e)→rm2, (t3e,t4e)→rm3
    GameBuilder::new_default(room_1.roomid, round_1_experienced.roundid)
        .set_leftteamid(team_1_experienced.teamid)
        .set_rightteamid(team_6_experienced.teamid)
        .set_quizmasterid(qm_1.id)
        .build_and_insert(db)
        .unwrap();
    GameBuilder::new_default(room_2.roomid, round_1_experienced.roundid)
        .set_leftteamid(team_2_experienced.teamid)
        .set_rightteamid(team_5_experienced.teamid)
        .set_quizmasterid(qm_2.id)
        .build_and_insert(db)
        .unwrap();
    GameBuilder::new_default(room_3.roomid, round_1_experienced.roundid)
        .set_leftteamid(team_3_experienced.teamid)
        .set_rightteamid(team_4_experienced.teamid)
        .set_quizmasterid(qm_3.id)
        .build_and_insert(db)
        .unwrap();
    // Round 2: (t1e,t5e)→rm2, (t6e,t4e)→rm3, (t2e,t3e)→rm1
    GameBuilder::new_default(room_2.roomid, round_2_experienced.roundid)
        .set_leftteamid(team_1_experienced.teamid)
        .set_rightteamid(team_5_experienced.teamid)
        .set_quizmasterid(qm_2.id)
        .build_and_insert(db)
        .unwrap();
    GameBuilder::new_default(room_3.roomid, round_2_experienced.roundid)
        .set_leftteamid(team_6_experienced.teamid)
        .set_rightteamid(team_4_experienced.teamid)
        .set_quizmasterid(qm_3.id)
        .build_and_insert(db)
        .unwrap();
    GameBuilder::new_default(room_1.roomid, round_2_experienced.roundid)
        .set_leftteamid(team_2_experienced.teamid)
        .set_rightteamid(team_3_experienced.teamid)
        .set_quizmasterid(qm_1.id)
        .build_and_insert(db)
        .unwrap();
    // Round 3: (t1e,t4e)→rm3, (t5e,t3e)→rm1, (t6e,t2e)→rm2
    GameBuilder::new_default(room_3.roomid, round_3_experienced.roundid)
        .set_leftteamid(team_1_experienced.teamid)
        .set_rightteamid(team_4_experienced.teamid)
        .set_quizmasterid(qm_3.id)
        .build_and_insert(db)
        .unwrap();
    GameBuilder::new_default(room_1.roomid, round_3_experienced.roundid)
        .set_leftteamid(team_5_experienced.teamid)
        .set_rightteamid(team_3_experienced.teamid)
        .set_quizmasterid(qm_1.id)
        .build_and_insert(db)
        .unwrap();
    GameBuilder::new_default(room_2.roomid, round_3_experienced.roundid)
        .set_leftteamid(team_6_experienced.teamid)
        .set_rightteamid(team_2_experienced.teamid)
        .set_quizmasterid(qm_2.id)
        .build_and_insert(db)
        .unwrap();
    // Round 4: (t1e,t3e)→rm1, (t4e,t2e)→rm3, (t5e,t6e)→rm2
    GameBuilder::new_default(room_1.roomid, round_4_experienced.roundid)
        .set_leftteamid(team_1_experienced.teamid)
        .set_rightteamid(team_3_experienced.teamid)
        .set_quizmasterid(qm_1.id)
        .build_and_insert(db)
        .unwrap();
    GameBuilder::new_default(room_3.roomid, round_4_experienced.roundid)
        .set_leftteamid(team_4_experienced.teamid)
        .set_rightteamid(team_2_experienced.teamid)
        .set_quizmasterid(qm_3.id)
        .build_and_insert(db)
        .unwrap();
    GameBuilder::new_default(room_2.roomid, round_4_experienced.roundid)
        .set_leftteamid(team_5_experienced.teamid)
        .set_rightteamid(team_6_experienced.teamid)
        .set_quizmasterid(qm_2.id)
        .build_and_insert(db)
        .unwrap();
    // Round 5: (t1e,t2e)→rm2, (t3e,t6e)→rm1, (t4e,t5e)→rm3
    GameBuilder::new_default(room_2.roomid, round_5_experienced.roundid)
        .set_leftteamid(team_1_experienced.teamid)
        .set_rightteamid(team_2_experienced.teamid)
        .set_quizmasterid(qm_2.id)
        .build_and_insert(db)
        .unwrap();
    GameBuilder::new_default(room_1.roomid, round_5_experienced.roundid)
        .set_leftteamid(team_3_experienced.teamid)
        .set_rightteamid(team_6_experienced.teamid)
        .set_quizmasterid(qm_1.id)
        .build_and_insert(db)
        .unwrap();
    GameBuilder::new_default(room_3.roomid, round_5_experienced.roundid)
        .set_leftteamid(team_4_experienced.teamid)
        .set_rightteamid(team_5_experienced.teamid)
        .set_quizmasterid(qm_3.id)
        .build_and_insert(db)
        .unwrap();

    // Div: Novice — 6 games across 3 rounds (2 games/round)
    // Fixed team alternates between rm4 and rm5 each round
    // Round 1: (t1n,t4n)→rm4, (t2n,t3n)→rm5
    GameBuilder::new_default(room_4.roomid, round_1_novice.roundid)
        .set_leftteamid(team_1_novice.teamid)
        .set_rightteamid(team_4_novice.teamid)
        .set_quizmasterid(qm_4.id)
        .build_and_insert(db)
        .unwrap();
    GameBuilder::new_default(room_5.roomid, round_1_novice.roundid)
        .set_leftteamid(team_2_novice.teamid)
        .set_rightteamid(team_3_novice.teamid)
        .set_quizmasterid(qm_5.id)
        .build_and_insert(db)
        .unwrap();
    // Round 2: (t1n,t3n)→rm5, (t4n,t2n)→rm4
    GameBuilder::new_default(room_5.roomid, round_2_novice.roundid)
        .set_leftteamid(team_1_novice.teamid)
        .set_rightteamid(team_3_novice.teamid)
        .set_quizmasterid(qm_5.id)
        .build_and_insert(db)
        .unwrap();
    GameBuilder::new_default(room_4.roomid, round_2_novice.roundid)
        .set_leftteamid(team_4_novice.teamid)
        .set_rightteamid(team_2_novice.teamid)
        .set_quizmasterid(qm_4.id)
        .build_and_insert(db)
        .unwrap();
    // Round 3: (t1n,t2n)→rm4, (t3n,t4n)→rm5
    GameBuilder::new_default(room_4.roomid, round_3_novice.roundid)
        .set_leftteamid(team_1_novice.teamid)
        .set_rightteamid(team_2_novice.teamid)
        .set_quizmasterid(qm_4.id)
        .build_and_insert(db)
        .unwrap();
    GameBuilder::new_default(room_5.roomid, round_3_novice.roundid)
        .set_leftteamid(team_3_novice.teamid)
        .set_rightteamid(team_4_novice.teamid)
        .set_quizmasterid(qm_5.id)
        .build_and_insert(db)
        .unwrap();

    // Div: Decades — 6 games across 3 rounds (2 games/round)
    // Fixed team alternates between rm6 and rm7 each round
    // Round 1: (t1d,t4d)→rm6, (t2d,t3d)→rm7
    GameBuilder::new_default(room_6.roomid, round_1_decades.roundid)
        .set_leftteamid(team_1_decades.teamid)
        .set_rightteamid(team_4_decades.teamid)
        .set_quizmasterid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    GameBuilder::new_default(room_7.roomid, round_1_decades.roundid)
        .set_leftteamid(team_2_decades.teamid)
        .set_rightteamid(team_3_decades.teamid)
        .set_quizmasterid(qm_7.id)
        .set_contentjudgeid(Some(tour_owner.id))
        .build_and_insert(db)
        .unwrap();
    // Round 2: (t1d,t3d)→rm7, (t4d,t2d)→rm6
    GameBuilder::new_default(room_7.roomid, round_2_decades.roundid)
        .set_leftteamid(team_1_decades.teamid)
        .set_rightteamid(team_3_decades.teamid)
        .set_quizmasterid(qm_7.id)
        .set_contentjudgeid(Some(tour_owner.id))
        .build_and_insert(db)
        .unwrap();
    GameBuilder::new_default(room_6.roomid, round_2_decades.roundid)
        .set_leftteamid(team_4_decades.teamid)
        .set_rightteamid(team_2_decades.teamid)
        .set_quizmasterid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    // Round 3: (t1d,t2d)→rm6, (t3d,t4d)→rm7
    GameBuilder::new_default(room_6.roomid, round_3_decades.roundid)
        .set_leftteamid(team_1_decades.teamid)
        .set_rightteamid(team_2_decades.teamid)
        .set_quizmasterid(tour_owner.id)
        .build_and_insert(db)
        .unwrap();
    GameBuilder::new_default(room_7.roomid, round_3_decades.roundid)
        .set_leftteamid(team_3_decades.teamid)
        .set_rightteamid(team_4_decades.teamid)
        .set_quizmasterid(qm_7.id)
        .set_contentjudgeid(Some(tour_owner.id))
        .build_and_insert(db)
        .unwrap();

    // Assign member role to all coaches
    let member_role = crate::models::role::read_by_name(db, "member").unwrap();
    let coaches = [
        coach_exp_1.id, coach_exp_2.id, coach_exp_3.id,
        coach_exp_4.id, coach_exp_5.id, coach_exp_6.id,
        coach_nov_2.id, coach_nov_3.id, coach_nov_4.id,
        coach_dec_1.id, coach_dec_2.id, coach_dec_3.id, coach_dec_4.id,
    ];
    for coach_id in coaches {
        UsersRolesBuilder::new(coach_id)
            .assign(member_role.id)
            .build_and_insert(db)
            .unwrap();
    }
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
