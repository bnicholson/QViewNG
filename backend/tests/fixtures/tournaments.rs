// In tests/common/mod.rs or tests/fixtures.rs
use diesel::prelude::*;
use backend::models::tournament;
use chrono::NaiveDate;

pub fn new_tournament_one() -> tournament::NewTournament {
    tournament::NewTournament {
        organization: "Nazarene".to_string(),
        tname: "Test Post".to_string(),
        breadcrumb: "/test/post".to_string(),
        fromdate: NaiveDate::from_ymd_opt(2025, 5, 23).unwrap(),
        todate: NaiveDate::from_ymd_opt(2025, 5, 27).unwrap(),
        venue: "Vancouver Universtiy".to_string(),
        city: "Vancouver".to_string(),
        region: "North America".to_string(),
        country: "Canada".to_string(),
        contact: "primemin".to_string(),
        contactemail: "primemin@fakeemail.com".to_string(),
        shortinfo: "Winter Olympics".to_string(),
        info: "Shawn White did excellent in the halfpipe.".to_string()
    }
}

pub fn get_tournament_payload() -> tournament::NewTournament {
    new_tournament_one()
}

fn new_tournament_two(name: &str) -> tournament::NewTournament {
    tournament::NewTournament {
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

fn create_and_insert_tournament(conn: &mut PgConnection, name: &str) -> tournament::Tournament {
    use backend::schema::tournaments;
    
    diesel::insert_into(tournaments::table)
        .values(new_tournament_two(name))
        .returning(tournament::Tournament::as_returning())
        .get_result::<tournament::Tournament>(conn)
        .expect("Failed to create tournament")
}

pub fn seed_tournaments(conn: &mut PgConnection) -> Vec<tournament::Tournament> {
    vec![
        create_and_insert_tournament(conn, "Q2025"),
        create_and_insert_tournament(conn, "Tour 2"),
        create_and_insert_tournament(conn, "Tour 3"),
    ]
}