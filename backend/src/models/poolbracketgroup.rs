use crate::database;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult, AsChangeset, Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

// A poolbracketgroup groups the pool_brackets that run concurrently within a division. Every team
// competes in exactly one pool per group; a later group may re-pool the same teams differently.
pub struct PoolBracketGroupBuilder {
    divisionid: Uuid,
    name: Option<String>,
    creator_userid: Option<Uuid>,
    last_modified_userid: Option<Uuid>,
}

impl PoolBracketGroupBuilder {
    pub fn new(divisionid: Uuid) -> Self {
        Self { divisionid, name: None, creator_userid: None, last_modified_userid: None }
    }
    pub fn new_default(divisionid: Uuid) -> Self {
        Self::new(divisionid)
    }
    pub fn set_divisionid(mut self, divisionid: Uuid) -> Self {
        self.divisionid = divisionid;
        self
    }
    pub fn set_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
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
    pub fn build(self) -> Result<NewPoolBracketGroup, Vec<String>> {
        let mut errors = Vec::new();
        if self.name.is_none() { errors.push("name is required".to_string()); }
        if self.creator_userid.is_none() { errors.push("creator_userid is required".to_string()); }
        if !errors.is_empty() { return Err(errors); }
        let creator = self.creator_userid.unwrap();
        Ok(NewPoolBracketGroup {
            divisionid: self.divisionid,
            name: self.name.unwrap(),
            creator_userid: creator,
            last_modified_userid: self.last_modified_userid.unwrap_or(creator),
        })
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<PoolBracketGroup> {
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
#[diesel(table_name = crate::schema::poolbracketgroups)]
#[diesel(primary_key(poolbracketgroupid))]
pub struct PoolBracketGroup {
    pub poolbracketgroupid: Uuid,             // identifies the pool bracket group uniquely
    pub divisionid: Uuid,                     // parent division
    pub name: String,                         // unique within the parent division
    pub created_date: DateTime<Utc>,
    pub creator_userid: Uuid,
    pub last_modified_date: DateTime<Utc>,
    pub last_modified_userid: Uuid,
    pub del_fl: bool,                         // soft-delete flag
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::poolbracketgroups)]
pub struct NewPoolBracketGroup {
    pub divisionid: Uuid,
    pub name: String,
    // Set from the authenticated user in the service layer; API payloads omit these.
    #[serde(default)]
    pub creator_userid: Uuid,
    #[serde(default)]
    pub last_modified_userid: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::poolbracketgroups)]
#[diesel(primary_key(poolbracketgroupid))]
pub struct PoolBracketGroupChangeset {
    pub divisionid: Option<Uuid>,
    pub name: Option<String>,
}

/// Whether a poolbracketgroup named `name_val` already exists in division `division_id`. When `exclude`
/// is set (e.g. during an update), that group id is ignored so a row doesn't clash with itself.
pub fn name_exists_in_division(
    db: &mut database::Connection,
    division_id: Uuid,
    name_val: &str,
    exclude: Option<Uuid>,
) -> QueryResult<bool> {
    use crate::schema::poolbracketgroups::dsl::*;
    let mut query = poolbracketgroups
        .filter(divisionid.eq(division_id))
        .filter(name.eq(name_val))
        .filter(del_fl.eq(false))
        .into_boxed();
    if let Some(ex) = exclude {
        query = query.filter(poolbracketgroupid.ne(ex));
    }
    let count: i64 = query.count().get_result(db)?;
    Ok(count > 0)
}

pub fn create(db: &mut database::Connection, item: &NewPoolBracketGroup) -> QueryResult<PoolBracketGroup> {
    // A group's name must be unique within its parent division.
    if name_exists_in_division(db, item.divisionid, &item.name, None)? {
        return Err(diesel::result::Error::QueryBuilderError(
            format!("A pool bracket group named \"{}\" already exists in this division.", item.name).into()
        ));
    }
    use crate::schema::poolbracketgroups::dsl::*;
    insert_into(poolbracketgroups).values(item).get_result::<PoolBracketGroup>(db)
}

pub fn exists(db: &mut database::Connection, item_id: Uuid) -> bool {
    use crate::schema::poolbracketgroups::dsl::*;
    poolbracketgroups.find(item_id).filter(del_fl.eq(false)).get_result::<PoolBracketGroup>(db).is_ok()
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<PoolBracketGroup> {
    use crate::schema::poolbracketgroups::dsl::*;
    poolbracketgroups.filter(poolbracketgroupid.eq(item_id)).filter(del_fl.eq(false)).first::<PoolBracketGroup>(db)
}

/// Read ignoring the soft-delete flag — used by purge, which must resolve even a soft-deleted row.
pub fn read_including_deleted(db: &mut database::Connection, item_id: Uuid) -> QueryResult<PoolBracketGroup> {
    use crate::schema::poolbracketgroups::dsl::*;
    poolbracketgroups.filter(poolbracketgroupid.eq(item_id)).first::<PoolBracketGroup>(db)
}

pub fn read_all(db: &mut database::Connection) -> QueryResult<Vec<PoolBracketGroup>> {
    use crate::schema::poolbracketgroups::dsl::*;
    poolbracketgroups.filter(del_fl.eq(false)).order(created_date).load::<PoolBracketGroup>(db)
}

/// All poolbracketgroups belonging to the given division.
pub fn read_all_of_division(db: &mut database::Connection, division_id: Uuid) -> QueryResult<Vec<PoolBracketGroup>> {
    use crate::schema::poolbracketgroups::dsl::*;
    poolbracketgroups.filter(divisionid.eq(division_id)).filter(del_fl.eq(false)).order(created_date).load::<PoolBracketGroup>(db)
}

/// Number of (non-deleted) pool brackets that belong to the group — used to guard deletion.
pub fn count_pool_brackets(db: &mut database::Connection, group_id: Uuid) -> QueryResult<i64> {
    use crate::schema::pool_brackets::dsl::*;
    pool_brackets
        .filter(poolbracketgroupid.eq(group_id))
        .filter(del_fl.eq(false))
        .count()
        .get_result(db)
}

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &PoolBracketGroupChangeset, modified_by: Uuid) -> QueryResult<PoolBracketGroup> {
    // Enforce name uniqueness within the (possibly changed) parent division.
    let existing = read(db, item_id)?;
    let effective_division = item.divisionid.unwrap_or(existing.divisionid);
    let effective_name = item.name.clone().unwrap_or(existing.name.clone());
    if name_exists_in_division(db, effective_division, &effective_name, Some(item_id))? {
        return Err(diesel::result::Error::QueryBuilderError(
            format!("A pool bracket group named \"{}\" already exists in this division.", effective_name).into()
        ));
    }
    use crate::schema::poolbracketgroups::dsl::*;
    diesel::update(poolbracketgroups.filter(poolbracketgroupid.eq(item_id)))
        .set((
            item,
            last_modified_date.eq(diesel::dsl::now),
            last_modified_userid.eq(modified_by),
        ))
        .get_result(db)
}

/// Soft delete: hide the group by setting its `del_fl`.
pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::poolbracketgroups::dsl::*;
    diesel::update(poolbracketgroups.filter(poolbracketgroupid.eq(item_id)))
        .set(del_fl.eq(true))
        .execute(db)
}

/// Purge: permanently remove the group row from the database.
pub fn purge(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::poolbracketgroups::dsl::*;
    diesel::delete(poolbracketgroups.filter(poolbracketgroupid.eq(item_id))).execute(db)
}
