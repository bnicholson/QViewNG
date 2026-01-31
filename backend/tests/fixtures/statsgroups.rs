use backend::models::statsgroup::{NewStatsGroup, StatsGroupBuilder};


pub fn arrange_create_works_integration_test() -> NewStatsGroup {
    StatsGroupBuilder::new_default("Test StatsGroup 2217")
        .set_description(Some("StatsGroup for integration test create.".to_string()))
        .build()
        .unwrap()
}
