use crate::database;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult, AsChangeset, Insertable};
use serde::{Deserialize, Serialize};
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
    pub creator_userid: Uuid,
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
