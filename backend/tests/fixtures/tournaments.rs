// In tests/common/mod.rs or tests/fixtures.rs
use diesel::prelude::*;
use backend::models::tournament::{NewTournament, Tournament};
use chrono::{Duration, Local, Months, NaiveDate};
use crate::fixtures::{divisions::seed_divisions_with_names, users::seed_users_with_fnames};

pub fn new_tournament_one() -> NewTournament {
    NewTournament {
        organization: "Nazarene".to_string(),
        tname: "Test Post".to_string(),
        breadcrumb: "/test/post".to_string(),
        fromdate: NaiveDate::from_ymd_opt(2025, 5, 23).unwrap(),
        todate: NaiveDate::from_ymd_opt(2025, 5, 27).unwrap(),
        venue: "Vancouver University".to_string(),
        city: "Vancouver".to_string(),
        region: "North America".to_string(),
        country: "Canada".to_string(),
        contact: "primemin".to_string(),
        contactemail: "primemin@fakeemail.com".to_string(),
        shortinfo: "Winter Olympics".to_string(),
        info: "Shawn White did excellent in the halfpipe.".to_string()
    }
}

pub fn get_tournament_payload() -> NewTournament {
    new_tournament_one()
}

pub fn seed_tournament(conn: &mut PgConnection) -> Tournament {
    create_and_insert_tournament(conn, new_tournament_one())
}

fn new_tournament_two(name: &str) -> NewTournament {
    NewTournament {
        organization: "Nazarene".to_string(),
        tname: name.to_string(),
        breadcrumb: "/test/bread/crumb".to_string(),
        fromdate: NaiveDate::from_ymd_opt(2025, 5, 23).unwrap(),
        todate: NaiveDate::from_ymd_opt(2025, 5, 27).unwrap(),
        venue: "Olivet Nazarene University".to_string(),
        city: "Bourbonnais".to_string(),
        region: "Central USA".to_string(),
        country: "USA".to_string(),
        contact: "Jason Morton".to_string(),
        contactemail: "jasonmorton@fakeemail.com".to_string(),
        shortinfo: "NYI International quiz meet of 2025.".to_string(),
        info: "If I wanted a longer description I would have provided it here.".to_string()
    }
}

fn create_and_insert_tournament(conn: &mut PgConnection, tournament: NewTournament) -> Tournament {
    use backend::schema::tournaments;
    
    diesel::insert_into(tournaments::table)
        .values(tournament)
        .returning(Tournament::as_returning())
        .get_result::<Tournament>(conn)
        .expect("Failed to create tournament")
}

pub fn seed_tournaments_with_names(conn: &mut PgConnection, tname_1: &str, tname_2: &str, tname_3: &str) -> Vec<Tournament> {
    vec![
        create_and_insert_tournament(conn, new_tournament_two(tname_1)),
        create_and_insert_tournament(conn, new_tournament_two(tname_2)),
        create_and_insert_tournament(conn, new_tournament_two(tname_3)),
    ]
}

pub fn seed_tournaments(conn: &mut PgConnection) -> Vec<Tournament> {
    seed_tournaments_with_names(conn, "Q2025", "Tour 2", "Tour 3")
}

pub fn seed_tournaments_for_get_today(conn: &mut PgConnection) -> Vec<Tournament> {
    let today = Local::now().date_naive();
    let two_months_from_today: NaiveDate = today.checked_add_months(Months::new(2)).unwrap();
    let days_10_past: NaiveDate = today - Duration::days(10);
    let days_10_future: NaiveDate = today + Duration::days(10);
    let one_month_before_today: NaiveDate = today.checked_sub_months(Months::new(1)).unwrap();

    vec![
        create_and_insert_tournament(conn, new_tournament_specify_dates("2 months in the future exactly", two_months_from_today, two_months_from_today)),
        create_and_insert_tournament(conn, new_tournament_specify_dates("20 Days, Including Today", days_10_past, days_10_future)),
        create_and_insert_tournament(conn, new_tournament_specify_dates("Today Exactly", today, today)),
        create_and_insert_tournament(conn, new_tournament_specify_dates("1 month ago exactly", one_month_before_today, one_month_before_today)),
    ]
}

pub fn seed_tournaments_for_get_all_in_date_range(conn: &mut PgConnection) -> Vec<Tournament> {
    let today = Local::now().date_naive();
    let days_8_past: NaiveDate = today - Duration::days(8);
    let days_8_future: NaiveDate = today + Duration::days(8);
    let days_12_future: NaiveDate = today + Duration::days(12);
    let two_months_from_today: NaiveDate = today.checked_add_months(Months::new(2)).unwrap();
    let one_month_before_today: NaiveDate = today.checked_sub_months(Months::new(1)).unwrap();

    vec![
        create_and_insert_tournament(conn, new_tournament_specify_dates("2 months in the future exactly", two_months_from_today, two_months_from_today)),
        create_and_insert_tournament(conn, new_tournament_specify_dates("eight days past exactly", days_8_past, days_8_past)),
        create_and_insert_tournament(conn, new_tournament_specify_dates("eight to twelve days future", days_8_future, days_12_future)),
        create_and_insert_tournament(conn, new_tournament_specify_dates("Today Exactly", today, today)),
        create_and_insert_tournament(conn, new_tournament_specify_dates("1 month ago exactly", one_month_before_today, one_month_before_today)),
    ]
}

fn new_tournament_specify_dates(name: &str, from_date: NaiveDate, to_date: NaiveDate) -> NewTournament {
    NewTournament {
        organization: "Nazarene".to_string(),
        tname: name.to_string(),
        breadcrumb: "/test/bread/crumb".to_string(),
        fromdate: from_date,
        todate: to_date,
        venue: "Olivet Nazarene University".to_string(),
        city: "Bourbonnais".to_string(),
        region: "Central USA".to_string(),
        country: "USA".to_string(),
        contact: "Jason Morton".to_string(),
        contactemail: "jasonmorton@fakeemail.com".to_string(),
        shortinfo: "NYI International quiz meet of 2025.".to_string(),
        info: "If I wanted a longer description I would have provided it here.".to_string()
    }
}

pub fn seed_get_divisions_by_tournament(conn: &mut PgConnection) -> Tournament {
    let tournaments = seed_tournaments_with_names(conn, "T1", "T2", "T3");  // unique names not really needed (realizing after the fact)

    let tour_1 = &tournaments[0];
    let tour_1_divisions = seed_divisions_with_names(conn, tour_1.tid, "Test Div 3276", "Test Div 9078", "Test Div 4611");

    let tour_2 = &tournaments[1];
    let tour_2_divisions = seed_divisions_with_names(conn, tour_2.tid, "Test Div 23", "Test Div 43", "Test Div 10");

    let tour_3 = &tournaments[2];
    let tour_3_divisions = seed_divisions_with_names(conn, tour_3.tid, "Test Div 9", "Test Div 2", "Test Div 7");

    let mut return_vec = tour_1_divisions;
    return_vec.extend(tour_2_divisions);
    return_vec.extend(tour_3_divisions);

    tour_3.clone()
}
