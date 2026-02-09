use backend::{database, models::{equipmentregistration::EquipmentRegistration, tournament::{NewTournament, Tournament, TournamentBuilder}, tournamentgroup::{TournamentGroup, TournamentGroupBuilder}, tournamentgroup_tournament::TournamentGroupTournamentBuilder}};
use chrono::{Duration, Local, Months, NaiveDate, TimeZone, Utc};
use crate::fixtures::{divisions::{seed_division_with_name, seed_divisions_with_names}, equipmentregistrations::seed_1_equipmentregistration_for_each_equipment_type_with_minimum_required_dependencies, rooms::seed_rooms_with_names, rounds::seed_rounds_with_sched_start_times};

pub fn get_tournament_payload() -> NewTournament {
    TournamentBuilder::new_default("Test Tour").build().unwrap()
}
    
pub fn seed_tournament(db: &mut database::Connection, tname: &str) -> Tournament {
    TournamentBuilder::new_default(tname).build_and_insert(db).unwrap()
}

pub fn seed_tournaments_with_names(db: &mut database::Connection, tname_1: &str, tname_2: &str, tname_3: &str) -> Vec<Tournament> {
    let tour_builder = TournamentBuilder::new_default(tname_1)
        .set_venue("Olivet Nazarene University")
        .set_city("Bourbonnais")
        .set_region("Central USA")
        .set_country("USA")
        .set_contact("Jason Morton")
        .set_contactemail("jasonmorton@fakeemail.com")
        .set_shortinfo("NYI International quiz meet of 2025.")
        .set_info("If I wanted a longer description I would have provided it here.");
    
    let tour_1 = tour_builder
        .clone()
        .build_and_insert(db)
        .unwrap();

    let tour_2 = tour_builder
        .clone()
        .set_breadcrumb("/test/bread/crumb")
        .set_tname(tname_2)
        .build_and_insert(db)
        .unwrap();

    let tour_3 = tour_builder
        .set_tname(tname_3)
        .build_and_insert(db)
        .unwrap();

    vec![tour_1, tour_2, tour_3]
}

pub fn seed_tournaments(db: &mut database::Connection) -> Vec<Tournament> {
    seed_tournaments_with_names(db, "Q2025", "Tour 2", "Tour 3")
}

pub fn seed_tournaments_for_get_today(db: &mut database::Connection) -> Vec<Tournament> {
    let today = Local::now().date_naive();
    let two_months_from_today: NaiveDate = today.checked_add_months(Months::new(2)).unwrap();
    let days_10_past: NaiveDate = today - Duration::days(10);
    let days_10_future: NaiveDate = today + Duration::days(10);
    let one_month_before_today: NaiveDate = today.checked_sub_months(Months::new(1)).unwrap();

    let tour_1 = TournamentBuilder::new_default("2 months in the future exactly")
        .set_fromdate(two_months_from_today)
        .set_todate(two_months_from_today)
        .build_and_insert(db)
        .unwrap();

    let tour_2 = TournamentBuilder::new_default("20 Days, Including Today")
        .set_fromdate(days_10_past)
        .set_todate(days_10_future)
        .build_and_insert(db)
        .unwrap();

    let tour_3 = TournamentBuilder::new_default("Today Exactly")
        .set_fromdate(today)
        .set_todate(today)
        .build_and_insert(db)
        .unwrap();

    let tour_4 = TournamentBuilder::new_default("1 month ago exactly")
        .set_fromdate(one_month_before_today)
        .set_todate(one_month_before_today)
        .build_and_insert(db)
        .unwrap();

    vec![tour_1,tour_2,tour_3,tour_4]
}

