use crate::database;
use crate::models::common::PaginationParams;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult, AsChangeset, Insertable};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

pub struct RoundGroupBuilder {
    did: Uuid,
    name: Option<String>,
    creator_userid: Option<Uuid>,
    last_modified_userid: Option<Uuid>,
}

impl RoundGroupBuilder {
    pub fn new(did: Uuid) -> Self {
        Self { did, name: None, creator_userid: None, last_modified_userid: None }
    }
    pub fn new_default(did: Uuid) -> Self {
        Self::new(did)
    }
    pub fn set_did(mut self, did: Uuid) -> Self {
        self.did = did;
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
    pub fn build(self) -> Result<NewRoundGroup, Vec<String>> {
        let mut errors = Vec::new();
        if self.name.is_none() { errors.push("name is required".to_string()); }
        if self.creator_userid.is_none() { errors.push("creator_userid is required".to_string()); }
        if !errors.is_empty() { return Err(errors); }
        let creator = self.creator_userid.unwrap();
        Ok(NewRoundGroup {
            did: self.did,
            name: self.name.unwrap(),
            creator_userid: creator,
            last_modified_userid: self.last_modified_userid.unwrap_or(creator),
        })
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<RoundGroup> {
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
#[diesel(table_name = crate::schema::roundgroups)]
#[diesel(primary_key(roundgroup_id))]
pub struct RoundGroup {
    pub roundgroup_id: Uuid,            // identifies the division roundgroup uniquely
    pub did: Uuid,                            // parent division
    pub created_date: DateTime<Utc>,
    pub creator_userid: Uuid,
    pub last_modified_date: DateTime<Utc>,
    pub last_modified_userid: Uuid,
    pub name: String,                         // unique within the parent division
    pub del_fl: bool,                         // soft-delete flag
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::roundgroups)]
pub struct NewRoundGroup {
    pub did: Uuid,
    pub name: String,
    // Set from the authenticated user in the service layer; API payloads omit these.
    #[serde(default)]
    pub creator_userid: Uuid,
    #[serde(default)]
    pub last_modified_userid: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::roundgroups)]
#[diesel(primary_key(roundgroup_id))]
pub struct RoundGroupChangeset {
    pub did: Option<Uuid>,
    pub name: Option<String>,
}

/// Whether a roundgroup named `name_val` already exists in division `division_id`. When `exclude` is
/// set (e.g. during an update), that roundgroup id is ignored so a row doesn't clash with itself.
pub fn name_exists_in_division(
    db: &mut database::Connection,
    division_id: Uuid,
    name_val: &str,
    exclude: Option<Uuid>,
) -> QueryResult<bool> {
    use crate::schema::roundgroups::dsl::*;
    let mut query = roundgroups
        .filter(did.eq(division_id))
        .filter(name.eq(name_val))
        .filter(del_fl.eq(false))
        .into_boxed();
    if let Some(ex) = exclude {
        query = query.filter(roundgroup_id.ne(ex));
    }
    let count: i64 = query.count().get_result(db)?;
    Ok(count > 0)
}

pub fn create(db: &mut database::Connection, item: &NewRoundGroup) -> QueryResult<RoundGroup> {
    // A roundgroup's name must be unique within its parent division.
    if name_exists_in_division(db, item.did, &item.name, None)? {
        return Err(diesel::result::Error::QueryBuilderError(
            format!("A division roundgroup named \"{}\" already exists in this division.", item.name).into()
        ));
    }
    use crate::schema::roundgroups::dsl::*;
    insert_into(roundgroups).values(item).get_result::<RoundGroup>(db)
}

pub fn exists(db: &mut database::Connection, item_id: Uuid) -> bool {
    use crate::schema::roundgroups::dsl::*;
    roundgroups.find(item_id).filter(del_fl.eq(false)).get_result::<RoundGroup>(db).is_ok()
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<RoundGroup> {
    use crate::schema::roundgroups::dsl::*;
    roundgroups.filter(roundgroup_id.eq(item_id)).filter(del_fl.eq(false)).first::<RoundGroup>(db)
}

/// Read ignoring the soft-delete flag — used by purge, which must resolve even a soft-deleted row.
pub fn read_including_deleted(db: &mut database::Connection, item_id: Uuid) -> QueryResult<RoundGroup> {
    use crate::schema::roundgroups::dsl::*;
    roundgroups.filter(roundgroup_id.eq(item_id)).first::<RoundGroup>(db)
}

pub fn read_all(db: &mut database::Connection) -> QueryResult<Vec<RoundGroup>> {
    use crate::schema::roundgroups::dsl::*;
    roundgroups.filter(del_fl.eq(false)).order(created_date).load::<RoundGroup>(db)
}

/// All roundgroups belonging to the given division.
pub fn read_all_of_division(db: &mut database::Connection, division_id: Uuid) -> QueryResult<Vec<RoundGroup>> {
    use crate::schema::roundgroups::dsl::*;
    roundgroups.filter(did.eq(division_id)).filter(del_fl.eq(false)).order(created_date).load::<RoundGroup>(db)
}

/// One fully-formed row of the roundgroups data table: the roundgroup plus its division name and the
/// display name of the user who last modified it, so the whole table is populated in a single call.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct RoundGroupRow {
    pub roundgroup_id: Uuid,
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

/// Returns one page of roundgroup-table rows for the division (enriched) and the total roundgroup count.
pub fn read_roundgroup_rows_of_division(
    db: &mut database::Connection,
    division_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<(Vec<RoundGroupRow>, i64)> {
    let dname_val: String = {
        use crate::schema::divisions::dsl::*;
        divisions.filter(did.eq(division_id)).select(dname).first::<String>(db)?
    };

    let total: i64 = {
        use crate::schema::roundgroups::dsl::*;
        roundgroups.filter(did.eq(division_id)).filter(del_fl.eq(false)).count().get_result(db)?
    };

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;
    let roundgroup_list: Vec<RoundGroup> = {
        use crate::schema::roundgroups::dsl::*;
        roundgroups
            .filter(did.eq(division_id))
            .filter(del_fl.eq(false))
            .order(created_date.asc())
            .limit(page_size)
            .offset(offset_val)
            .load::<RoundGroup>(db)?
    };

    let name_ids: Vec<Uuid> = roundgroup_list.iter().map(|s| s.last_modified_userid).collect();
    let name_by_id: HashMap<Uuid, String> = crate::models::user::read_display_names(db, &name_ids)?;

    let rows = roundgroup_list
        .into_iter()
        .map(|s| RoundGroupRow {
            roundgroup_id: s.roundgroup_id,
            did: s.did,
            division_name: dname_val.clone(),
            name: s.name,
            created_date: s.created_date,
            last_modified_date: s.last_modified_date,
            last_modified_user_name: name_by_id
                .get(&s.last_modified_userid)
                .cloned()
                .unwrap_or_else(|| s.last_modified_userid.to_string()),
            last_modified_user_id: s.last_modified_userid,
        })
        .collect();

    Ok((rows, total))
}

/// Returns one page of roundgroup-table rows for the whole tournament (enriched), across every
/// division, plus the total roundgroup count.
pub fn read_roundgroup_rows_of_tournament(
    db: &mut database::Connection,
    tournament_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<(Vec<RoundGroupRow>, i64)> {
    // Division id -> name for the tournament's divisions.
    let div_pairs: Vec<(Uuid, String)> = {
        use crate::schema::divisions::dsl::*;
        divisions.filter(tid.eq(tournament_id)).select((did, dname)).load::<(Uuid, String)>(db)?
    };
    let div_ids: Vec<Uuid> = div_pairs.iter().map(|(d, _)| *d).collect();
    let div_name_by_id: HashMap<Uuid, String> = div_pairs.into_iter().collect();

    if div_ids.is_empty() {
        return Ok((Vec::new(), 0));
    }

    let total: i64 = {
        use crate::schema::roundgroups::dsl::*;
        roundgroups.filter(did.eq_any(&div_ids)).filter(del_fl.eq(false)).count().get_result(db)?
    };

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;
    let roundgroup_list: Vec<RoundGroup> = {
        use crate::schema::roundgroups::dsl::*;
        roundgroups
            .filter(did.eq_any(&div_ids))
            .filter(del_fl.eq(false))
            .order(created_date.asc())
            .limit(page_size)
            .offset(offset_val)
            .load::<RoundGroup>(db)?
    };

    let name_ids: Vec<Uuid> = roundgroup_list.iter().map(|s| s.last_modified_userid).collect();
    let name_by_id: HashMap<Uuid, String> = crate::models::user::read_display_names(db, &name_ids)?;

    let rows = roundgroup_list
        .into_iter()
        .map(|s| RoundGroupRow {
            roundgroup_id: s.roundgroup_id,
            division_name: div_name_by_id.get(&s.did).cloned().unwrap_or_default(),
            did: s.did,
            name: s.name,
            created_date: s.created_date,
            last_modified_date: s.last_modified_date,
            last_modified_user_name: name_by_id
                .get(&s.last_modified_userid)
                .cloned()
                .unwrap_or_else(|| s.last_modified_userid.to_string()),
            last_modified_user_id: s.last_modified_userid,
        })
        .collect();

    Ok((rows, total))
}

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &RoundGroupChangeset, modified_by: Uuid) -> QueryResult<RoundGroup> {
    // Enforce name uniqueness within the (possibly changed) parent division.
    let existing = read(db, item_id)?;
    let effective_did = item.did.unwrap_or(existing.did);
    let effective_name = item.name.clone().unwrap_or(existing.name.clone());
    if name_exists_in_division(db, effective_did, &effective_name, Some(item_id))? {
        return Err(diesel::result::Error::QueryBuilderError(
            format!("A division roundgroup named \"{}\" already exists in this division.", effective_name).into()
        ));
    }
    use crate::schema::roundgroups::dsl::*;
    diesel::update(roundgroups.filter(roundgroup_id.eq(item_id)))
        .set((
            item,
            last_modified_date.eq(diesel::dsl::now),
            last_modified_userid.eq(modified_by),
        ))
        .get_result(db)
}

/// Soft delete: hide the roundgroup by setting its `del_fl`.
pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::roundgroups::dsl::*;
    diesel::update(roundgroups.filter(roundgroup_id.eq(item_id)))
        .set(del_fl.eq(true))
        .execute(db)
}

/// Purge: permanently remove the roundgroup row from the database.
pub fn purge(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::roundgroups::dsl::*;
    diesel::delete(roundgroups.filter(roundgroup_id.eq(item_id))).execute(db)
}
