use backend::{database, models::{microphonerecorder::{MicrophoneRecorder, MicrophoneRecorderBuilder, NewMicrophoneRecorder}, equipmentset::EquipmentSetBuilder, user::UserBuilder}};


pub fn arrange_create_works_integration_test(db: &mut database::Connection) -> NewMicrophoneRecorder {
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
    MicrophoneRecorderBuilder::new_default(equipment_set.id)
        .set_type_(Some("External".to_string()))
        .set_misc_note(Some("This is a test microphonerecorder.".to_string()))
        .build()
        .unwrap()
}

pub fn arrange_get_all_works_integration_test(db: &mut database::Connection) -> (MicrophoneRecorder, MicrophoneRecorder) {
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
    let microphonerecorder_1 = MicrophoneRecorderBuilder::new_default(equipment_set.id)
        .set_type_(Some("External".to_string()))
        .set_misc_note(Some("First test microphonerecorder.".to_string()))
        .build_and_insert(db)
        .unwrap();
    let microphonerecorder_2 = MicrophoneRecorderBuilder::new_default(equipment_set.id)
        .set_type_(Some("Built-in".to_string()))
        .set_misc_note(Some("Second test microphonerecorder.".to_string()))
        .build_and_insert(db)
        .unwrap();
    (microphonerecorder_1, microphonerecorder_2)
}

pub fn arrange_get_microphonerecorder_by_id_works_integration_test(db: &mut database::Connection) -> MicrophoneRecorder {
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
    MicrophoneRecorderBuilder::new_default(equipment_set.id)
        .set_type_(Some("External".to_string()))
        .set_misc_note(Some("Test microphonerecorder for get by ID.".to_string()))
        .build_and_insert(db)
        .unwrap();
    MicrophoneRecorderBuilder::new_default(equipment_set.id)
        .set_type_(Some("Built-in".to_string()))
        .set_misc_note(Some("Test microphonerecorder for get by ID.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_update_works_integration_test(db: &mut database::Connection) -> MicrophoneRecorder {
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
    MicrophoneRecorderBuilder::new_default(equipment_set.id)
        .set_type_(Some("External".to_string()))
        .set_misc_note(Some("Test microphonerecorder for update.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

// pub fn arrange_delete_works_integration_test(db: &mut database::Connection) -> MicrophoneRecorder {
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
//     MicrophoneRecorderBuilder::new_default(equipment_set.id)
//         .set_type_(Some("External".to_string()))
//         .set_brand(Some("Brand H".to_string()))
//         .set_misc_note(Some("Test microphonerecorder for delete.".to_string()))
//         .build_and_insert(db)
//         .unwrap()
// }
