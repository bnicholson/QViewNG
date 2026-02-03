use backend::{database, models::{equipmentset::{EquipmentSet, EquipmentSetBuilder, NewEquipmentSet}, user::UserBuilder}};

pub fn arrange_create_works_integration_test(
    conn: &mut database::Connection
) -> NewEquipmentSet {
    let equipment_owner = UserBuilder::new_default("Carrie")
        .set_hash_password("notsupersecurepassword")
        .build_and_insert(conn)
        .unwrap();
    EquipmentSetBuilder::new_default(equipment_owner.id)
        .build()
        .unwrap()
}

pub fn arrange_get_all_works_integration_test(
    conn: &mut database::Connection
) -> (EquipmentSet, EquipmentSet) {
    let equipment_owner = UserBuilder::new_default("Derek")
        .set_hash_password("anothernotsupersecurepassword")
        .build_and_insert(conn)
        .unwrap();
    (
        EquipmentSetBuilder::new_default(equipment_owner.id)
            .set_name("Equipment Set 1")
            .build_and_insert(conn)
            .unwrap(),
        EquipmentSetBuilder::new_default(equipment_owner.id)
            .set_name("Equipment Set 2")
            .build_and_insert(conn)
            .unwrap()
    )
}

pub fn arrange_get_equipmentset_by_id_works_integration_test(
    conn: &mut database::Connection
) -> EquipmentSet {
    let equipment_owner = UserBuilder::new_default("Evelyn")
        .set_hash_password("yetanothernotsupersecurepassword")
        .build_and_insert(conn)
        .unwrap();
    EquipmentSetBuilder::new_default(equipment_owner.id)
            .set_name("Equipment Set 1")
            .build_and_insert(conn)
            .unwrap();
    EquipmentSetBuilder::new_default(equipment_owner.id)
        .set_name("Equipment Set 2")
        .build_and_insert(conn)
        .unwrap()
}

pub fn arrange_update_works_integration_test(
    conn: &mut database::Connection
) -> EquipmentSet {
    let equipment_owner = UserBuilder::new_default("Frank")
        .set_hash_password("securepassword123")
        .build_and_insert(conn)
        .unwrap();
    EquipmentSetBuilder::new_default(equipment_owner.id)
        .set_name("Old Equipment Set Name")
        .build_and_insert(conn)
        .unwrap()
}

pub fn arrange_delete_works_integration_test(
    conn: &mut database::Connection
) -> EquipmentSet {
    let equipment_owner = UserBuilder::new_default("Grace")
        .set_hash_password("mypasswordisverysecure")
        .build_and_insert(conn)
        .unwrap();
    EquipmentSetBuilder::new_default(equipment_owner.id)
        .set_name("Equipment Set To Be Deleted")
        .build_and_insert(conn)
        .unwrap()
}
