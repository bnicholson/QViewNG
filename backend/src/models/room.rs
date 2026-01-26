
use crate::database;
use crate::models::common::PaginationParams;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{Utc,DateTime};

// #[tsync::tsync]
#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Identifiable,
    ToSchema
)]
#[diesel(table_name = crate::schema::rooms)]
#[diesel(primary_key(roomid))]
pub struct Room {
    pub roomid: Uuid,                           // identifies the room uniquely
    pub name: String,                           // Name of the room (human readable)
    pub building: String,                       // What is the building this room is in
    pub comments: String,                       // Any comments about the room,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tid: Uuid                               // id of the associated tournament
}

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug
)]
#[diesel(table_name = crate::schema::rooms)]
pub struct NewRoom {
    pub name: String,                           // Name of the room (human readable)
    pub building: String,                       // What is the building this room is in
    pub comments: String,                       // Any comments about the room,
    pub tid: Uuid                               // id of the associated tournament
}

// #[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::rooms)]
#[diesel(primary_key(roomid))]
pub struct RoomChangeset {   
    pub name: Option<String>,                   // Name of the room (human readable)
    pub building: Option<String>,               // What is the building this room is in
    pub comments: Option<String>                // Any comments about the room
}

pub fn create(db: &mut database::Connection, item: &NewRoom) -> QueryResult<Room> {
    use crate::schema::rooms::dsl::*;
    insert_into(rooms).values(item).get_result::<Room>(db)
}

pub fn exists(db: &mut database::Connection, roomid: Uuid) -> bool {
    use crate::schema::rooms::dsl::rooms;
    rooms
        .find(roomid)
        .get_result::<Room>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<Room> {
    use crate::schema::rooms::dsl::*;
    rooms.filter(roomid.eq(item_id)).first::<Room>(db)
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<Room>> {
    use crate::schema::rooms::dsl::*;
    rooms
        .order(created_at)
        .limit(pagination.page_size)
        .offset(
            pagination.page
                * std::cmp::max(pagination.page_size, PaginationParams::MAX_PAGE_SIZE as i64),
        )
        .load::<Room>(db)
}

pub fn read_all_rooms_of_tournament(
    db: &mut database::Connection,
    item_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<Vec<Room>> {
    use crate::schema::rooms::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    rooms
        .filter(tid.eq(item_id))
        .order(name.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<Room>(db)
}

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &RoomChangeset) -> QueryResult<Room> {
    use crate::schema::rooms::dsl::*;
    diesel::update(rooms.filter(roomid.eq(item_id)))
        .set((
            item,
            updated_at.eq(diesel::dsl::now),
        ))
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::rooms::dsl::*;
    diesel::delete(rooms.filter(roomid.eq(item_id))).execute(db)
}