pub fn seed_tournaments_for_get_all_in_date_range(db: &mut database::Connection) -> Vec<Tournament> {
    let today = Local::now().date_naive();
    let days_8_past: NaiveDate = today - Duration::days(8);
    let days_8_future: NaiveDate = today + Duration::days(8);
    let days_12_future: NaiveDate = today + Duration::days(12);
    let two_months_from_today: NaiveDate = today.checked_add_months(Months::new(2)).unwrap();
    let one_month_before_today: NaiveDate = today.checked_sub_months(Months::new(1)).unwrap();

    let tour_1 = TournamentBuilder::new_default("2 months in the future exactly")
        .set_fromdate(two_months_from_today)
        .set_todate(two_months_from_today)
        .build_and_insert(db)
        .unwrap();

    let tour_2 = TournamentBuilder::new_default("eight days past exactly")
        .set_fromdate(days_8_past)
        .set_todate(days_8_past)
        .build_and_insert(db)
        .unwrap();

    let tour_3 = TournamentBuilder::new_default("eight to twelve days future")
        .set_fromdate(days_8_future)
        .set_todate(days_12_future)
        .build_and_insert(db)
        .unwrap();

    let tour_4 = TournamentBuilder::new_default("Today Exactly")
        .set_fromdate(today)
        .set_todate(today)
        .build_and_insert(db)
        .unwrap();

    let tour_5 = TournamentBuilder::new_default("1 month ago exactly")
        .set_fromdate(one_month_before_today)
        .set_todate(one_month_before_today)
        .build_and_insert(db)
        .unwrap();

    vec![tour_1,tour_2,tour_3,tour_4,tour_5]
}

pub fn seed_get_divisions_by_tournament(db: &mut database::Connection) -> Tournament {
    let tournaments = seed_tournaments_with_names(db, "T1", "T2", "T3");  // unique names not really needed (realizing after the fact)

    let tour_1 = &tournaments[0];
    let tour_1_divisions = seed_divisions_with_names(db, tour_1.tid, "Test Div 3276", "Test Div 9078", "Test Div 4611");

    let tour_2 = &tournaments[1];
    let tour_2_divisions = seed_divisions_with_names(db, tour_2.tid, "Test Div 23", "Test Div 43", "Test Div 10");

    let tour_3 = &tournaments[2];
    let tour_3_divisions = seed_divisions_with_names(db, tour_3.tid, "Test Div 9", "Test Div 2", "Test Div 7");

    let mut return_vec = tour_1_divisions;
    return_vec.extend(tour_2_divisions);
    return_vec.extend(tour_3_divisions);

    tour_3.clone()
}

pub fn seed_get_rooms_by_tournament(db: &mut database::Connection) -> Tournament {
    let tournaments = seed_tournaments_with_names(db, "T1", "T2", "T3");  // unique names not really needed (realizing after the fact)

    let tour_1 = &tournaments[0];
    let tour_1_rooms = seed_rooms_with_names(db, tour_1.tid, "Test Room 3276", "Test Room 9078", "Test Room 4611");

    let tour_2 = &tournaments[1];
    let tour_2_rooms = seed_rooms_with_names(db, tour_2.tid, "Test Room 23", "Test Room 43", "Test Room 10");

    let tour_3 = &tournaments[2];
    let tour_3_rooms = seed_rooms_with_names(db, tour_3.tid, "Test Room 9", "Test Room 2", "Test Room 7");

    let mut return_vec = tour_1_rooms;
    return_vec.extend(tour_2_rooms);
    return_vec.extend(tour_3_rooms);

    tour_3.clone()
}

