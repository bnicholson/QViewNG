use backend::{database, models::{computer::{Computer, ComputerBuilder}, equipmentregistration::{EquipmentRegistration, EquipmentRegistrationBuilder, NewEquipmentRegistration}, equipmentset::{EquipmentSet, EquipmentSetBuilder}, room::Room, tournament::{Tournament, TournamentBuilder}, user::{User, UserBuilder}}};

use crate::fixtures::{equipment::seed_1_of_each_equipment_piece_type, rooms::seed_1_room_with_minimum_required_dependencies};

pub fn seed_1_equipmentregistration_for_each_equipment_type_with_minimum_required_dependencies(db: &mut database::Connection) 
    -> (Tournament, EquipmentRegistration,EquipmentRegistration,EquipmentRegistration,EquipmentRegistration,
        EquipmentRegistration,EquipmentRegistration,EquipmentRegistration,EquipmentRegistration) {
    let tour_1 = TournamentBuilder::new_default("Tour 1")
        .build_and_insert(db)
        .unwrap();
    let (computer, jumppad, interfacebox, monitor, microphonerecorder, projector, powerstrip, extensioncord) = 
        seed_1_of_each_equipment_piece_type(db);
    (
        tour_1.clone(),
        EquipmentRegistrationBuilder::new_default(computer.equipmentid, tour_1.tid)
            .build_and_insert(db)
            .unwrap(),
        EquipmentRegistrationBuilder::new_default(jumppad.equipmentid, tour_1.tid)
            .build_and_insert(db)
            .unwrap(),
        EquipmentRegistrationBuilder::new_default(interfacebox.equipmentid, tour_1.tid)
            .build_and_insert(db)
            .unwrap(),
        EquipmentRegistrationBuilder::new_default(monitor.equipmentid, tour_1.tid)
            .build_and_insert(db)
            .unwrap(),
        EquipmentRegistrationBuilder::new_default(microphonerecorder.equipmentid, tour_1.tid)
            .build_and_insert(db)
            .unwrap(),
        EquipmentRegistrationBuilder::new_default(projector.equipmentid, tour_1.tid)
            .build_and_insert(db)
            .unwrap(),
        EquipmentRegistrationBuilder::new_default(powerstrip.equipmentid, tour_1.tid)
            .build_and_insert(db)
            .unwrap(),
        EquipmentRegistrationBuilder::new_default(extensioncord.equipmentid, tour_1.tid)
            .build_and_insert(db)
            .unwrap()
    )   
}

pub fn seed_1_equipmentregistration_with_minimum_required_dependencies(db: &mut database::Connection) 
    -> (EquipmentRegistration, Tournament, Computer, EquipmentSet, User) {
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
    let computer = ComputerBuilder::new_default(equipment_set.id)
        .set_brand(Some("Test Brand".to_string()))
        .set_operating_system(Some("Test OS".to_string()))
        .set_misc_note(Some("This is a test computer.".to_string()))
        .build_and_insert(db)
        .unwrap();

    let tour = TournamentBuilder::new_default("Tour 1")
        .build_and_insert(db)
        .unwrap();

    let equipment_registration = EquipmentRegistrationBuilder::new_default(computer.equipmentid, tour.tid)
        .build_and_insert(db)
        .unwrap();

    (equipment_registration, tour, computer, equipment_set, user)
}

pub fn arrange_create_works_integration_test(db: &mut database::Connection) -> NewEquipmentRegistration {
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
    let computer = ComputerBuilder::new_default(equipment_set.id)
        .set_brand(Some("Test Brand".to_string()))
        .set_operating_system(Some("Test OS".to_string()))
        .set_misc_note(Some("This is a test computer.".to_string()))
        .build_and_insert(db)
        .unwrap();

    let tour = TournamentBuilder::new_default("Tour 1")
        .build_and_insert(db)
        .unwrap();

    EquipmentRegistrationBuilder::new_default(computer.equipmentid, tour.tid)
        .build()
        .unwrap()
}

pub fn arrange_get_all_works_integration_test(db: &mut database::Connection) -> (EquipmentRegistration, EquipmentRegistration) {
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
    let computer_1 = ComputerBuilder::new_default(equipment_set.id)
        .set_brand(Some("Test Brand".to_string()))
        .set_operating_system(Some("Test OS".to_string()))
        .set_misc_note(Some("This is a test computer.".to_string()))
        .build_and_insert(db)
        .unwrap();
    let computer_2 = ComputerBuilder::new_default(equipment_set.id)
        .set_brand(Some("CoolioShmoolio".to_string()))
        .set_operating_system(Some("CrashOS".to_string()))
        .set_misc_note(Some("This is a test computer.".to_string()))
        .build_and_insert(db)
        .unwrap();

    let tour = TournamentBuilder::new_default("Tour 1")
        .build_and_insert(db)
        .unwrap();

    (
        EquipmentRegistrationBuilder::new_default(computer_1.equipmentid, tour.tid)
            .build_and_insert(db)
            .unwrap(),
        EquipmentRegistrationBuilder::new_default(computer_2.equipmentid, tour.tid)
            .build_and_insert(db)
            .unwrap()
    )
}

pub fn arrange_get_by_id_works_integration_test(db: &mut database::Connection) -> EquipmentRegistration {
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
    let computer_1 = ComputerBuilder::new_default(equipment_set.id)
        .set_brand(Some("Test Brand".to_string()))
        .set_operating_system(Some("Test OS".to_string()))
        .set_misc_note(Some("This is a test computer.".to_string()))
        .build_and_insert(db)
        .unwrap();
    let computer_2 = ComputerBuilder::new_default(equipment_set.id)
        .set_brand(Some("CoolioShmoolio".to_string()))
        .set_operating_system(Some("CrashOS".to_string()))
        .set_misc_note(Some("This is a test computer.".to_string()))
        .build_and_insert(db)
        .unwrap();

    let tour = TournamentBuilder::new_default("Tour 1")
        .build_and_insert(db)
        .unwrap();

    EquipmentRegistrationBuilder::new_default(computer_1.equipmentid, tour.tid)
        .build_and_insert(db)
        .unwrap();
    EquipmentRegistrationBuilder::new_default(computer_2.equipmentid, tour.tid)
        .build_and_insert(db)
        .unwrap()
}

pub fn arrange_update_works_integration_test(db: &mut database::Connection) -> (EquipmentRegistration, Room) {
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
    let computer_1 = ComputerBuilder::new_default(equipment_set.id)
        .set_brand(Some("Test Brand".to_string()))
        .set_operating_system(Some("Test OS".to_string()))
        .set_misc_note(Some("This is a test computer.".to_string()))
        .build_and_insert(db)
        .unwrap();

    let (room, tour) = seed_1_room_with_minimum_required_dependencies(db);

    (
        EquipmentRegistrationBuilder::new_default(computer_1.equipmentid, tour.tid)
            .build_and_insert(db)
            .unwrap(),
        room
    )
}

pub fn arrange_delete_works_integration_test(db: &mut database::Connection) -> EquipmentRegistration {
    let (equipmentregistration, _,_,_,_,) = 
        seed_1_equipmentregistration_with_minimum_required_dependencies(db);
    equipmentregistration
}
