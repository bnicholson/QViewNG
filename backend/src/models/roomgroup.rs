use crate::database;
use crate::models::common::PaginationParams;
use std::collections::HashMap;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult, AsChangeset, Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

pub struct RoomGroupBuilder {
    tournamentid: Uuid,
    type_: String,
    name: Option<String>,
    notes: String,
    creator_userid: Option<Uuid>,
    last_modified_userid: Option<Uuid>,
}

impl RoomGroupBuilder {
    pub fn new(tournamentid: Uuid) -> Self {
        Self {
            tournamentid,
            type_: "building".to_string(),
            name: None,
            notes: String::new(),
            creator_userid: None,
            last_modified_userid: None,
        }
    }
    pub fn new_default(tournamentid: Uuid) -> Self {
        Self::new(tournamentid)
    }
    pub fn set_type(mut self, type_: &str) -> Self {
        self.type_ = type_.to_string();
        self
    }
    pub fn set_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
    pub fn set_notes(mut self, notes: &str) -> Self {
        self.notes = notes.to_string();
        self
    }
    pub fn set_creator_userid(mut self, user_id: Uuid) -> Self {
        self.creator_userid = Some(user_id);
        self
    }
    pub fn set_last_modified_userid(mut self, user_id: Uuid) -> Self {
        self.last_modified_userid = Some(user_id);
        self
    }
    pub fn build(self) -> Result<NewRoomGroup, Vec<String>> {
        let mut errors = Vec::new();
        if self.name.is_none() { errors.push("name is required".to_string()); }
        if self.creator_userid.is_none() { errors.push("creator_userid is required".to_string()); }
        if !errors.is_empty() { return Err(errors); }
        let creator = self.creator_userid.unwrap();
        Ok(NewRoomGroup {
            tournamentid: self.tournamentid,
            type_: self.type_,
            name: self.name.unwrap(),
            notes: self.notes,
            creator_userid: creator,
            last_modified_userid: self.last_modified_userid.unwrap_or(creator),
        })
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<RoomGroup> {
        create(db, &self.build().unwrap())
    }
}

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
#[diesel(table_name = crate::schema::roomgroups)]
#[diesel(primary_key(roomgroupid))]
pub struct RoomGroup {
    pub roomgroupid: Uuid,                     // identifies the roomgroup uniquely
    pub tournamentid: Uuid,                    // parent tournament
    #[diesel(column_name = type_)]
    #[serde(rename = "type")]
    pub type_: String,                         // grouping type (e.g. "building"); defaults to "building"
    pub name: String,
    pub notes: String,
    pub created_date: DateTime<Utc>,
    pub creator_userid: Uuid,
    pub last_modified_date: DateTime<Utc>,
    pub last_modified_userid: Uuid,
    pub del_fl: bool,                          // soft-delete flag
}

fn default_roomgroup_type() -> String { "building".to_string() }

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::roomgroups)]
pub struct NewRoomGroup {
    pub tournamentid: Uuid,
    #[diesel(column_name = type_)]
    #[serde(rename = "type", default = "default_roomgroup_type")]
    pub type_: String,
    pub name: String,
    #[serde(default)]
    pub notes: String,
    // Set from the authenticated user in the service layer; API payloads omit these.
    #[serde(default)]
    pub creator_userid: Uuid,
    #[serde(default)]
    pub last_modified_userid: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::roomgroups)]
#[diesel(primary_key(roomgroupid))]
pub struct RoomGroupChangeset {
    #[diesel(column_name = type_)]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub name: Option<String>,
    pub notes: Option<String>,
}

pub fn create(db: &mut database::Connection, item: &NewRoomGroup) -> QueryResult<RoomGroup> {
    use crate::schema::roomgroups::dsl::*;
    insert_into(roomgroups).values(item).get_result::<RoomGroup>(db)
}

