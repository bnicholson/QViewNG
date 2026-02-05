use backend::{database, models::{computer::ComputerBuilder, equipment_dbo::{EquipmentDbo, EquipmentDboBuilder, NewEquipmentDbo}, equipmentset::EquipmentSetBuilder, user::UserBuilder}};


pub fn arrange_create_works_integration_test(db: &mut database::Connection) -> NewEquipmentDbo {
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
    let computer = ComputerBuilder::new_default()
        .build_and_insert(db)
        .unwrap();
    EquipmentDboBuilder::new_default()
        .set_computerid(Some(computer.computerid))
        .set_misc_note(Some("Test note 9909".to_string()))
        .set_equipmentsetid(Some(equipment_set.id))
        .build()
        .unwrap()
}

pub fn arrange_get_all_works_integration_test(db: &mut database::Connection) -> (EquipmentDbo, EquipmentDbo) {
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
    let computer_1 = ComputerBuilder::new_default()
        .build_and_insert(db)
        .unwrap();
    let computer_2 = ComputerBuilder::new_default()
        .build_and_insert(db)
        .unwrap();
    (
        EquipmentDboBuilder::new_default()
            .set_computerid(Some(computer_1.computerid))
            .set_misc_note(Some("Test note 9909".to_string()))
            .set_equipmentsetid(Some(equipment_set.id))
            .build_and_insert(db)
            .unwrap(),
        EquipmentDboBuilder::new_default()
            .set_computerid(Some(computer_2.computerid))
            .set_misc_note(Some("Test note 9900".to_string()))
            .set_equipmentsetid(Some(equipment_set.id))
            .build_and_insert(db)
            .unwrap()
    )
}

pub fn arrange_get_equipment_by_id_works_integration_test(db: &mut database::Connection) -> EquipmentDbo {
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
    let computer_1 = ComputerBuilder::new_default()
        .build_and_insert(db)
        .unwrap();
    let computer_2 = ComputerBuilder::new_default()
        .build_and_insert(db)
        .unwrap();
    EquipmentDboBuilder::new_default()
            .set_computerid(Some(computer_1.computerid))
            .set_misc_note(Some("Test note 9909".to_string()))
            .set_equipmentsetid(Some(equipment_set.id))
            .build_and_insert(db)
            .unwrap();
    EquipmentDboBuilder::new_default()
        .set_computerid(Some(computer_2.computerid))
        .set_misc_note(Some("Test note 9900".to_string()))
        .set_equipmentsetid(Some(equipment_set.id))
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_update_works_integration_test(db: &mut database::Connection) -> EquipmentDbo {
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
    let computer = ComputerBuilder::new_default()
        .build_and_insert(db)
        .unwrap();
    EquipmentDboBuilder::new_default()
        .set_computerid(Some(computer.computerid))
        .set_misc_note(Some("Test note 9900".to_string()))
        .set_equipmentsetid(Some(equipment_set.id))
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_delete_works_integration_test(db: &mut database::Connection) -> EquipmentDbo {
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
    let computer = ComputerBuilder::new_default()
        .build_and_insert(db)
        .unwrap();
    EquipmentDboBuilder::new_default()
        .set_computerid(Some(computer.computerid))
        .set_misc_note(Some("Test note 9900".to_string()))
        .set_equipmentsetid(Some(equipment_set.id))
        .build_and_insert(db)
        .unwrap()
}
