use backend::{database, models::{computer::{Computer, ComputerBuilder, NewComputer}, equipmentset::EquipmentSetBuilder, user::UserBuilder}};


pub fn arrange_create_works_integration_test(db: &mut database::Connection) -> NewComputer {
    let user = UserBuilder::new_default("User 1")
        .set_hash_password("SOmeTHinGSeCUre!23")
        .build_and_insert(db)
        .unwrap();
    // let equipment_set = EquipmentSetBuilder::new_default(user.id)
    //     .set_is_active(true)
    //     .set_is_default(true)
    //     .set_description(Some("This is a test equipment set.".to_string()))
    //     .build_and_insert(db)
    //     .unwrap();
    ComputerBuilder::new_default()
        .set_brand(Some("Test Brand".to_string()))
        .set_operating_system(Some("Test OS".to_string()))
        // .set_misc_note(Some("This is a test computer.".to_string()))
        .build()
        .unwrap()
}

pub fn arrange_get_all_works_integration_test(db: &mut database::Connection) -> (Computer, Computer) {
    let user = UserBuilder::new_default("User 1")
        .set_hash_password("SOmeTHinGSeCUre!23")
        .build_and_insert(db)
        .unwrap();
    // let equipment_set = EquipmentSetBuilder::new_default(user.id)
    //     .set_is_active(true)
    //     .set_is_default(true)
    //     .set_description(Some("This is a test equipment set.".to_string()))
    //     .build_and_insert(db)
    //     .unwrap();
    let computer_1 = ComputerBuilder::new_default()
        .set_brand(Some("Brand A".to_string()))
        .set_operating_system(Some("OS A".to_string()))
        // .set_misc_note(Some("First test computer.".to_string()))
        .build_and_insert(db)
        .unwrap();
    let computer_2 = ComputerBuilder::new_default()
        .set_brand(Some("Brand B".to_string()))
        .set_operating_system(Some("OS B".to_string()))
        // .set_misc_note(Some("Second test computer.".to_string()))
        .build_and_insert(db)
        .unwrap();
    (computer_1, computer_2)
}

pub fn arrange_get_computer_by_id_works_integration_test(db: &mut database::Connection) -> Computer {
    let user = UserBuilder::new_default("User 1")
        .set_hash_password("SOmeTHinGSeCUre!23")
        .build_and_insert(db)
        .unwrap();
    // let equipment_set = EquipmentSetBuilder::new_default(user.id)
    //     .set_is_active(true)
    //     .set_is_default(true)
    //     .set_description(Some("This is a test equipment set.".to_string()))
    //     .build_and_insert(db)
    //     .unwrap();
    ComputerBuilder::new_default()
        .set_brand(Some("Brand Y".to_string()))
        .set_operating_system(Some("OS Y".to_string()))
        // .set_misc_note(Some("Test computer for get by ID.".to_string()))
        .build_and_insert(db)
        .unwrap();
    ComputerBuilder::new_default()
        .set_brand(Some("Brand X".to_string()))
        .set_operating_system(Some("OS X".to_string()))
        // .set_misc_note(Some("Test computer for get by ID.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_update_works_integration_test(db: &mut database::Connection) -> Computer {
    let user = UserBuilder::new_default("User 1")
        .set_hash_password("SOmeTHinGSeCUre!23")
        .build_and_insert(db)
        .unwrap();
    // let equipment_set = EquipmentSetBuilder::new_default(user.id)
    //     .set_is_active(true)
    //     .set_is_default(true)
    //     .set_description(Some("This is a test equipment set.".to_string()))
    //     .build_and_insert(db)
    //     .unwrap();
    ComputerBuilder::new_default()
        .set_brand(Some("Brand Z".to_string()))
        .set_operating_system(Some("OS Z".to_string()))
        // .set_misc_note(Some("Test computer for update.".to_string()))
        .build_and_insert(db)
        .unwrap()
}
