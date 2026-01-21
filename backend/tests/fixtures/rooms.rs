use backend::models::room::{Room,NewRoom};
use diesel::prelude::*;
use uuid::Uuid;
use backend::schema::rooms;

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
