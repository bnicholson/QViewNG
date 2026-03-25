use crate::{constants::SUPER_USER, database, models::{division::DivisionBuilder, game::GameBuilder, permission::PermissionBuilder, role::RoleBuilder, role_permission::RolePermissionBuilder, room::RoomBuilder, round::RoundBuilder, team::TeamBuilder, tournament::TournamentBuilder, user::UserBuilder, users_roles::UsersRolesBuilder}};
use chrono::{Local, NaiveDate, Duration};

pub fn seed_data_one(db: &mut database::Connection) {
    init_roles_and_permissions(db);
    add_super_user(db);
    add_tournament_manager_user(db);
    add_member_user(db);
    add_tour_1_demo(db);
}

pub fn add_super_user(db: &mut database::Connection) {
    let super_user = UserBuilder::new("Super")
        .set_lname("User")
        .set_username("goqview")
        .set_hash_password("Password123!")
        .set_email("goqview@fakeemail.com")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();

    let member_role    = crate::models::role::read_by_name(db, "member").unwrap();
    let super_user_role = crate::models::role::read_by_name(db, SUPER_USER).unwrap();

    UsersRolesBuilder::new(super_user.id)
        .assign(member_role.id)
        .assign(super_user_role.id)
        .build_and_insert(db)
        .unwrap();
}

pub fn add_tournament_manager_user(db: &mut database::Connection) {
    let user = UserBuilder::new("Taylor")
        .set_lname("Morgan")
        .set_username("tourmanager")
        .set_hash_password("Password123!")
        .set_email("tmorgan@fakeemail.com")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();

    let member_role             = crate::models::role::read_by_name(db, "member").unwrap();
    let tournament_manager_role = crate::models::role::read_by_name(db, "tournament_manager").unwrap();

    UsersRolesBuilder::new(user.id)
        .assign(member_role.id)
        .assign(tournament_manager_role.id)
        .build_and_insert(db)
        .unwrap();
}

pub fn add_member_user(db: &mut database::Connection) {
    let user = UserBuilder::new("Jordan")
        .set_lname("Ellis")
        .set_username("member")
        .set_hash_password("Password123!")
        .set_email("jellis@fakeemail.com")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();

    let member_role = crate::models::role::read_by_name(db, "member").unwrap();

    UsersRolesBuilder::new(user.id)
        .assign(member_role.id)
        .build_and_insert(db)
        .unwrap();
}

/// Idempotent baseline setup — creates all canonical permissions and the three
/// application roles. Does NOT assign any roles to users; call separately after
/// inserting users (e.g. via `UsersRolesBuilder`).
///
/// Role permission tree (each role is self-contained):
///
///  member             → :read on every resource
///  tournament_manager → tournament:create/update/delete only (member covers :read)
///  super_user         → full CRUD on every resource
pub fn init_roles_and_permissions(db: &mut database::Connection) {
    let resources = ["tournament", "division", "round", "room", "game", "team", "user"];
    let actions   = ["create", "read", "update", "delete"];

    // ── Build permissions ──────────────────────────────────────────────
    //
    // Stored as a 2-D array indexed [resource][action] matching the order above
    // so we can reference individual cells when composing role trees below.

    let perms: Vec<Vec<_>> = resources.iter().map(|resource| {
        actions.iter().map(|action| {
            PermissionBuilder::new(&format!("{}:{}", resource, action))
                .resource(resource)
                .action(action)
                .build_and_insert(db)
                .unwrap()
        }).collect()
    }).collect();

    // Convenience closures: find permission IDs for a resource by action index.
    // actions: 0=create, 1=read, 2=update, 3=delete
    let read_ids: Vec<i32> = perms.iter().map(|r| r[1].id).collect();

    let all_ids: Vec<i32> = perms.iter()
        .flat_map(|r| r.iter().map(|p| p.id))
        .collect();

    // ── member: read-only on everything ──────────────────────────────────────
    let member = RoleBuilder::new("member")
        .description("View all resources; no write access")
        .build_and_insert(db)
        .unwrap();

    RolePermissionBuilder::new(member.id)
        .add_many(read_ids.iter().copied())
        .build_and_insert(db)
        .unwrap();

    // ── tournament_manager: write access on tournaments only ──────────────────
    // :read on all resources is already granted by member, so this role carries
    // only the three additional permissions that member does not have.
    // actions: 0=create, 1=read, 2=update, 3=delete  (perms[0] = tournament)
    let tournament_manager = RoleBuilder::new("tournament_manager")
        .description("Create, update, and delete tournaments (assign alongside member)")
        .build_and_insert(db)
        .unwrap();

    RolePermissionBuilder::new(tournament_manager.id)
        .add(perms[0][0].id)  // tournament:create
        .add(perms[0][2].id)  // tournament:update
        .add(perms[0][3].id)  // tournament:delete
        .build_and_insert(db)
        .unwrap();

    // ── super_user: full CRUD on everything ──────────────────────────────────
    let super_user = RoleBuilder::new(SUPER_USER)
        .description("Unrestricted access to all resources and actions")
        .build_and_insert(db)
        .unwrap();

    RolePermissionBuilder::new(super_user.id)
        .add_many(all_ids)
        .build_and_insert(db)
        .unwrap();
}

