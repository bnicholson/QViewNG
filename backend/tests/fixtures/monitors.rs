use backend::{database, models::{monitor::{Monitor, MonitorBuilder, NewMonitor}, equipmentset::EquipmentSetBuilder, user::UserBuilder}};


pub fn arrange_create_works_integration_test(db: &mut database::Connection) -> NewMonitor {
    let user = UserBuilder::new_default("User 1")
        .set_hash_password("SOmeTHinGSeCUre!23")
        .build_and_insert(db)
        .unwrap();
    let equipment_set = EquipmentSetBuilder::new_default(user.id)
        .set_is_active(true)
        .set_is_default(true)
        .set_description(Some("This is a test equipment set.".to_string()))
        .build_and_insert(db)
        .unwrap();
    MonitorBuilder::new_default(equipment_set.id)
        .set_size(Some("17 inches".to_string()))
        .set_brand(Some("Brand H".to_string()))
        .set_misc_note(Some("This is a test monitor.".to_string()))
        .build()
        .unwrap()
}

pub fn arrange_get_all_works_integration_test(db: &mut database::Connection) -> (Monitor, Monitor) {
    let user = UserBuilder::new_default("User 1")
        .set_hash_password("SOmeTHinGSeCUre!23")
        .build_and_insert(db)
        .unwrap();
    let equipment_set = EquipmentSetBuilder::new_default(user.id)
        .set_is_active(true)
        .set_is_default(true)
        .set_description(Some("This is a test equipment set.".to_string()))
        .build_and_insert(db)
        .unwrap();
    let monitor_1 = MonitorBuilder::new_default(equipment_set.id)
        .set_size(Some("17 inches".to_string()))
        .set_brand(Some("Brand H".to_string()))
        .set_misc_note(Some("First test monitor.".to_string()))
        .build_and_insert(db)
        .unwrap();
    let monitor_2 = MonitorBuilder::new_default(equipment_set.id)
        .set_size(Some("17 inches".to_string()))
        .set_brand(Some("Brand L".to_string()))
        .set_misc_note(Some("Second test monitor.".to_string()))
        .build_and_insert(db)
        .unwrap();
    (monitor_1, monitor_2)
}

pub fn arrange_get_monitor_by_id_works_integration_test(db: &mut database::Connection) -> Monitor {
    let user = UserBuilder::new_default("User 1")
        .set_hash_password("SOmeTHinGSeCUre!23")
        .build_and_insert(db)
        .unwrap();
    let equipment_set = EquipmentSetBuilder::new_default(user.id)
        .set_is_active(true)
        .set_is_default(true)
        .set_description(Some("This is a test equipment set.".to_string()))
        .build_and_insert(db)
        .unwrap();
    MonitorBuilder::new_default(equipment_set.id)
        .set_size(Some("17 inches".to_string()))
        .set_brand(Some("Brand H".to_string()))
        .set_misc_note(Some("Test monitor for get by ID.".to_string()))
        .build_and_insert(db)
        .unwrap();
    MonitorBuilder::new_default(equipment_set.id)
        .set_size(Some("17 inches".to_string()))
        .set_brand(Some("Brand K".to_string()))
        .set_misc_note(Some("Test monitor for get by ID.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

// pub fn arrange_update_works_integration_test(db: &mut database::Connection) -> Monitor {
//     let user = UserBuilder::new_default("User 1")
//         .set_hash_password("SOmeTHinGSeCUre!23")
//         .build_and_insert(db)
//         .unwrap();
//     let equipment_set = EquipmentSetBuilder::new_default(user.id)
//         .set_is_active(true)
//         .set_is_default(true)
//         .set_description(Some("This is a test equipment set.".to_string()))
//         .build_and_insert(db)
//         .unwrap();
//     MonitorBuilder::new_default(equipment_set.id)
        // .set_size(Some("17 inches".to_string()))
        // .set_brand(Some("Brand H".to_string()))
//         .set_misc_note(Some("Test monitor for update.".to_string()))
//         .build_and_insert(db)
//         .unwrap()
// }

// pub fn arrange_delete_works_integration_test(db: &mut database::Connection) -> Monitor {
//     let user = UserBuilder::new_default("User 1")
//         .set_hash_password("SOmeTHinGSeCUre!23")
//         .build_and_insert(db)
//         .unwrap();
//     let equipment_set = EquipmentSetBuilder::new_default(user.id)
//         .set_is_active(true)
//         .set_is_default(true)
//         .set_description(Some("This is a test equipment set.".to_string()))
//         .build_and_insert(db)
//         .unwrap();
//     MonitorBuilder::new_default(equipment_set.id)
        // .set_size(Some("17 inches".to_string()))
        // .set_brand(Some("Brand H".to_string()))
//         .set_misc_note(Some("Test monitor for delete.".to_string()))
//         .build_and_insert(db)
//         .unwrap()
// }
