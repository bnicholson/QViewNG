// In tests/common/mod.rs or tests/fixtures.rs
use diesel::prelude::*;
use backend::models::tournament::{self,Tournament,NewTournament};
use chrono::{Duration, Local, Months, NaiveDate};

pub fn new_tournament_one() -> NewTournament {
    NewTournament {
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

pub fn get_tournament_payload() -> NewTournament {
    new_tournament_one()
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

fn create_and_insert_tournament(conn: &mut PgConnection, tournament: NewTournament) -> tournament::Tournament {
    use backend::schema::tournaments;
    
    diesel::insert_into(tournaments::table)
        .values(tournament)
        .returning(tournament::Tournament::as_returning())
        .get_result::<tournament::Tournament>(conn)
        .expect("Failed to create tournament")
}

pub fn seed_tournaments(conn: &mut PgConnection) -> Vec<tournament::Tournament> {
    vec![
        create_and_insert_tournament(conn, new_tournament_two("Q2025")),
        create_and_insert_tournament(conn, new_tournament_two("Tour 2")),
        create_and_insert_tournament(conn, new_tournament_two("Tour 3")),
    ]
}

pub fn seed_tournaments_for_get_today(conn: &mut PgConnection) -> Vec<tournament::Tournament> {
    vec![
        create_and_insert_tournament(conn, new_tournament_three_2_months_in_the_future("Future 2 months")),
        create_and_insert_tournament(conn, new_tournament_seven_date_range_includes_today("20 Days, Including Today")),
        create_and_insert_tournament(conn, new_tournament_six_today_exactly("Today Exactly")),
        create_and_insert_tournament(conn, new_tournament_five_1_month_in_the_past("Past 1 month")),
    ]
}

fn new_tournament_three_2_months_in_the_future(name: &str) -> NewTournament {
    // This is intended to be used by the "get today" endpoint.

    let today = Local::now().date_naive();
    let two_months_from_today: NaiveDate = today.checked_add_months(Months::new(2)).unwrap();

    NewTournament {
        organization: "Nazarene".to_string(),
        tname: name.to_string(),
        breadcrumb: "/test/bread/crumb".to_string(),
        fromdate: two_months_from_today,
        todate: two_months_from_today,
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

fn new_tournament_four_27_days_in_the_future(name: &str) -> NewTournament {

    let today = Local::now().date_naive();
    // let one_month_from_today: NaiveDate = today.checked_add_months(Months::new(1)).unwrap();
    let eq_27_days_in_the_future: NaiveDate = today + Duration::days(27);

    NewTournament {
        organization: "Nazarene".to_string(),
        tname: name.to_string(),
        breadcrumb: "/test/bread/crumb".to_string(),
        fromdate: eq_27_days_in_the_future,
        todate: eq_27_days_in_the_future,
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

fn new_tournament_five_1_month_in_the_past(name: &str) -> NewTournament {
    // This is intended to be used by the "get today" endpoint.

    let today = Local::now().date_naive();
    let one_month_before_today: NaiveDate = today.checked_sub_months(Months::new(1)).unwrap();
    
    NewTournament {
        organization: "Nazarene".to_string(),
        tname: name.to_string(),
        breadcrumb: "/test/bread/crumb".to_string(),
        fromdate: one_month_before_today,
        todate: one_month_before_today,
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

fn new_tournament_six_today_exactly(name: &str) -> NewTournament {
    // This is intended to be used by the "get today" endpoint.

    let today = Local::now().date_naive();
    
    NewTournament {
        organization: "Nazarene".to_string(),
        tname: name.to_string(),
        breadcrumb: "/test/bread/crumb".to_string(),
        fromdate: today,
        todate: today,
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

fn new_tournament_seven_date_range_includes_today(name: &str) -> NewTournament {
    // This is intended to be used by the "get today" endpoint.

    let today = Local::now().date_naive();
    let days_10_past: NaiveDate = today - Duration::days(10);
    let days_10_future: NaiveDate = today + Duration::days(10);
    
    NewTournament {
        organization: "Nazarene".to_string(),
        tname: name.to_string(),
        breadcrumb: "/test/bread/crumb".to_string(),
        fromdate: days_10_past,
        todate: days_10_future,
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
