use backend::{database, models::{computer::{ComputerBuilder, NewComputer}, equipmentset::EquipmentSetBuilder, user::UserBuilder}};


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
