use backend::{database, models::{jumppad::{JumpPad, JumpPadBuilder, NewJumpPad}, equipmentset::EquipmentSetBuilder, user::UserBuilder}};


pub fn arrange_create_works_integration_test(db: &mut database::Connection) -> NewJumpPad {
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
    JumpPadBuilder::new_default(equipment_set.id)
        .set_color(Some("red".to_string()))
        .set_misc_note(Some("This is a test jumppad.".to_string()))
        .build()
        .unwrap()
}

pub fn arrange_get_all_works_integration_test(db: &mut database::Connection) -> (JumpPad, JumpPad) {
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
    let jumppad_1 = JumpPadBuilder::new_default(equipment_set.id)
        .set_color(Some("red".to_string()))
        .set_misc_note(Some("First test jumppad.".to_string()))
        .build_and_insert(db)
        .unwrap();
    let jumppad_2 = JumpPadBuilder::new_default(equipment_set.id)
        .set_color(Some("green B".to_string()))
        .set_misc_note(Some("Second test jumppad.".to_string()))
        .build_and_insert(db)
        .unwrap();
    (jumppad_1, jumppad_2)
}

// pub fn arrange_get_jumppad_by_id_works_integration_test(db: &mut database::Connection) -> JumpPad {
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
//     JumpPadBuilder::new_default(equipment_set.id)
//         .set_color(Some("Brand Y".to_string()))
//         .set_operating_system(Some("OS Y".to_string()))
//         .set_misc_note(Some("Test jumppad for get by ID.".to_string()))
//         .build_and_insert(db)
//         .unwrap();
//     JumpPadBuilder::new_default(equipment_set.id)
//         .set_color(Some("Brand X".to_string()))
//         .set_operating_system(Some("OS X".to_string()))
//         .set_misc_note(Some("Test jumppad for get by ID.".to_string()))
//         .build_and_insert(db)
//         .unwrap()
// }

// pub fn arrange_update_works_integration_test(db: &mut database::Connection) -> JumpPad {
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
//     JumpPadBuilder::new_default(equipment_set.id)
//         .set_color(Some("Brand Z".to_string()))
//         .set_operating_system(Some("OS Z".to_string()))
//         .set_misc_note(Some("Test jumppad for update.".to_string()))
//         .build_and_insert(db)
//         .unwrap()
// }

// pub fn arrange_delete_works_integration_test(db: &mut database::Connection) -> JumpPad {
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
//     JumpPadBuilder::new_default(equipment_set.id)
//         .set_color(Some("Brand Delete".to_string()))
//         .set_operating_system(Some("OS Delete".to_string()))
//         .set_misc_note(Some("Test jumppad for delete.".to_string()))
//         .build_and_insert(db)
//         .unwrap()
// }
