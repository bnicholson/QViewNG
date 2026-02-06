use backend::{database, models::{projector::{Projector, ProjectorBuilder, NewProjector}, equipmentset::EquipmentSetBuilder, user::UserBuilder}};


pub fn arrange_create_works_integration_test(db: &mut database::Connection) -> NewProjector {
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
    ProjectorBuilder::new_default(equipment_set.id)
        .set_brand(Some("CoolFlik".to_string()))
        .set_misc_note(Some("This is a test projector.".to_string()))
        .build()
        .unwrap()
}

pub fn arrange_get_all_works_integration_test(db: &mut database::Connection) -> (Projector, Projector) {
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
    let projector_1 = ProjectorBuilder::new_default(equipment_set.id)
        .set_brand(Some("CoolFlik".to_string()))
        .set_misc_note(Some("First test projector.".to_string()))
        .build_and_insert(db)
        .unwrap();
    let projector_2 = ProjectorBuilder::new_default(equipment_set.id)
        .set_brand(Some("ResMaster".to_string()))
        .set_misc_note(Some("Second test projector.".to_string()))
        .build_and_insert(db)
        .unwrap();
    (projector_1, projector_2)
}

pub fn arrange_get_projector_by_id_works_integration_test(db: &mut database::Connection) -> Projector {
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
    ProjectorBuilder::new_default(equipment_set.id)
        .set_brand(Some("CoolFlik".to_string()))
        .set_misc_note(Some("Test projector for get by ID.".to_string()))
        .build_and_insert(db)
        .unwrap();
    ProjectorBuilder::new_default(equipment_set.id)
        .set_brand(Some("ResMaster".to_string()))
        .set_misc_note(Some("Test projector for get by ID.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_update_works_integration_test(db: &mut database::Connection) -> Projector {
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
    ProjectorBuilder::new_default(equipment_set.id)
        .set_brand(Some("CoolFlik".to_string()))
        .set_misc_note(Some("Test projector for update.".to_string()))
        .build_and_insert(db)
        .unwrap()
}

// pub fn arrange_delete_works_integration_test(db: &mut database::Connection) -> Projector {
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
//     ProjectorBuilder::new_default(equipment_set.id)
//         .set_brand(Some("CoolFlik".to_string()))
//         .set_misc_note(Some("Test projector for delete.".to_string()))
//         .build_and_insert(db)
//         .unwrap()
// }
