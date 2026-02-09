use backend::{database, models::{computer::ComputerBuilder, equipmentregistration::{EquipmentRegistration, EquipmentRegistrationBuilder}, equipmentset::EquipmentSetBuilder, monitor::MonitorBuilder, room::{NewRoom, Room, RoomBuilder}, tournament::{Tournament, TournamentBuilder}, user::UserBuilder}};
use diesel::prelude::*;
use uuid::Uuid;
use backend::schema::rooms;

pub fn seed_1_room_with_minimum_required_dependencies(db: &mut database::Connection) 
    -> (Room, Tournament) {
    let tour = TournamentBuilder::new_default("Tour 1")
        .build_and_insert(db)
        .unwrap();
    let room = RoomBuilder::new_default("Room 1", tour.tid)
        .build_and_insert(db)
        .unwrap();
    (room, tour)
}

pub fn new_room_one(tid: Uuid, room_name: &str) -> NewRoom {
    NewRoom {
        tid: tid,
        name: room_name.to_string(),
        building: "Building 451".to_string(),
        comments: "None at this time.".to_string()
    }
}

pub fn new_room_two(tid: Uuid, room_name: &str) -> NewRoom {
    NewRoom {
        tid: tid,
        name: room_name.to_string(),
        building: "Bldng 2".to_string(),
        comments: "I thought I recognized this place.".to_string()
    }
}

pub fn new_room_three(tid: Uuid, room_name: &str) -> NewRoom {
    NewRoom {
        tid: tid,
        name: room_name.to_string(),
        building: "Building H".to_string(),
        comments: "How'd we get here?".to_string()
    }
}

pub fn get_room_payload(tid: Uuid) -> NewRoom {
    new_room_one(tid, "Test Room 2217")
}

fn create_and_insert_room(conn: &mut PgConnection, new_room: NewRoom) -> Room {
    diesel::insert_into(rooms::table)
        .values(new_room)
        .returning(Room::as_returning())
        .get_result::<Room>(conn)
        .expect("Failed to create room")
}

pub fn seed_room(conn: &mut PgConnection, tid: Uuid) -> Room {
    let new_room = new_room_one(tid, "Test Room 3276");
    create_and_insert_room(conn, new_room)
}

pub fn seed_rooms(conn: &mut PgConnection, tid: Uuid) -> Vec<Room> {
    seed_rooms_with_names(
        conn, 
        tid, 
        "Test Room 3276", 
        "Test Room 9078", 
        "Test Room 4611")
}

pub fn seed_rooms_with_names(
    conn: &mut PgConnection, 
    tid: Uuid, 
    room_1_name: &str,
    room_2_name: &str,
    room_3_name: &str,
) -> Vec<Room> {
    let new_room_1 = new_room_one(tid, room_1_name);
    let new_room_2 = new_room_two(tid, room_2_name);
    let new_room_3 = new_room_three(tid, room_3_name);

    vec![
        create_and_insert_room(conn, new_room_1),
        create_and_insert_room(conn, new_room_2),
        create_and_insert_room(conn, new_room_3),
    ]
}

pub fn arrange_get_all_equipmentregistrations_of_room_works_integration_test(db: &mut database::Connection) 
    -> (Room, EquipmentRegistration, EquipmentRegistration) {
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
    
    let tour_1 = TournamentBuilder::new_default("Tour 1")
        .build_and_insert(db)
        .unwrap();
    let room_1 = RoomBuilder::new_default("Room 1", tour_1.tid)
        .build_and_insert(db)
        .unwrap();

    let computer = ComputerBuilder::new_default(equipment_set.id)
        .set_brand(Some("Test Brand".to_string()))
        .set_operating_system(Some("Test OS".to_string()))
        .set_misc_note(Some("This is a test computer.".to_string()))
        .build_and_insert(db)
        .unwrap();
    let equipmentregistration_1 = EquipmentRegistrationBuilder::new_default(computer.equipmentid, tour_1.tid)
        .set_roomid(Some(room_1.roomid))
        .build_and_insert(db)
        .unwrap();

    let monitor = MonitorBuilder::new_default(equipment_set.id)
        .set_size(Some("17 inches".to_string()))
        .set_brand(Some("Brand H".to_string()))
        .set_misc_note(Some("Test monitor for delete.".to_string()))
        .build_and_insert(db)
        .unwrap();
    let equipmentregistration_2 = EquipmentRegistrationBuilder::new_default(monitor.equipmentid, tour_1.tid)
        .set_roomid(Some(room_1.roomid))
        .build_and_insert(db)
        .unwrap();

    (room_1, equipmentregistration_1, equipmentregistration_2)
}
