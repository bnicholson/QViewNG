use backend::{database, models::{interfacebox::{InterfaceBox, InterfaceBoxBuilder, NewInterfaceBox}, equipmentset::EquipmentSetBuilder, user::UserBuilder}};


pub fn arrange_create_works_integration_test(db: &mut database::Connection) -> NewInterfaceBox {
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
    InterfaceBoxBuilder::new_default(equipment_set.id)
        .set_type_(Some("USB".to_string()))
        .set_serial_number(Some("lkjhaluifhoiun".to_string()))
        .set_misc_note(Some("This is a test interfacebox.".to_string()))
        .build()
        .unwrap()
}

pub fn arrange_get_all_works_integration_test(db: &mut database::Connection) -> (InterfaceBox, InterfaceBox) {
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
    let interfacebox_1 = InterfaceBoxBuilder::new_default(equipment_set.id)
        .set_type_(Some("USB".to_string()))
        .set_serial_number(Some("lkjhaluifhoiun".to_string()))
        .set_misc_note(Some("First test interfacebox.".to_string()))
        .build_and_insert(db)
        .unwrap();
    let interfacebox_2 = InterfaceBoxBuilder::new_default(equipment_set.id)
        .set_type_(Some("USB".to_string()))
        .set_serial_number(Some("hloiununun".to_string()))
        .set_misc_note(Some("Second test interfacebox.".to_string()))
        .build_and_insert(db)
        .unwrap();
    (interfacebox_1, interfacebox_2)
}

// pub fn arrange_get_interfacebox_by_id_works_integration_test(db: &mut database::Connection) -> InterfaceBox {
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
//     InterfaceBoxBuilder::new_default(equipment_set.id)
//         .set_serial_number(Some("hloiununun".to_string()))
//         .set_misc_note(Some("Test interfacebox for get by ID.".to_string()))
//         .build_and_insert(db)
//         .unwrap();
//     InterfaceBoxBuilder::new_default(equipment_set.id)
        // .set_type_(Some("USB".to_string()))
//         .set_serial_number(Some("iununuipiajsnkjsd".to_string()))
//         .set_misc_note(Some("Test interfacebox for get by ID.".to_string()))
//         .build_and_insert(db)
//         .unwrap()
// }

// pub fn arrange_update_works_integration_test(db: &mut database::Connection) -> InterfaceBox {
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
//     InterfaceBoxBuilder::new_default(equipment_set.id)
        // .set_type_(Some("USB".to_string()))
//         .set_serial_number(Some("hloiununun".to_string()))
//         .set_misc_note(Some("Test interfacebox for update.".to_string()))
//         .build_and_insert(db)
//         .unwrap()
// }

// pub fn arrange_delete_works_integration_test(db: &mut database::Connection) -> InterfaceBox {
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
//     InterfaceBoxBuilder::new_default(equipment_set.id)
        // .set_type_(Some("USB".to_string()))
//         .set_serial_number(Some("hloiununun".to_string()))
//         .set_misc_note(Some("Test interfacebox for delete.".to_string()))
//         .build_and_insert(db)
//         .unwrap()
// }
