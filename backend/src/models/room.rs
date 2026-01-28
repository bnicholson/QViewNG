
use crate::database;
use crate::models::common::PaginationParams;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{Utc,DateTime};

pub struct RoomBuilder {
    name: Option<String>,                           // Name of the room (human readable)
    building: Option<String>,                       // What is the building this room is in
    comments: Option<String>,                       // Any comments about the room,
    tid: Uuid                                       // id of the associated tournament
}

impl RoomBuilder {
    pub fn new(room_name: &str, tid: Uuid) -> Self {
        Self {
            name: Some(room_name.to_string()),
            building: None,
            comments: None,
            tid: tid
        }
    }
    pub fn new_default(room_name: &str, tid: Uuid) -> Self {
        Self {
            name: Some(room_name.to_string()),
            building: Some("Building 451".to_string()),
            comments: Some("None at this time.".to_string()),
            tid: tid
        }
    }
    pub fn set_name(mut self, room_name: String) -> Self {
        self.name = Some(room_name);
        self
    }
    pub fn set_building(mut self, building: String) -> Self {
        self.building = Some(building);
        self
    }
    pub fn set_comments(mut self, comments: String) -> Self {
        self.comments = Some(comments);
        self
    }
    pub fn set_tid(mut self, tid: Uuid) -> Self {
        self.tid = tid;
        self
    }
    fn validate_all_are_some(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if self.name.is_none() {
            errors.push("name is required".to_string());
        }
        if self.building.is_none() {
            errors.push("building is required".to_string());
        }
        if self.comments.is_none() {
            errors.push("comments is required".to_string());
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
    pub fn build(self) -> Result<NewRoom, Vec<String>> {
        match self.validate_all_are_some() {
            Err(e) => {
                Err(e)
            },
            Ok(_) => {
                Ok(
                    NewRoom {
                        name: self.name.unwrap(),            // Name of the room (human readable)
                        building: self.building.unwrap(),    // What is the building this room is in
                        comments: self.comments.unwrap(),    // Any comments about the room,
                        tid: self.tid                        // id of the associated tournament
                    }
                )
            }
        }
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<Room> {
        let new_room = self.build();
        create(db, &new_room.unwrap())
    }
}

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
