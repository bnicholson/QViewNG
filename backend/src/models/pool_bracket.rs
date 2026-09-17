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

pub struct PoolBracketBuilder {
    divisionid: Uuid,
    name: Option<String>,
    type_: String,
    creator_userid: Option<Uuid>,
    last_modified_userid: Option<Uuid>,
}

impl PoolBracketBuilder {
    pub fn new(divisionid: Uuid) -> Self {
        Self {
            divisionid,
            name: None,
            type_: "pool".to_string(),
            creator_userid: None,
            last_modified_userid: None,
        }
    }
    pub fn new_default(divisionid: Uuid) -> Self {
        Self::new(divisionid)
    }
    pub fn set_division_id(mut self, id: Uuid) -> Self {
        self.divisionid = id;
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
            divisionid: self.divisionid,
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
    #[diesel(column_name = type_)]
    #[serde(rename = "type")]
    pub type_: String,                        // grouping type (e.g. "pool"); required, defaults to "pool"
    pub created_date: DateTime<Utc>,
    pub creator_userid: Uuid,
    pub last_modified_date: DateTime<Utc>,
    pub last_modified_userid: Uuid,
    pub name: String,                         // unique within the parent division
    pub del_fl: bool,                         // soft-delete flag
    pub divisionid: Uuid,                     // parent division
    pub team_group_id: Option<Uuid>,          // 1-to-1 team group (UNIQUE); created lazily, so nullable
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::pool_brackets)]
pub struct NewPoolBracket {
    pub divisionid: Uuid,
    pub name: String,
    #[diesel(column_name = type_)]
    #[serde(rename = "type")]
    pub type_: String,
    // Set from the authenticated user in the service layer; API payloads omit these.
    #[serde(default)]
    pub creator_userid: Uuid,
    #[serde(default)]
    pub last_modified_userid: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::pool_brackets)]
