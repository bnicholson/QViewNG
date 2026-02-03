use backend::{database, models::{equipmentset::{EquipmentSetBuilder, NewEquipmentSet}, user::UserBuilder}};

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
