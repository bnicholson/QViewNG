use backend::models::roster::{NewRoster, RosterBuilder};


pub fn arrange_create_works_integration_test() -> NewRoster {
    RosterBuilder::new("Test Roster 2317")
        .set_description(Some("Roster for integration test create.".to_string()))
        .build()
        .unwrap()
}