#[diesel(primary_key(pool_bracket_id))]
pub struct PoolBracketChangeset {
    pub divisionid: Option<Uuid>,
    pub name: Option<String>,
    #[diesel(column_name = type_)]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

/// Whether a pool bracket named `name_val` already exists in division `division_id`. When `exclude`
/// is set (e.g. during an update), that bracket id is ignored so a row doesn't clash with itself.
pub fn name_exists_in_division(
    db: &mut database::Connection,
    division_id: Uuid,
    name_val: &str,
    exclude: Option<Uuid>,
) -> QueryResult<bool> {
    use crate::schema::pool_brackets::dsl::*;
    let mut query = pool_brackets
        .filter(divisionid.eq(division_id))
        .filter(name.eq(name_val))
        .filter(del_fl.eq(false))
        .into_boxed();
    if let Some(ex) = exclude {
        query = query.filter(pool_bracket_id.ne(ex));
    }
    let count: i64 = query.count().get_result(db)?;
    Ok(count > 0)
}

pub fn create(db: &mut database::Connection, item: &NewPoolBracket) -> QueryResult<PoolBracket> {
    // A bracket's name must be unique within its parent division.
    if name_exists_in_division(db, item.divisionid, &item.name, None)? {
        return Err(diesel::result::Error::QueryBuilderError(
            format!("A pool bracket named \"{}\" already exists in this division.", item.name).into()
        ));
    }
    use crate::schema::pool_brackets::dsl::*;
    insert_into(pool_brackets).values(item).get_result::<PoolBracket>(db)
}

pub fn exists(db: &mut database::Connection, item_id: Uuid) -> bool {
    use crate::schema::pool_brackets::dsl::*;
    pool_brackets.find(item_id).filter(del_fl.eq(false)).get_result::<PoolBracket>(db).is_ok()
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<PoolBracket> {
    use crate::schema::pool_brackets::dsl::*;
    pool_brackets.filter(pool_bracket_id.eq(item_id)).filter(del_fl.eq(false)).first::<PoolBracket>(db)
}

/// Read ignoring the soft-delete flag — used by purge, which must resolve even a soft-deleted row.
pub fn read_including_deleted(db: &mut database::Connection, item_id: Uuid) -> QueryResult<PoolBracket> {
    use crate::schema::pool_brackets::dsl::*;
    pool_brackets.filter(pool_bracket_id.eq(item_id)).first::<PoolBracket>(db)
}

pub fn read_all(db: &mut database::Connection) -> QueryResult<Vec<PoolBracket>> {
    use crate::schema::pool_brackets::dsl::*;
    pool_brackets.filter(del_fl.eq(false)).order(created_date).load::<PoolBracket>(db)
}

/// All pool brackets belonging to the given division.
pub fn read_all_of_division(db: &mut database::Connection, division_id: Uuid) -> QueryResult<Vec<PoolBracket>> {
    use crate::schema::pool_brackets::dsl::*;
    pool_brackets
        .filter(divisionid.eq(division_id))
        .filter(del_fl.eq(false))
        .order(created_date)
        .load::<PoolBracket>(db)
}

/// All pool brackets of the given division whose `type` matches `type_val`.
pub fn read_all_of_division_by_type(db: &mut database::Connection, division_id: Uuid, type_val: &str) -> QueryResult<Vec<PoolBracket>> {
    use crate::schema::pool_brackets::dsl::*;
    pool_brackets
        .filter(divisionid.eq(division_id))
        .filter(type_.eq(type_val))
        .filter(del_fl.eq(false))
        .order(created_date)
        .load::<PoolBracket>(db)
}

/// One fully-formed row of the pool-brackets data table: the bracket plus its parent division name
/// and the display name of the user who last modified it.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct PoolBracketRow {
    pub pool_bracket_id: Uuid,
    pub did: Uuid,
    pub division_name: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_date: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub last_modified_date: DateTime<Utc>,
    pub last_modified_user_name: String,
    pub last_modified_user_id: Uuid,
}

/// Assembles enriched pool-bracket rows from a page of brackets, given each bracket's division name
/// (looked up by the caller) and the id→(division id, division name) mapping.
fn build_pool_bracket_rows(
    db: &mut database::Connection,
    bracket_list: Vec<PoolBracket>,
    division_name_by_id: &HashMap<Uuid, String>,
) -> QueryResult<Vec<PoolBracketRow>> {
    let name_ids: Vec<Uuid> = bracket_list.iter().map(|b| b.last_modified_userid).collect();
    let name_by_id: HashMap<Uuid, String> = crate::models::user::read_display_names(db, &name_ids)?;

    Ok(bracket_list
        .into_iter()
        .map(|b| PoolBracketRow {
            pool_bracket_id: b.pool_bracket_id,
            did: b.divisionid,
            division_name: division_name_by_id.get(&b.divisionid).cloned().unwrap_or_default(),
            name: b.name,
            type_: b.type_,
            created_date: b.created_date,
            last_modified_date: b.last_modified_date,
            last_modified_user_name: name_by_id
                .get(&b.last_modified_userid)
                .cloned()
                .unwrap_or_else(|| b.last_modified_userid.to_string()),
            last_modified_user_id: b.last_modified_userid,
        })
        .collect())
}

/// Returns one page of pool-bracket-table rows for the division (enriched), filtered to `type_val`,
/// plus the total count for that type.
pub fn read_pool_bracket_rows_of_division(
    db: &mut database::Connection,
    division_id: Uuid,
    type_val: &str,
    pagination: &PaginationParams,
) -> QueryResult<(Vec<PoolBracketRow>, i64)> {
    let dname_val: String = {
        use crate::schema::divisions::dsl::*;
        divisions.filter(did.eq(division_id)).select(dname).first::<String>(db)?
    };

    let total: i64 = {
        use crate::schema::pool_brackets::dsl::*;
        pool_brackets
            .filter(divisionid.eq(division_id))
            .filter(type_.eq(type_val))
            .filter(del_fl.eq(false))
            .count()
            .get_result(db)?
    };

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;
    let bracket_list: Vec<PoolBracket> = {
        use crate::schema::pool_brackets::dsl::*;
        pool_brackets
            .filter(divisionid.eq(division_id))
            .filter(type_.eq(type_val))
            .filter(del_fl.eq(false))
            .order(created_date.asc())
            .limit(page_size)
            .offset(offset_val)
            .load::<PoolBracket>(db)?
    };

    let mut division_name_by_id = HashMap::new();
    division_name_by_id.insert(division_id, dname_val);
    let rows = build_pool_bracket_rows(db, bracket_list, &division_name_by_id)?;
    Ok((rows, total))
}

/// Returns one page of pool-bracket-table rows for a whole tournament (enriched), across every
/// division, filtered to `type_val`, plus the total count for that type.
pub fn read_pool_bracket_rows_of_tournament(
    db: &mut database::Connection,
    tournament_id: Uuid,
    type_val: &str,
    pagination: &PaginationParams,
) -> QueryResult<(Vec<PoolBracketRow>, i64)> {
    // Division id -> name for the tournament's divisions.
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
        use crate::schema::pool_brackets::dsl::*;
        pool_brackets
            .filter(divisionid.eq_any(&div_ids))
            .filter(type_.eq(type_val))
            .filter(del_fl.eq(false))
            .count()
            .get_result(db)?
    };

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;
    let bracket_list: Vec<PoolBracket> = {
        use crate::schema::pool_brackets::dsl::*;
        pool_brackets
            .filter(divisionid.eq_any(&div_ids))
            .filter(type_.eq(type_val))
            .filter(del_fl.eq(false))
            .order(created_date.asc())
            .limit(page_size)
            .offset(offset_val)
            .load::<PoolBracket>(db)?
    };

