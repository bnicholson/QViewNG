use backend::{database, models::{extensioncord::{ExtensionCord, ExtensionCordBuilder, NewExtensionCord}, equipmentset::EquipmentSetBuilder, user::UserBuilder}};


pub fn arrange_create_works_integration_test(db: &mut database::Connection) -> NewExtensionCord {
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
    ExtensionCordBuilder::new_default(equipment_set.id)
        .set_length(Some("10-foot".to_string()))
        .set_misc_note(Some("This is a test extensioncord.".to_string()))
        .build()
        .unwrap()
}

pub fn arrange_get_all_works_integration_test(db: &mut database::Connection) -> (ExtensionCord, ExtensionCord) {
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
    let extensioncord_1 = ExtensionCordBuilder::new_default(equipment_set.id)
        .set_length(Some("8-foot".to_string()))
        .set_misc_note(Some("First test extensioncord.".to_string()))
        .build_and_insert(db)
        .unwrap();
    let extensioncord_2 = ExtensionCordBuilder::new_default(equipment_set.id)
        .set_length(Some("10-foot".to_string()))
        .set_misc_note(Some("Second test extensioncord.".to_string()))
        .build_and_insert(db)
        .unwrap();
    (extensioncord_1, extensioncord_2)
}

pub fn arrange_get_extensioncord_by_id_works_integration_test(db: &mut database::Connection) -> ExtensionCord {
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
    ExtensionCordBuilder::new_default(equipment_set.id)
        .set_length(Some("8-foot".to_string()))
        .set_misc_note(Some("Test extensioncord for get by ID.".to_string()))
        .build_and_insert(db)
        .unwrap();
    ExtensionCordBuilder::new_default(equipment_set.id)
        .set_length(Some("10-foot".to_string()))
        .set_misc_note(Some("Test extensioncord for get by ID.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_update_works_integration_test(db: &mut database::Connection) -> ExtensionCord {
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
    ExtensionCordBuilder::new_default(equipment_set.id)
        .set_length(Some("10-foot".to_string()))
        .set_misc_note(Some("Test extensioncord for update.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

// pub fn arrange_delete_works_integration_test(db: &mut database::Connection) -> ExtensionCord {
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
//     ExtensionCordBuilder::new_default(equipment_set.id)
//         .set_length(Some("10-foot".to_string()))
//         .set_model(Some("6-port".to_string()))
//         .set_num_of_plugs(Some(6))
//         .set_misc_note(Some("Test extensioncord for delete.".to_string()))
//         .build_and_insert(db)
//         .unwrap()
// }
