
use crate::database;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable};
use serde::{Deserialize, Serialize};
use crate::models::common::*;
use utoipa::ToSchema;
// this import requires this syntax (to appease rustc):
use crate::schema::rooms::dsl::{roomid,tid,name,building,comments};
use chrono::{Utc,DateTime};

// #[tsync::tsync]
#[derive(
Debug,
Serialize,
Deserialize,
Clone,
Queryable,
Insertable,
Identifiable,
AsChangeset,
ToSchema
)]
#[diesel(table_name = crate::schema::rooms)]
#[diesel(primary_key(roomid))]
pub struct Room {
    pub roomid: BigId,                          // identifies the room uniquely
    pub tid: BigId,                             // id of the associated tournament
    pub name: String,                           // Name of the room (human readable)
    pub building: String,                       // What is the building this room is in
    pub comments: String,                       // Any comments about the room,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

// #[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::rooms)]
#[diesel(primary_key(roomid))]
pub struct RoomChangeset {   
    pub tid: BigId,                             // id of the associated tournament    
    pub name: String,                           // Name of the room (human readable)
    pub building: String,                       // What is the building this room is in
    pub comments: String,                       // Any comments about the room
}

pub fn create(db: &mut database::Connection, item: &RoomChangeset) -> QueryResult<Room> {
    use crate::schema::rooms::dsl::*;
    insert_into(rooms).values(item).get_result::<Room>(db)
}

pub fn read(db: &mut database::Connection, item_id: BigId) -> QueryResult<Room> {
    use crate::schema::rooms::dsl::*;
    rooms.filter(roomid.eq(item_id)).first::<Room>(db)
}

pub fn read_all(db: &mut database::Connection, tournamentid: BigId) -> QueryResult<Vec<Room>> {
    use crate::schema::rooms::dsl::*;
    let values = rooms
        .order(name)
        .filter(tid.eq(tournamentid))
        .load::<Room>(db);
        values
}

pub fn update(db: &mut database::Connection, item_id: BigId, item: &RoomChangeset) -> QueryResult<Room> {
    use crate::schema::rooms::dsl::*;
    diesel::update(rooms.filter(roomid.eq(item_id)))
        .set(item)
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, item_id: BigId) -> QueryResult<usize> {
    use crate::schema::rooms::dsl::*;
    diesel::delete(rooms.filter(roomid.eq(item_id))).execute(db)
}
