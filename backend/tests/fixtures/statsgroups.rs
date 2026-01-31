use backend::{database, models::statsgroup::{NewStatsGroup, StatsGroup, StatsGroupBuilder}};

pub fn arrange_create_works_integration_test() -> NewStatsGroup {
    StatsGroupBuilder::new_default("Test StatsGroup 2217")
        .set_description(Some("StatsGroup for integration test create.".to_string()))
        .build()
        .unwrap()
}

pub fn arrange_get_all_works_integration_test(db: &mut database::Connection) -> (StatsGroup, StatsGroup) {
    (
        StatsGroupBuilder::new_default("Test StatsGroup 1")
            .set_description(Some("This is StatsGroup 1's description.".to_string()))
            .build_and_insert(db)
            .unwrap(),
        StatsGroupBuilder::new_default("Test StatsGroup 2")
            .set_description(Some("This is StatsGroup 2's description.".to_string()))
            .build_and_insert(db)
            .unwrap()
    )
}

pub fn arrange_get_statsgroup_by_id_integration_test(db: &mut database::Connection) -> StatsGroup {
    StatsGroupBuilder::new_default("Test StatsGroup 1")
        .set_description(Some("This is StatsGroup 1's description.".to_string()))
        .build_and_insert(db)
        .unwrap();
    StatsGroupBuilder::new_default("Test StatsGroup 2")
        .set_description(Some("This is StatsGroup 2's description.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_update_works_integration_test(db: &mut database::Connection) -> StatsGroup {
    StatsGroupBuilder::new_default("Test StatsGroup 1")
        .set_description(Some("StatsGroup 1 testing update.".to_string()))
        .build_and_insert(db)
        .unwrap()
}
