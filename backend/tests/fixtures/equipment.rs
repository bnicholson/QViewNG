use backend::{database, models::{computer::{ComputerBuilder, Computer}, equipmentset::EquipmentSetBuilder, extensioncord::{ExtensionCordBuilder, ExtensionCord}, interfacebox::{InterfaceBoxBuilder, InterfaceBox}, jumppad::{JumpPadBuilder, JumpPad}, microphonerecorder::{MicrophoneRecorderBuilder, MicrophoneRecorder}, monitor::{MonitorBuilder, Monitor}, powerstrip::{PowerStrip, PowerStripBuilder}, projector::{Projector, ProjectorBuilder}, user::UserBuilder}};


pub fn arrange_get_equipment_by_id_works_integration_test(db: &mut database::Connection) -> 
    (Computer, JumpPad, InterfaceBox, Monitor, MicrophoneRecorder, Projector, PowerStrip, ExtensionCord)
{
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
    (
        ComputerBuilder::new_default(equipment_set.id)
            .set_brand(Some("Test Brand".to_string()))
            .set_operating_system(Some("Test OS".to_string()))
            .set_misc_note(Some("This is a test computer.".to_string()))
            .build_and_insert(db)
            .unwrap(),
        JumpPadBuilder::new_default(equipment_set.id)
            .set_color(Some("blue".to_string()))
            .set_misc_note(Some("Test jumppad for delete.".to_string()))
            .build_and_insert(db)
            .unwrap(),
        InterfaceBoxBuilder::new_default(equipment_set.id)
            .set_type_(Some("USB".to_string()))
            .set_serial_number(Some("hloiununun".to_string()))
            .set_misc_note(Some("Test interfacebox for delete.".to_string()))
            .build_and_insert(db)
            .unwrap(),
        MonitorBuilder::new_default(equipment_set.id)
            .set_size(Some("17 inches".to_string()))
            .set_brand(Some("Brand H".to_string()))
            .set_misc_note(Some("Test monitor for delete.".to_string()))
            .build_and_insert(db)
            .unwrap(),
        MicrophoneRecorderBuilder::new_default(equipment_set.id)
            .set_type_(Some("External".to_string()))
            .set_misc_note(Some("Test microphonerecorder for delete.".to_string()))
            .build_and_insert(db)
            .unwrap(),
        ProjectorBuilder::new_default(equipment_set.id)
            .set_brand(Some("CoolFlik".to_string()))
            .set_misc_note(Some("Test projector for delete.".to_string()))
            .build_and_insert(db)
            .unwrap(),
        PowerStripBuilder::new_default(equipment_set.id)
            .set_make(Some("Amazon".to_string()))
            .set_model(Some("6-port".to_string()))
            .set_num_of_plugs(Some(6))
            .set_misc_note(Some("Test powerstrip for delete.".to_string()))
            .build_and_insert(db)
            .unwrap(),
        ExtensionCordBuilder::new_default(equipment_set.id)
            .set_length(Some("10-foot".to_string()))
            .set_misc_note(Some("Test extensioncord for delete.".to_string()))
            .build_and_insert(db)
            .unwrap()
    )
}
