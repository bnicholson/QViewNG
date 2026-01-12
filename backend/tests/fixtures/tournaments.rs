// In tests/common/mod.rs or tests/fixtures.rs
use diesel::prelude::*;
use backend::models::tournament::Tournament;
use chrono::NaiveDate;

pub fn create_tournament(conn: &mut PgConnection, name: &str) -> Tournament {
    use backend::schema::tournaments;
    
    diesel::insert_into(tournaments::table)
        .values(
            (
                tournaments::organization.eq("Nazarene"),
                tournaments::tname.eq(name),
                tournaments::breadcrumb.eq("/test/bread/crumb"),
                tournaments::fromdate.eq(NaiveDate::from_ymd_opt(2025, 5, 23).unwrap()),
                tournaments::todate.eq(NaiveDate::from_ymd_opt(2025, 5, 27).unwrap()),
                tournaments::venue.eq("Olivet Nazarene University"),
                tournaments::city.eq("Bourbonnais"),
                tournaments::region.eq("Central USA"),
                tournaments::country.eq("USA"),
                tournaments::contact.eq("Jason Morton"),
                tournaments::contactemail.eq("jasonmorton@fakeemail.com"),
                tournaments::is_public.eq(false),
                tournaments::shortinfo.eq("This is your captain speaking."),
            )
        )
        .returning(Tournament::as_returning())
        .get_result::<Tournament>(conn)
        .expect("Failed to create tournament")
}

pub fn seed_tournaments(conn: &mut PgConnection) -> Vec<Tournament> {
    vec![
        create_tournament(conn, "Q2025"),
        create_tournament(conn, "Tour 2"),
        create_tournament(conn, "Tour 3"),
    ]
}