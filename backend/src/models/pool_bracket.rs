use crate::database;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult, AsChangeset, Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

pub struct PoolBracketBuilder {
    division_session_id: Uuid,
    name: Option<String>,
    type_: String,
    creator_userid: Option<Uuid>,
    last_modified_userid: Option<Uuid>,
}

impl PoolBracketBuilder {
    pub fn new(division_session_id: Uuid) -> Self {
        Self {
            division_session_id,
            name: None,
            type_: "pool".to_string(),
            creator_userid: None,
            last_modified_userid: None,
        }
    }
    pub fn new_default(division_session_id: Uuid) -> Self {
        Self::new(division_session_id)
    }
    pub fn set_division_session_id(mut self, id: Uuid) -> Self {
        self.division_session_id = id;
        self
    }
    pub fn set_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
    pub fn set_type(mut self, type_: &str) -> Self {
        self.type_ = type_.to_string();
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
    pub fn build(self) -> Result<NewPoolBracket, Vec<String>> {
        let mut errors = Vec::new();
        if self.name.is_none() { errors.push("name is required".to_string()); }
        if self.creator_userid.is_none() { errors.push("creator_userid is required".to_string()); }
        if !errors.is_empty() { return Err(errors); }
        let creator = self.creator_userid.unwrap();
        Ok(NewPoolBracket {
            division_session_id: self.division_session_id,
            name: self.name.unwrap(),
            type_: self.type_,
            creator_userid: creator,
            last_modified_userid: self.last_modified_userid.unwrap_or(creator),
        })
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<PoolBracket> {
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
#[diesel(table_name = crate::schema::pool_brackets)]
#[diesel(primary_key(pool_bracket_id))]
pub struct PoolBracket {
    pub pool_bracket_id: Uuid,                // identifies the pool bracket uniquely
    pub division_session_id: Uuid,            // parent division session
    #[diesel(column_name = type_)]
    #[serde(rename = "type")]
    pub type_: String,                        // grouping type (e.g. "pool"); required, defaults to "pool"
    pub created_date: DateTime<Utc>,
    pub creator_userid: Uuid,
    pub last_modified_date: DateTime<Utc>,
    pub last_modified_userid: Uuid,
    pub name: String,                         // unique within the parent division session
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::pool_brackets)]
pub struct NewPoolBracket {
    pub division_session_id: Uuid,
    pub name: String,
    #[diesel(column_name = type_)]
    #[serde(rename = "type")]
    pub type_: String,
    pub creator_userid: Uuid,
    pub last_modified_userid: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::pool_brackets)]
#[diesel(primary_key(pool_bracket_id))]
pub struct PoolBracketChangeset {
    pub division_session_id: Option<Uuid>,
    pub name: Option<String>,
    #[diesel(column_name = type_)]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

/// Whether a pool bracket named `name_val` already exists in division session `session_id`. When
/// `exclude` is set (e.g. during an update), that bracket id is ignored so a row doesn't clash with
/// itself.
pub fn name_exists_in_division_session(
    db: &mut database::Connection,
    session_id: Uuid,
    name_val: &str,
    exclude: Option<Uuid>,
) -> QueryResult<bool> {
    use crate::schema::pool_brackets::dsl::*;
    let mut query = pool_brackets
        .filter(division_session_id.eq(session_id))
        .filter(name.eq(name_val))
        .into_boxed();
    if let Some(ex) = exclude {
        query = query.filter(pool_bracket_id.ne(ex));
    }
    let count: i64 = query.count().get_result(db)?;
    Ok(count > 0)
}

pub fn create(db: &mut database::Connection, item: &NewPoolBracket) -> QueryResult<PoolBracket> {
    // A bracket's name must be unique within its parent division session.
    if name_exists_in_division_session(db, item.division_session_id, &item.name, None)? {
        return Err(diesel::result::Error::QueryBuilderError(
            format!("A pool bracket named \"{}\" already exists in this division session.", item.name).into()
        ));
    }
    use crate::schema::pool_brackets::dsl::*;
    insert_into(pool_brackets).values(item).get_result::<PoolBracket>(db)
}

pub fn exists(db: &mut database::Connection, item_id: Uuid) -> bool {
    use crate::schema::pool_brackets::dsl::*;
    pool_brackets.find(item_id).get_result::<PoolBracket>(db).is_ok()
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<PoolBracket> {
    use crate::schema::pool_brackets::dsl::*;
    pool_brackets.filter(pool_bracket_id.eq(item_id)).first::<PoolBracket>(db)
}

pub fn read_all(db: &mut database::Connection) -> QueryResult<Vec<PoolBracket>> {
    use crate::schema::pool_brackets::dsl::*;
    pool_brackets.order(created_date).load::<PoolBracket>(db)
}

/// All pool brackets belonging to the given division session.
pub fn read_all_of_division_session(db: &mut database::Connection, session_id: Uuid) -> QueryResult<Vec<PoolBracket>> {
    use crate::schema::pool_brackets::dsl::*;
    pool_brackets.filter(division_session_id.eq(session_id)).order(created_date).load::<PoolBracket>(db)
}

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &PoolBracketChangeset, modified_by: Uuid) -> QueryResult<PoolBracket> {
    // Enforce name uniqueness within the (possibly changed) parent division session.
    let existing = read(db, item_id)?;
    let effective_session = item.division_session_id.unwrap_or(existing.division_session_id);
    let effective_name = item.name.clone().unwrap_or(existing.name.clone());
    if name_exists_in_division_session(db, effective_session, &effective_name, Some(item_id))? {
        return Err(diesel::result::Error::QueryBuilderError(
            format!("A pool bracket named \"{}\" already exists in this division session.", effective_name).into()
        ));
    }
    use crate::schema::pool_brackets::dsl::*;
    diesel::update(pool_brackets.filter(pool_bracket_id.eq(item_id)))
        .set((
            item,
            last_modified_date.eq(diesel::dsl::now),
            last_modified_userid.eq(modified_by),
        ))
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::pool_brackets::dsl::*;
    diesel::delete(pool_brackets.filter(pool_bracket_id.eq(item_id))).execute(db)
}
