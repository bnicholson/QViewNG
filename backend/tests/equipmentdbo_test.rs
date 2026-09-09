
mod common;
mod fixtures;

use backend::{database::Database, models::{self, common::PaginationParams, equipment_dbo::{EquipmentDbo, EquipmentDboChangeset}}};
use crate::common::{PAGE_SIZE, TEST_DB_URL, clean_database};
use diesel::prelude::*;

#[actix_web::test]
async fn create_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let payload = fixtures::equipment_dbos::arrange_create_works_integration_test(&mut conn);

    // Act:

    let equipment_dbo_result = models::equipment_dbo::create(&mut conn, &payload);    
    
    // Assert:

    // if equipment_dbo_result.is_err() {
    //     println!("Error creating EquipmentDbo: {:?}", equipment_dbo_result.as_ref().unwrap_err());
    //     assert!(false);
    // }
    
    assert!(equipment_dbo_result.is_ok());

    let equipment_dbo: EquipmentDbo = equipment_dbo_result.unwrap();
    assert_eq!(equipment_dbo.computerid, payload.computerid);
    assert_eq!(equipment_dbo.jumppadid, payload.jumppadid);
    assert_eq!(equipment_dbo.interfaceboxid, payload.interfaceboxid);
    assert_eq!(equipment_dbo.monitorid, payload.monitorid);
    assert_eq!(equipment_dbo.microphonerecorderid, payload.microphonerecorderid);
    assert_eq!(equipment_dbo.projectorid, payload.projectorid);
    assert_eq!(equipment_dbo.powerstripid, payload.powerstripid);
    assert_eq!(equipment_dbo.extensioncordid, payload.extensioncordid);
    assert_eq!(equipment_dbo.misc_note, payload.misc_note);
}

#[actix_web::test]
async fn get_all_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (equipment_computer_1, equipment_computer_2) = 
        fixtures::equipment_dbos::arrange_get_all_works_integration_test(&mut conn);

    let pagination = PaginationParams {
        page: 0,
        page_size: PAGE_SIZE,
    };
    
    // Act:
    
    let equipment_dbo_result = models::equipment_dbo::read_all(&mut conn, &pagination);    
    
    // Assert:
        
    let equipment_dbo_vec: Vec<EquipmentDbo> = equipment_dbo_result.unwrap();

    let len = 2;

    assert_eq![equipment_dbo_vec.len(), len];

    let mut equipment_1_interest_idx = 10;
    let mut equipment_2_interest_idx = 10;
    for idx in 0..len {
        if equipment_dbo_vec[idx].id == equipment_computer_1.equipmentid {
            equipment_1_interest_idx = idx;
            continue;
        }
        if equipment_dbo_vec[idx].id == equipment_computer_2.equipmentid {
            equipment_2_interest_idx = idx;
            continue;
        }
    }
    assert_ne!(equipment_1_interest_idx, 10);
    assert_ne!(equipment_2_interest_idx, 10);
}

#[actix_web::test]
async fn get_by_id_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let equipment = 
        fixtures::equipment_dbos::arrange_get_equipment_by_id_works_integration_test(&mut conn);

    // Act:
    
    let equipment_dbo_result = models::equipment_dbo::read(&mut conn, equipment.id);    

    // Assert:

    assert!(equipment_dbo_result.is_ok());

    let equipment_dbo: EquipmentDbo = equipment_dbo_result.unwrap();
    assert_eq!(equipment.misc_note, equipment_dbo.misc_note);
}

#[actix_web::test]
async fn update_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let original_equipment = 
        fixtures::equipment_dbos::arrange_update_works_integration_test(&mut conn);

    let new_misc_note = "NEW Misc Note".to_string();

    let put_payload = EquipmentDboChangeset {
        misc_note: Some(new_misc_note),
        equipmentsetid: None,
    };

    // Act:
    
    let equipment_dbo_result = models::equipment_dbo::update(&mut conn, original_equipment.id, &put_payload, original_equipment.last_modified_user);

    // Assert:
    
    assert!(equipment_dbo_result.is_ok());
    let equipment_dbo = equipment_dbo_result.unwrap();
    assert_eq!(equipment_dbo.misc_note, put_payload.misc_note);
}

#[actix_web::test]
async fn delete_works() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let computer = fixtures::equipment_dbos::arrange_delete_works_integration_test(&mut conn);

    // Act:
    
    let equipment_dbo_result = models::equipment_dbo::delete(&mut conn, computer.equipmentid);   

    // Assert:
    
    assert_eq!(equipment_dbo_result.unwrap(), 1);  // count of records deleted

    // Check DB if it shows the correct number of equipment records after deletion:
    let pagination = PaginationParams {
        page: 0,
        page_size: PAGE_SIZE,
    };
    let get_result = models::equipment_dbo::read_all(&mut conn, &pagination);  
    assert!(get_result.is_ok());  

    let equipment_dbo_vec = get_result.unwrap();
    assert_eq!(equipment_dbo_vec.len(), 0);
}

#[actix_web::test]
async fn delete_soft_deletes_keeps_component_and_purge_removes_both() {

    // Arrange:

    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    let computer = fixtures::equipment_dbos::arrange_delete_works_integration_test(&mut conn);
    let equipment_id = computer.equipmentid;
    let component_id = computer.computerid;

    // Act + Assert: a gear-item delete soft-deletes the equipment row (hidden from reads) but keeps
    // the underlying component row so the item stays recoverable.
    let (component_deleted, equipment_soft_deleted) =
        models::computer::delete(&mut conn, equipment_id).unwrap();
    assert_eq!(component_deleted, 0);
    assert_eq!(equipment_soft_deleted, 1);
    assert!(models::equipment_dbo::read(&mut conn, equipment_id).is_err());

    // The equipment row still exists with del_fl = true (raw query that ignores the flag)...
    use backend::schema::equipment::dsl as e;
    let eq_count: i64 = e::equipment.filter(e::id.eq(equipment_id)).count().get_result(&mut conn).unwrap();
    assert_eq!(eq_count, 1);
    let flag: bool = e::equipment.filter(e::id.eq(equipment_id)).select(e::del_fl).first(&mut conn).unwrap();
    assert!(flag);
    // ...and the underlying component row is untouched.
    use backend::schema::computers::dsl as c;
    let comp_count: i64 = c::computers.filter(c::computerid.eq(component_id)).count().get_result(&mut conn).unwrap();
    assert_eq!(comp_count, 1);

    // Act + Assert: purge permanently removes both the equipment row and its component row.
    let (component_purged, equipment_purged) =
        models::computer::purge(&mut conn, equipment_id).unwrap();
    assert_eq!(component_purged, 1);
    assert_eq!(equipment_purged, 1);
    let eq_count_after: i64 = e::equipment.filter(e::id.eq(equipment_id)).count().get_result(&mut conn).unwrap();
    assert_eq!(eq_count_after, 0);
    let comp_count_after: i64 = c::computers.filter(c::computerid.eq(component_id)).count().get_result(&mut conn).unwrap();
    assert_eq!(comp_count_after, 0);
}