pub fn seed_get_rounds_by_tournament(db: &mut database::Connection) -> Tournament {
    let tournaments = seed_tournaments_with_names(db, "T1", "T2", "T3");
    let tour_1 = &tournaments[0];
    let tour_1_div = seed_division_with_name(db, tour_1.tid, "Test Div 3276");
    let start_time_1 = Utc.with_ymd_and_hms(2055, 5, 23, 00, 00, 0).unwrap();
    let start_time_2 = Utc.with_ymd_and_hms(2056, 5, 23, 00, 00, 0).unwrap();
    let start_time_3 = Utc.with_ymd_and_hms(2057, 5, 23, 00, 00, 0).unwrap();
    seed_rounds_with_sched_start_times(db, tour_1_div.did, start_time_1, start_time_2, start_time_3);

    let tour_2 = &tournaments[1];
    let tour_2_div_1 = seed_division_with_name(db, tour_2.tid, "Test Div 9078");
    let start_time_4 = Utc.with_ymd_and_hms(2058, 5, 23, 00, 00, 0).unwrap();
    let start_time_5 = Utc.with_ymd_and_hms(2059, 5, 23, 00, 00, 0).unwrap();
    let start_time_6 = Utc.with_ymd_and_hms(2060, 5, 23, 00, 00, 0).unwrap();
    seed_rounds_with_sched_start_times(db, tour_2_div_1.did, start_time_4, start_time_5, start_time_6);

    let tour_2_div_2 = seed_division_with_name(db, tour_2.tid, "Test Div 9079");
    let start_time_7 = Utc.with_ymd_and_hms(2061, 5, 23, 00, 00, 0).unwrap();
    let start_time_8 = Utc.with_ymd_and_hms(2062, 5, 23, 00, 00, 0).unwrap();
    let start_time_9 = Utc.with_ymd_and_hms(2063, 5, 23, 00, 00, 0).unwrap();
    seed_rounds_with_sched_start_times(db, tour_2_div_2.did, start_time_7, start_time_8, start_time_9);

    tour_2.clone()
}

pub fn arrange_get_all_tournamentgroups_of_tournament_works_integration_test(db: &mut database::Connection) -> (Tournament, TournamentGroup, TournamentGroup) {
    let tour_1 = TournamentBuilder::new_default("Tour 1")
        .build_and_insert(db)
        .unwrap();

    let tour_2 = TournamentBuilder::new_default("Tour 2")
        .build_and_insert(db)
        .unwrap();

    let tg_1 = TournamentGroupBuilder::new_default("Test TourGroup 1")
        .set_description(Some("This is TourGroup 1 testing.".to_string()))
        .build_and_insert(db)
        .unwrap();
    
    let tg_2 = TournamentGroupBuilder::new_default("Test TourGroup 2")
        .set_description(Some("This is TourGroup 2 testing.".to_string()))
        .build_and_insert(db)
        .unwrap();

    let tg_3 = TournamentGroupBuilder::new_default("Test TourGroup 3")
        .set_description(Some("This is TourGroup 3 testing.".to_string()))
        .build_and_insert(db)
        .unwrap();
    
    let tg_4 = TournamentGroupBuilder::new_default("Test TourGroup 4")
        .set_description(Some("This is TourGroup 4 testing.".to_string()))
        .build_and_insert(db)
        .unwrap();

    let tg_1_bridge_tour_1 = TournamentGroupTournamentBuilder::new_default(tg_1.tgid, tour_1.tid)
        .build_and_insert(db)
        .unwrap();
    let tg_2_bridge_tour_1 = TournamentGroupTournamentBuilder::new_default(tg_2.tgid, tour_1.tid)
        .build_and_insert(db)
        .unwrap();
    let tg_3_bridge_tour_2 = TournamentGroupTournamentBuilder::new_default(tg_3.tgid, tour_2.tid)
        .build_and_insert(db)
        .unwrap();
    let tg_4_bridge_tour_2 = TournamentGroupTournamentBuilder::new_default(tg_4.tgid, tour_2.tid)
        .build_and_insert(db)
        .unwrap();

    (tour_1, tg_1, tg_2)
}

pub fn arrange_get_all_equipmentregistrations_of_tournament_works_integration_test(db: &mut database::Connection) 
    -> (Tournament,EquipmentRegistration,EquipmentRegistration,EquipmentRegistration,EquipmentRegistration,
        EquipmentRegistration,EquipmentRegistration,EquipmentRegistration,EquipmentRegistration) {
    seed_1_equipmentregistration_for_each_equipment_type_with_minimum_required_dependencies(db)
}
