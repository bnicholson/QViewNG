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

/// One fully-formed row of the poolbracketgroups data table: the group plus its division name and
/// the display name of the user who last modified it, so the whole table is populated in one call.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct PoolBracketGroupRow {
    pub poolbracketgroupid: Uuid,
    pub did: Uuid,
    pub division_name: String,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_date: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub last_modified_date: DateTime<Utc>,
    pub last_modified_user_name: String,
    pub last_modified_user_id: Uuid,
}

/// Assembles enriched rows from a page of groups, given the id→name mapping for their divisions.
fn build_poolbracketgroup_rows(
    db: &mut database::Connection,
    group_list: Vec<PoolBracketGroup>,
    division_name_by_id: &HashMap<Uuid, String>,
) -> QueryResult<Vec<PoolBracketGroupRow>> {
    let name_ids: Vec<Uuid> = group_list.iter().map(|g| g.last_modified_userid).collect();
    let name_by_id: HashMap<Uuid, String> = crate::models::user::read_display_names(db, &name_ids)?;

    Ok(group_list
        .into_iter()
        .map(|g| PoolBracketGroupRow {
            poolbracketgroupid: g.poolbracketgroupid,
            did: g.divisionid,
            division_name: division_name_by_id.get(&g.divisionid).cloned().unwrap_or_default(),
            name: g.name,
            created_date: g.created_date,
            last_modified_date: g.last_modified_date,
            last_modified_user_name: name_by_id
                .get(&g.last_modified_userid)
                .cloned()
                .unwrap_or_else(|| g.last_modified_userid.to_string()),
            last_modified_user_id: g.last_modified_userid,
        })
        .collect())
}

/// Returns one page of poolbracketgroup-table rows for the division (enriched) and the total count.
pub fn read_poolbracketgroup_rows_of_division(
    db: &mut database::Connection,
    division_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<(Vec<PoolBracketGroupRow>, i64)> {
    let dname_val: String = {
        use crate::schema::divisions::dsl::*;
        divisions.filter(did.eq(division_id)).select(dname).first::<String>(db)?
    };

    let total: i64 = {
        use crate::schema::poolbracketgroups::dsl::*;
        poolbracketgroups
            .filter(divisionid.eq(division_id))
            .filter(del_fl.eq(false))
            .count()
            .get_result(db)?
    };

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;
    let group_list: Vec<PoolBracketGroup> = {
        use crate::schema::poolbracketgroups::dsl::*;
        poolbracketgroups
            .filter(divisionid.eq(division_id))
            .filter(del_fl.eq(false))
            .order(created_date.asc())
            .limit(page_size)
            .offset(offset_val)
            .load::<PoolBracketGroup>(db)?
    };

    let mut division_name_by_id = HashMap::new();
    division_name_by_id.insert(division_id, dname_val);
    let rows = build_poolbracketgroup_rows(db, group_list, &division_name_by_id)?;
    Ok((rows, total))
}

/// Returns one page of poolbracketgroup-table rows for the whole tournament (enriched), across every
/// division, plus the total count.
pub fn read_poolbracketgroup_rows_of_tournament(
    db: &mut database::Connection,
    tournament_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<(Vec<PoolBracketGroupRow>, i64)> {
    let div_pairs: Vec<(Uuid, String)> = {
        use crate::schema::divisions::dsl::*;
        divisions.filter(tid.eq(tournament_id)).select((did, dname)).load::<(Uuid, String)>(db)?
    };
    let div_ids: Vec<Uuid> = div_pairs.iter().map(|(d, _)| *d).collect();
    let division_name_by_id: HashMap<Uuid, String> = div_pairs.into_iter().collect();

    if div_ids.is_empty() {
        return Ok((Vec::new(), 0));
    }

    let total: i64 = {
        use crate::schema::poolbracketgroups::dsl::*;
        poolbracketgroups
            .filter(divisionid.eq_any(&div_ids))
            .filter(del_fl.eq(false))
            .count()
            .get_result(db)?
    };

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;
    let group_list: Vec<PoolBracketGroup> = {
        use crate::schema::poolbracketgroups::dsl::*;
        poolbracketgroups
            .filter(divisionid.eq_any(&div_ids))
            .filter(del_fl.eq(false))
            .order(created_date.asc())
            .limit(page_size)
            .offset(offset_val)
            .load::<PoolBracketGroup>(db)?
    };

    let rows = build_poolbracketgroup_rows(db, group_list, &division_name_by_id)?;
    Ok((rows, total))
}