pub fn exists(db: &mut database::Connection, item_id: Uuid) -> bool {
    use crate::schema::roomgroups::dsl::*;
    roomgroups.find(item_id).filter(del_fl.eq(false)).get_result::<RoomGroup>(db).is_ok()
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<RoomGroup> {
    use crate::schema::roomgroups::dsl::*;
    roomgroups.filter(roomgroupid.eq(item_id)).filter(del_fl.eq(false)).first::<RoomGroup>(db)
}

/// Read ignoring the soft-delete flag — used by purge, which must resolve even a soft-deleted row.
pub fn read_including_deleted(db: &mut database::Connection, item_id: Uuid) -> QueryResult<RoomGroup> {
    use crate::schema::roomgroups::dsl::*;
    roomgroups.filter(roomgroupid.eq(item_id)).first::<RoomGroup>(db)
}

pub fn read_all(db: &mut database::Connection) -> QueryResult<Vec<RoomGroup>> {
    use crate::schema::roomgroups::dsl::*;
    roomgroups.filter(del_fl.eq(false)).order(name).load::<RoomGroup>(db)
}

/// The roomgroups belonging to a tournament (its buildings). A tournament is the roomgroup's parent,
/// so this is a direct lookup.
pub fn read_all_of_tournament(db: &mut database::Connection, tournament_id: Uuid) -> QueryResult<Vec<RoomGroup>> {
    use crate::schema::roomgroups::dsl::*;
    roomgroups
        .filter(tournamentid.eq(tournament_id))
        .filter(del_fl.eq(false))
        .order(name)
        .load::<RoomGroup>(db)
}

/// One fully-formed row of the roomgroups (Buildings) data table: the roomgroup plus the display
/// name of the user who last modified it.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct RoomGroupRow {
    pub roomgroupid: Uuid,
    pub tournamentid: Uuid,
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
    pub notes: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_date: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub last_modified_date: DateTime<Utc>,
    pub last_modified_user_name: String,
    pub last_modified_user_id: Uuid,
}

/// Returns one page of enriched roomgroup (Buildings) rows for the tournament plus the total count.
pub fn read_roomgroup_rows_of_tournament(
    db: &mut database::Connection,
    tournament_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<(Vec<RoomGroupRow>, i64)> {
    let total: i64 = {
        use crate::schema::roomgroups::dsl::*;
        roomgroups.filter(tournamentid.eq(tournament_id)).filter(del_fl.eq(false)).count().get_result(db)?
    };
    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;
    let list: Vec<RoomGroup> = {
        use crate::schema::roomgroups::dsl::*;
        roomgroups
            .filter(tournamentid.eq(tournament_id))
            .filter(del_fl.eq(false))
            .order(name.asc())
            .limit(page_size)
            .offset(offset_val)
            .load::<RoomGroup>(db)?
    };
    let name_ids: Vec<Uuid> = list.iter().map(|r| r.last_modified_userid).collect();
    let name_by_id: HashMap<Uuid, String> = crate::models::user::read_display_names(db, &name_ids)?;
    let rows = list
        .into_iter()
        .map(|r| RoomGroupRow {
            roomgroupid: r.roomgroupid,
            tournamentid: r.tournamentid,
            type_: r.type_,
            name: r.name,
            notes: r.notes,
            created_date: r.created_date,
            last_modified_date: r.last_modified_date,
            last_modified_user_name: name_by_id
                .get(&r.last_modified_userid)
                .cloned()
                .unwrap_or_else(|| r.last_modified_userid.to_string()),
            last_modified_user_id: r.last_modified_userid,
        })
        .collect();
    Ok((rows, total))
}

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &RoomGroupChangeset, modified_by: Uuid) -> QueryResult<RoomGroup> {
    use crate::schema::roomgroups::dsl::*;
    diesel::update(roomgroups.filter(roomgroupid.eq(item_id)))
        .set((
            item,
            last_modified_date.eq(diesel::dsl::now),
            last_modified_userid.eq(modified_by),
        ))
        .get_result(db)
}

/// Soft delete: hide the roomgroup by setting its `del_fl`.
pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::roomgroups::dsl::*;
    diesel::update(roomgroups.filter(roomgroupid.eq(item_id)))
        .set(del_fl.eq(true))
        .execute(db)
}

/// Purge: permanently remove the roomgroup row from the database.
pub fn purge(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::roomgroups::dsl::*;
    diesel::delete(roomgroups.filter(roomgroupid.eq(item_id))).execute(db)
}
