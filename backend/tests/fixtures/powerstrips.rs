use backend::{database, models::{powerstrip::{PowerStrip, PowerStripBuilder, NewPowerStrip}, equipmentset::EquipmentSetBuilder, user::UserBuilder}};


pub fn arrange_create_works_integration_test(db: &mut database::Connection) -> NewPowerStrip {
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
    PowerStripBuilder::new_default(equipment_set.id)
        .set_make(Some("Amazon".to_string()))
        .set_model(Some("6-port".to_string()))
        .set_num_of_plugs(Some(6))
        .set_misc_note(Some("This is a test powerstrip.".to_string()))
        .build()
        .unwrap()
}

pub fn arrange_get_all_works_integration_test(db: &mut database::Connection) -> (PowerStrip, PowerStrip) {
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
    let powerstrip_1 = PowerStripBuilder::new_default(equipment_set.id)
        .set_make(Some("BestBuy".to_string()))
        .set_model(Some("8-port".to_string()))
        .set_num_of_plugs(Some(8))
        .set_misc_note(Some("First test powerstrip.".to_string()))
        .build_and_insert(db)
        .unwrap();
    let powerstrip_2 = PowerStripBuilder::new_default(equipment_set.id)
        .set_make(Some("Amazon".to_string()))
        .set_model(Some("6-port".to_string()))
        .set_num_of_plugs(Some(6))
        .set_misc_note(Some("Second test powerstrip.".to_string()))
        .build_and_insert(db)
        .unwrap();
    (powerstrip_1, powerstrip_2)
}

pub fn arrange_get_powerstrip_by_id_works_integration_test(db: &mut database::Connection) -> PowerStrip {
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
    PowerStripBuilder::new_default(equipment_set.id)
        .set_make(Some("BestBuy".to_string()))
        .set_model(Some("8-port".to_string()))
        .set_num_of_plugs(Some(8))
        .set_misc_note(Some("Test powerstrip for get by ID.".to_string()))
        .build_and_insert(db)
        .unwrap();
    PowerStripBuilder::new_default(equipment_set.id)
        .set_make(Some("Amazon".to_string()))
        .set_model(Some("6-port".to_string()))
        .set_num_of_plugs(Some(6))
        .set_misc_note(Some("Test powerstrip for get by ID.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_update_works_integration_test(db: &mut database::Connection) -> PowerStrip {
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
    PowerStripBuilder::new_default(equipment_set.id)
        .set_make(Some("Amazon".to_string()))
        .set_model(Some("6-port".to_string()))
        .set_num_of_plugs(Some(6))
        .set_misc_note(Some("Test powerstrip for update.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_delete_works_integration_test(db: &mut database::Connection) -> PowerStrip {
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
    PowerStripBuilder::new_default(equipment_set.id)
        .set_make(Some("Amazon".to_string()))
        .set_model(Some("6-port".to_string()))
        .set_num_of_plugs(Some(6))
        .set_misc_note(Some("Test powerstrip for delete.".to_string()))
        .build_and_insert(db)
        .unwrap()
}