    let rows = build_pool_bracket_rows(db, bracket_list, &division_name_by_id)?;
    Ok((rows, total))
}

/// Returns a pool bracket id for the division, creating a default bracket if the division has none
/// yet. Used by programmatic game creation (e.g. seeds, tests) where no bracket was explicitly
/// chosen. The API create path never reaches this — it rejects a nil poolbracket_id.
pub fn resolve_default_for_division(db: &mut database::Connection, division_id: Uuid, user_id: Uuid) -> QueryResult<Uuid> {
    if let Some(existing) = read_all_of_division(db, division_id)?.into_iter().next() {
        return Ok(existing.pool_bracket_id);
    }
    // No brackets yet — create a default one for the division.
    let bracket = create(
        db,
        &NewPoolBracket {
            divisionid: division_id,
            name: "Default".to_string(),
            type_: "pool".to_string(),
            creator_userid: user_id,
            last_modified_userid: user_id,
        },
    )?;
    Ok(bracket.pool_bracket_id)
}

/// Attaches the bracket's 1-to-1 team group (the FK + UNIQUE now live on pool_brackets). Called when
/// a team group is first created for the bracket.
pub fn set_team_group_id(db: &mut database::Connection, bracket_id: Uuid, tgid: Uuid) -> QueryResult<usize> {
    use crate::schema::pool_brackets::dsl::*;
    diesel::update(pool_brackets.filter(pool_bracket_id.eq(bracket_id)))
        .set(team_group_id.eq(tgid))
        .execute(db)
}

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &PoolBracketChangeset, modified_by: Uuid) -> QueryResult<PoolBracket> {
    // Enforce name uniqueness within the (possibly changed) parent division.
    let existing = read(db, item_id)?;
    let effective_division = item.divisionid.unwrap_or(existing.divisionid);
    let effective_name = item.name.clone().unwrap_or(existing.name.clone());
    if name_exists_in_division(db, effective_division, &effective_name, Some(item_id))? {
        return Err(diesel::result::Error::QueryBuilderError(
            format!("A pool bracket named \"{}\" already exists in this division.", effective_name).into()
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

/// Soft delete: hide the pool bracket by setting its `del_fl`.
pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::pool_brackets::dsl::*;
    diesel::update(pool_brackets.filter(pool_bracket_id.eq(item_id)))
        .set(del_fl.eq(true))
        .execute(db)
}

/// Purge: permanently remove the pool bracket row from the database.
pub fn purge(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::pool_brackets::dsl::*;
    diesel::delete(pool_brackets.filter(pool_bracket_id.eq(item_id))).execute(db)
}
