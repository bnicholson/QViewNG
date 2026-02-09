
mod common;
mod fixtures;

use backend::{database::Database, models::{computer::Computer, equipment::Equipment, equipmentregistration::EquipmentRegistration, extensioncord::ExtensionCord, interfacebox::InterfaceBox, jumppad::JumpPad, microphonerecorder::MicrophoneRecorder, monitor::Monitor, powerstrip::PowerStrip, projector::Projector}, routes::configure_routes};
use crate::common::{PAGE_NUM, PAGE_SIZE, TEST_DB_URL, clean_database};
use actix_web::{App, test, web::{self}};
use actix_http::StatusCode;

#[actix_web::test]
async fn get_by_id_works() {

    // 1 test for each Equipment type.

    // Arrange All:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (computer, jumppad, interfacebox, monitor, microphonerecorder, 
        projector, powerstrip, extensioncord) = 
        fixtures::equipment::arrange_get_equipment_by_id_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;

    // Arrange (Computer)

    let computer_uri = format!("/api/equipment/{}", &computer.equipmentid);
    let computer_req = test::TestRequest::get()
        .uri(computer_uri.as_str())
        .to_request();

    // Act (Computer):
    
    let computer_resp = test::call_service(&app, computer_req).await;
    
    // Assert (Computer):

    assert_eq!(computer_resp.status(), StatusCode::OK);
    
    let body_computer_equipment: Equipment = test::read_body_json(computer_resp).await;
    let body_computer: Computer = match body_computer_equipment {
        Equipment::Computer(c) => c,
        _ => panic!("Expected Computer variant"),
    }; 
    assert_eq!(body_computer.brand, computer.brand);

    // Arrange (JumpPad)

    let jumppad_uri = format!("/api/equipment/{}", &jumppad.equipmentid);
    let jumppad_req = test::TestRequest::get()
        .uri(jumppad_uri.as_str())
        .to_request();

    // Act (JumpPad):
    
    let jumppad_resp = test::call_service(&app, jumppad_req).await;
    
    // Assert (JumpPad):

    assert_eq!(jumppad_resp.status(), StatusCode::OK);  

    let body_jumppad_equipment: Equipment = test::read_body_json(jumppad_resp).await;
    let body_jumppad: JumpPad = match body_jumppad_equipment {
        Equipment::JumpPad(e) => e,
        _ => panic!("Expected JumpPad variant"),
    }; 
    assert_eq!(body_jumppad.color, jumppad.color);

    // Arrange (InterfaceBox)

    let interfacebox_uri = format!("/api/equipment/{}", &interfacebox.equipmentid);
    let interfacebox_req = test::TestRequest::get()
        .uri(interfacebox_uri.as_str())
        .to_request();

    // Act (InterfaceBox):
    
    let interfacebox_resp = test::call_service(&app, interfacebox_req).await;
    
    // Assert (InterfaceBox):

    assert_eq!(interfacebox_resp.status(), StatusCode::OK);  

    let body_interfacebox_equipment: Equipment = test::read_body_json(interfacebox_resp).await;
    let body_interfacebox: InterfaceBox = match body_interfacebox_equipment {
        Equipment::InterfaceBox(e) => e,
        _ => panic!("Expected InterfaceBox variant"),
    }; 
    assert_eq!(body_interfacebox.serial_number, interfacebox.serial_number);

    // Arrange (Monitor)

    let monitor_uri = format!("/api/equipment/{}", &monitor.equipmentid);
    let monitor_req = test::TestRequest::get()
        .uri(monitor_uri.as_str())
        .to_request();

    // Act (Monitor):
    
    let monitor_resp = test::call_service(&app, monitor_req).await;
    
    // Assert (Monitor):

    assert_eq!(monitor_resp.status(), StatusCode::OK);  

    let body_monitor_equipment: Equipment = test::read_body_json(monitor_resp).await;
    let body_monitor: Monitor = match body_monitor_equipment {
        Equipment::Monitor(e) => e,
        _ => panic!("Expected Monitor variant"),
    }; 
    assert_eq!(body_monitor.brand, monitor.brand);

    // Arrange (MicrophoneRecorder)

    let microphonerecorder_uri = format!("/api/equipment/{}", &microphonerecorder.equipmentid);
    let microphonerecorder_req = test::TestRequest::get()
        .uri(microphonerecorder_uri.as_str())
        .to_request();

    // Act (MicrophoneRecorder):
    
    let microphonerecorder_resp = test::call_service(&app, microphonerecorder_req).await;
    
    // Assert (MicrophoneRecorder):

    assert_eq!(microphonerecorder_resp.status(), StatusCode::OK);  

    let body_microphonerecorder_equipment: Equipment = test::read_body_json(microphonerecorder_resp).await;
    let body_microphonerecorder: MicrophoneRecorder = match body_microphonerecorder_equipment {
        Equipment::MicrophoneRecorder(e) => e,
        _ => panic!("Expected MicrophoneRecorder variant"),
    }; 
    assert_eq!(body_microphonerecorder.type_, microphonerecorder.type_);

    // Arrange (Projector)

    let projector_uri = format!("/api/equipment/{}", &projector.equipmentid);
    let projector_req = test::TestRequest::get()
        .uri(projector_uri.as_str())
        .to_request();

    // Act (Projector):
    
    let projector_resp = test::call_service(&app, projector_req).await;
    
    // Assert (Projector):

    assert_eq!(projector_resp.status(), StatusCode::OK);  

    let body_projector_equipment: Equipment = test::read_body_json(projector_resp).await;
    let body_projector: Projector = match body_projector_equipment {
        Equipment::Projector(e) => e,
        _ => panic!("Expected Projector variant"),
    }; 
    assert_eq!(body_projector.brand, projector.brand);

    // Arrange (PowerStrip)

    let powerstrip_uri = format!("/api/equipment/{}", &powerstrip.equipmentid);
    let powerstrip_req = test::TestRequest::get()
        .uri(powerstrip_uri.as_str())
        .to_request();

    // Act (PowerStrip):
    
    let powerstrip_resp = test::call_service(&app, powerstrip_req).await;
    
    // Assert (PowerStrip):

    assert_eq!(powerstrip_resp.status(), StatusCode::OK);  

    let body_powerstrip_equipment: Equipment = test::read_body_json(powerstrip_resp).await;
    let body_powerstrip: PowerStrip = match body_powerstrip_equipment {
        Equipment::PowerStrip(e) => e,
        _ => panic!("Expected PowerStrip variant"),
    }; 
    assert_eq!(body_powerstrip.num_of_plugs, powerstrip.num_of_plugs);

    // Arrange (ExtensionCord)

    let extensioncord_uri = format!("/api/equipment/{}", &extensioncord.equipmentid);
    let extensioncord_req = test::TestRequest::get()
        .uri(extensioncord_uri.as_str())
        .to_request();

    // Act (ExtensionCord):
    
    let extensioncord_resp = test::call_service(&app, extensioncord_req).await;
    
    // Assert (ExtensionCord):

    assert_eq!(extensioncord_resp.status(), StatusCode::OK);  

    let body_extensioncord_equipment: Equipment = test::read_body_json(extensioncord_resp).await;
    let body_extensioncord: ExtensionCord = match body_extensioncord_equipment {
        Equipment::ExtensionCord(e) => e,
        _ => panic!("Expected ExtensionCord variant"),
    }; 
    assert_eq!(body_extensioncord.length, extensioncord.length);
}

#[actix_web::test]
async fn get_all_equipmentregistrations_of_equipment_piece_works() {

    // Arrange:
    
    clean_database();
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");
    
    let (computer, er_1, er_2) = 
        fixtures::equipment::arrange_get_all_equipmentregistrations_of_equipment_piece_works_integration_test(&mut conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(configure_routes)
    ).await;
    
    let uri = format!("/api/equipment/{}/equipmentregistrations?page={}&page_size={}", computer.equipmentid, PAGE_NUM, PAGE_SIZE);
    let req = test::TestRequest::get()
        .uri(&uri)
        .to_request();
    
    // Act:
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Assert:

    let body: Vec<EquipmentRegistration> = test::read_body_json(resp).await;

    let len = 2;

    assert_eq!(body.len(), len);

    let mut equipmentregistration_1_idx = 10;
    let mut equipmentregistration_2_idx = 10;
    for idx in 0..len {
        if body[idx].id == er_1.id {
            equipmentregistration_1_idx = idx;
        }
        if body[idx].id == er_2.id {
            equipmentregistration_2_idx = idx;
        }
    }
    assert_ne!(equipmentregistration_1_idx, 10);
    assert_ne!(equipmentregistration_2_idx, 10);
}