pub fn add_tour_1_demo(db: &mut database::Connection) {
    let tour_owner = crate::models::user::UserBuilder::new("Alex")
        .set_lname("Rivera")
        .set_username("arivera")
        .set_hash_password("Alex@Riv3ra!")
        .set_email("arivera@fakeemail.com")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();

    let member_role             = crate::models::role::read_by_name(db, "member").unwrap();
    let tournament_manager_role = crate::models::role::read_by_name(db, "tournament_manager").unwrap();

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

    let division_experienced = DivisionBuilder::new_default("Experienced", tour.tid)
        .set_shortinfo("Been around the block".to_string())
        .build_and_insert(db)
        .unwrap();
    let division_novice = DivisionBuilder::new_default("Novice", tour.tid)
        .set_shortinfo("New to this".to_string())
        .build_and_insert(db)
        .unwrap();
    let division_decades = DivisionBuilder::new_default("Decades", tour.tid)
        .set_shortinfo("Young at heart!".to_string())
        .build_and_insert(db)
        .unwrap();

    let room_1 = RoomBuilder::new_default("Room 1", tour.tid)
        .set_comments("".to_string())
        .set_clientkey(Some("bankdiu".to_string()))
        .build_and_insert(db)
        .unwrap();
    let room_2 = RoomBuilder::new_default("Room 2", tour.tid)
        .set_comments("".to_string())
        .set_clientkey(Some("bbhsth4".to_string()))
        .build_and_insert(db)
        .unwrap();
    let room_3 = RoomBuilder::new_default("Room 3", tour.tid)
        .set_comments("".to_string())
        .set_clientkey(Some("16587397".to_string()))
        .build_and_insert(db)
        .unwrap();
    let room_4 = RoomBuilder::new_default("Room 4", tour.tid)
        .set_comments("".to_string())
        .set_clientkey(Some("aplyhen".to_string()))
        .build_and_insert(db)
        .unwrap();
    let room_5 = RoomBuilder::new_default("Room 5", tour.tid)
        .set_comments("".to_string())
        .set_clientkey(Some("llpjhin".to_string()))
        .build_and_insert(db)
        .unwrap();
    let room_6 = RoomBuilder::new_default("Room 6", tour.tid)
        .set_comments("".to_string())
        .set_clientkey(Some("qwx7bfyh".to_string()))
        .build_and_insert(db)
        .unwrap();
    let room_7 = RoomBuilder::new_default("Room 7", tour.tid)
        .set_comments("".to_string())
        .set_clientkey(Some("jjkalndi".to_string()))
        .build_and_insert(db)
        .unwrap();

    // Div: Experienced
    let coach_exp_1 = UserBuilder::new("Lena")
        .set_lname("Hartwell")
        .set_email("lhartwell@fakeemail.com")
        .set_username("lhartwell")
        .set_hash_password("Lena@H4rt")
        .set_activated(true)
        .build_and_insert(db)
        .unwrap();
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
    let team_1_experienced = TeamBuilder::new_default(division_experienced.did)
        .set_name("Iron Covenant")
        .set_coachid(coach_exp_1.id)
        .set_quizzer_one_id(q_exp_1_1.id)
        .set_quizzer_two_id(q_exp_1_2.id)
        .set_quizzer_three_id(q_exp_1_3.id)
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
        .build_and_insert(db)
        .unwrap();
    let round_2_experienced = RoundBuilder::new_default(division_experienced.did)
        .build_and_insert(db)
        .unwrap();
    let round_3_experienced = RoundBuilder::new_default(division_experienced.did)
        .build_and_insert(db)
        .unwrap();
    let round_4_experienced = RoundBuilder::new_default(division_experienced.did)
        .build_and_insert(db)
        .unwrap();
    let round_5_experienced = RoundBuilder::new_default(division_experienced.did)
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
        .set_coachid(coach_nov_1.id)
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
        .build_and_insert(db)
        .unwrap();
    let round_2_novice = RoundBuilder::new_default(division_novice.did)
        .build_and_insert(db)
        .unwrap();
    let round_3_novice = RoundBuilder::new_default(division_novice.did)
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
        .build_and_insert(db)
        .unwrap();
    let round_2_decades = RoundBuilder::new_default(division_decades.did)
        .build_and_insert(db)
        .unwrap();
    let round_3_decades = RoundBuilder::new_default(division_decades.did)
        .build_and_insert(db)
        .unwrap();

    // One quizmaster per room
    let qm_1 = UserBuilder::new("Jordan")
        .set_lname("Avery")
        .set_email("javery@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let qm_2 = UserBuilder::new("Riley")
        .set_lname("Blake")
        .set_email("rblake@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let qm_3 = UserBuilder::new("Morgan")
        .set_lname("Casey")
        .set_email("mcasey@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let qm_4 = UserBuilder::new("Quinn")
        .set_lname("Drew")
        .set_email("qdrew@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let qm_5 = UserBuilder::new("Sage")
        .set_lname("Ellis")
        .set_email("sellis@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let qm_6 = UserBuilder::new("Avery")
        .set_lname("Flynn")
        .set_email("aflynn@fakeemail.com")
        .set_activated(false)
        .build_and_insert(db)
        .unwrap();
    let qm_7 = UserBuilder::new("Blake")
        .set_lname("Grant")
        .set_email("bgrant@fakeemail.com")
        .set_activated(false)
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
        .set_quizmasterid(qm_6.id)
        .build_and_insert(db)
        .unwrap();
    GameBuilder::new_default(room_7.roomid, round_1_decades.roundid)
        .set_leftteamid(team_2_decades.teamid)
        .set_rightteamid(team_3_decades.teamid)
        .set_quizmasterid(qm_7.id)
        .build_and_insert(db)
        .unwrap();
    // Round 2: (t1d,t3d)→rm7, (t4d,t2d)→rm6
    GameBuilder::new_default(room_7.roomid, round_2_decades.roundid)
        .set_leftteamid(team_1_decades.teamid)
        .set_rightteamid(team_3_decades.teamid)
        .set_quizmasterid(qm_7.id)
        .build_and_insert(db)
        .unwrap();
    GameBuilder::new_default(room_6.roomid, round_2_decades.roundid)
        .set_leftteamid(team_4_decades.teamid)
        .set_rightteamid(team_2_decades.teamid)
        .set_quizmasterid(qm_6.id)
        .build_and_insert(db)
        .unwrap();
    // Round 3: (t1d,t2d)→rm6, (t3d,t4d)→rm7
    GameBuilder::new_default(room_6.roomid, round_3_decades.roundid)
        .set_leftteamid(team_1_decades.teamid)
        .set_rightteamid(team_2_decades.teamid)
        .set_quizmasterid(qm_6.id)
        .build_and_insert(db)
        .unwrap();
    GameBuilder::new_default(room_7.roomid, round_3_decades.roundid)
        .set_leftteamid(team_3_decades.teamid)
        .set_rightteamid(team_4_decades.teamid)
        .set_quizmasterid(qm_7.id)
        .build_and_insert(db)
        .unwrap();

    // Assign member role to all coaches
    let member_role = crate::models::role::read_by_name(db, "member").unwrap();
    let coaches = [
        coach_exp_1.id, coach_exp_2.id, coach_exp_3.id,
        coach_exp_4.id, coach_exp_5.id, coach_exp_6.id,
        coach_nov_1.id, coach_nov_2.id, coach_nov_3.id, coach_nov_4.id,
        coach_dec_1.id, coach_dec_2.id, coach_dec_3.id, coach_dec_4.id,
    ];
    for coach_id in coaches {
        UsersRolesBuilder::new(coach_id)
            .assign(member_role.id)
            .build_and_insert(db)
            .unwrap();
    }
}
