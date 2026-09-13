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

pub struct DivisionSessionBuilder {
    did: Uuid,
    name: Option<String>,
    creator_userid: Option<Uuid>,
    last_modified_userid: Option<Uuid>,
}

impl DivisionSessionBuilder {
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
    pub fn build(self) -> Result<NewDivisionSession, Vec<String>> {
        let mut errors = Vec::new();
        if self.name.is_none() { errors.push("name is required".to_string()); }
        if self.creator_userid.is_none() { errors.push("creator_userid is required".to_string()); }
        if !errors.is_empty() { return Err(errors); }
        let creator = self.creator_userid.unwrap();
        Ok(NewDivisionSession {
            did: self.did,
            name: self.name.unwrap(),
            creator_userid: creator,
            last_modified_userid: self.last_modified_userid.unwrap_or(creator),
        })
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<DivisionSession> {
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
#[diesel(table_name = crate::schema::division_sessions)]
#[diesel(primary_key(division_session_id))]
pub struct DivisionSession {
    pub division_session_id: Uuid,            // identifies the division session uniquely
    pub did: Uuid,                            // parent division
    pub created_date: DateTime<Utc>,
    pub creator_userid: Uuid,
    pub last_modified_date: DateTime<Utc>,
    pub last_modified_userid: Uuid,
    pub name: String,                         // unique within the parent division
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::division_sessions)]
pub struct NewDivisionSession {
    pub did: Uuid,
    pub name: String,
    // Set from the authenticated user in the service layer; API payloads omit these.
    #[serde(default)]
    pub creator_userid: Uuid,
    #[serde(default)]
    pub last_modified_userid: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::division_sessions)]
#[diesel(primary_key(division_session_id))]
pub struct DivisionSessionChangeset {
    pub did: Option<Uuid>,
    pub name: Option<String>,
}

/// Whether a session named `name_val` already exists in division `division_id`. When `exclude` is
/// set (e.g. during an update), that session id is ignored so a row doesn't clash with itself.
pub fn name_exists_in_division(
    db: &mut database::Connection,
    division_id: Uuid,
    name_val: &str,
    exclude: Option<Uuid>,
) -> QueryResult<bool> {
    use crate::schema::division_sessions::dsl::*;
    let mut query = division_sessions
        .filter(did.eq(division_id))
        .filter(name.eq(name_val))
        .into_boxed();
    if let Some(ex) = exclude {
        query = query.filter(division_session_id.ne(ex));
    }
    let count: i64 = query.count().get_result(db)?;
    Ok(count > 0)
}

pub fn create(db: &mut database::Connection, item: &NewDivisionSession) -> QueryResult<DivisionSession> {
    // A session's name must be unique within its parent division.
    if name_exists_in_division(db, item.did, &item.name, None)? {
        return Err(diesel::result::Error::QueryBuilderError(
            format!("A division session named \"{}\" already exists in this division.", item.name).into()
        ));
    }
    use crate::schema::division_sessions::dsl::*;
    insert_into(division_sessions).values(item).get_result::<DivisionSession>(db)
}

pub fn exists(db: &mut database::Connection, item_id: Uuid) -> bool {
    use crate::schema::division_sessions::dsl::*;
    division_sessions.find(item_id).get_result::<DivisionSession>(db).is_ok()
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<DivisionSession> {
    use crate::schema::division_sessions::dsl::*;
    division_sessions.filter(division_session_id.eq(item_id)).first::<DivisionSession>(db)
}

pub fn read_all(db: &mut database::Connection) -> QueryResult<Vec<DivisionSession>> {
    use crate::schema::division_sessions::dsl::*;
    division_sessions.order(created_date).load::<DivisionSession>(db)
}

/// All sessions belonging to the given division.
pub fn read_all_of_division(db: &mut database::Connection, division_id: Uuid) -> QueryResult<Vec<DivisionSession>> {
    use crate::schema::division_sessions::dsl::*;
    division_sessions.filter(did.eq(division_id)).order(created_date).load::<DivisionSession>(db)
}

/// One fully-formed row of the sessions data table: the session plus its division name and the
/// display name of the user who last modified it, so the whole table is populated in a single call.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct DivisionSessionRow {
    pub division_session_id: Uuid,
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

/// Returns one page of session-table rows for the division (enriched) and the total session count.
pub fn read_session_rows_of_division(
    db: &mut database::Connection,
    division_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<(Vec<DivisionSessionRow>, i64)> {
    let dname_val: String = {
        use crate::schema::divisions::dsl::*;
        divisions.filter(did.eq(division_id)).select(dname).first::<String>(db)?
    };

    let total: i64 = {
        use crate::schema::division_sessions::dsl::*;
        division_sessions.filter(did.eq(division_id)).count().get_result(db)?
    };

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;
    let session_list: Vec<DivisionSession> = {
        use crate::schema::division_sessions::dsl::*;
        division_sessions
            .filter(did.eq(division_id))
            .order(created_date.asc())
            .limit(page_size)
            .offset(offset_val)
            .load::<DivisionSession>(db)?
    };

    let name_ids: Vec<Uuid> = session_list.iter().map(|s| s.last_modified_userid).collect();
    let name_by_id: HashMap<Uuid, String> = crate::models::user::read_display_names(db, &name_ids)?;

    let rows = session_list
        .into_iter()
        .map(|s| DivisionSessionRow {
            division_session_id: s.division_session_id,
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

/// Returns one page of session-table rows for the whole tournament (enriched), across every
/// division, plus the total session count.
pub fn read_session_rows_of_tournament(
    db: &mut database::Connection,
    tournament_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<(Vec<DivisionSessionRow>, i64)> {
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
        use crate::schema::division_sessions::dsl::*;
        division_sessions.filter(did.eq_any(&div_ids)).count().get_result(db)?
    };

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;
    let session_list: Vec<DivisionSession> = {
        use crate::schema::division_sessions::dsl::*;
        division_sessions
            .filter(did.eq_any(&div_ids))
            .order(created_date.asc())
            .limit(page_size)
            .offset(offset_val)
            .load::<DivisionSession>(db)?
    };

    let name_ids: Vec<Uuid> = session_list.iter().map(|s| s.last_modified_userid).collect();
    let name_by_id: HashMap<Uuid, String> = crate::models::user::read_display_names(db, &name_ids)?;

    let rows = session_list
        .into_iter()
        .map(|s| DivisionSessionRow {
            division_session_id: s.division_session_id,
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

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &DivisionSessionChangeset, modified_by: Uuid) -> QueryResult<DivisionSession> {
    // Enforce name uniqueness within the (possibly changed) parent division.
    let existing = read(db, item_id)?;
    let effective_did = item.did.unwrap_or(existing.did);
    let effective_name = item.name.clone().unwrap_or(existing.name.clone());
    if name_exists_in_division(db, effective_did, &effective_name, Some(item_id))? {
        return Err(diesel::result::Error::QueryBuilderError(
            format!("A division session named \"{}\" already exists in this division.", effective_name).into()
        ));
    }
    use crate::schema::division_sessions::dsl::*;
    diesel::update(division_sessions.filter(division_session_id.eq(item_id)))
        .set((
            item,
            last_modified_date.eq(diesel::dsl::now),
            last_modified_userid.eq(modified_by),
        ))
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::division_sessions::dsl::*;
    diesel::delete(division_sessions.filter(division_session_id.eq(item_id))).execute(db)
}
