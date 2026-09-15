
use crate::database;
use crate::models::common::PaginationParams;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{DateTime, TimeZone, Utc};
use std::collections::HashMap;

pub struct RoundBuilder {
    pub division_session_id: Option<Uuid>,             // id of the associated (time-bound) division session
    pub name: Option<String>,
    pub scheduled_start_time: Option<DateTime<Utc>>,
    pub last_modified_user: Option<Uuid>
}

impl RoundBuilder {
    pub fn new(division_session_id: Uuid) -> Self {
        Self {
            division_session_id: Some(division_session_id),
            name: None,
            scheduled_start_time: None,
            last_modified_user: None
        }
    }
    pub fn new_default(division_session_id: Uuid) -> Self {
        Self {
            division_session_id: Some(division_session_id),
            name: None,
            scheduled_start_time: Some(Utc.with_ymd_and_hms(2055, 5, 23, 00, 00, 0).unwrap()),
            last_modified_user: None
        }
    }
    pub fn set_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
    pub fn set_scheduled_start_time(mut self, time: DateTime<Utc>) -> Self {
        self.scheduled_start_time = Some(time);
        self
    }
    pub fn set_last_modified_user(mut self, user_id: Uuid) -> Self {
        self.last_modified_user = Some(user_id);
        self
    }
    fn validate_all_are_some(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if self.division_session_id.is_none() {
            errors.push("division_session_id is required".to_string());
        }
        if self.name.is_none() {
            errors.push("name is required".to_string());
        }
        if self.scheduled_start_time.is_none() {
            errors.push("scheduled_start_time is required".to_string());
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
    pub fn build(self) -> Result<NewRound, Vec<String>> {
        match self.validate_all_are_some() {
            Err(e) => {
                Err(e)
            },
            Ok(_) => {
                Ok(
                    NewRound {
                        division_session_id: self.division_session_id.unwrap(),
                        name: self.name.unwrap(),
                        scheduled_start_time: self.scheduled_start_time,
                        last_modified_user: self.last_modified_user.unwrap_or_else(Uuid::nil)
                    }
                )
            }
        }
    }
    pub fn build_and_insert(mut self, db: &mut database::Connection) -> QueryResult<Round> {
        // For seed/test convenience: if no modifier was set, attribute it to the tournament owner
        // (session -> division -> tournament).
        if self.last_modified_user.is_none() {
            if let Some(dsid) = self.division_session_id {
                if let Ok(session) = crate::models::division_session::read(db, dsid) {
                    if let Ok(division) = crate::models::division::read(db, session.did) {
                        if let Ok(tournament) = crate::models::tournament::read(db, division.tid) {
                            self.last_modified_user = Some(tournament.owner_id);
                        }
                    }
                }
            }
        }
        let new_round = self.build();
        create(db, &new_round.unwrap())
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
#[diesel(table_name = crate::schema::rounds)]
#[diesel(primary_key(roundid))]
pub struct Round {
    pub roundid: Uuid,                          // identifies the round uniquely
    pub scheduled_start_time: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub scheduled_question_one_id: Option<Uuid>,
    pub scheduled_question_two_id: Option<Uuid>,
    pub scheduled_question_three_id: Option<Uuid>,
    pub scheduled_question_four_id: Option<Uuid>,
    pub scheduled_question_five_id: Option<Uuid>,
    pub scheduled_question_six_id: Option<Uuid>,
    pub scheduled_question_seven_id: Option<Uuid>,
    pub scheduled_question_eight_id: Option<Uuid>,
    pub scheduled_question_nine_id: Option<Uuid>,
    pub scheduled_question_ten_id: Option<Uuid>,
    pub scheduled_question_eleven_id: Option<Uuid>,
    pub scheduled_question_twelve_id: Option<Uuid>,
    pub scheduled_question_thirteen_id: Option<Uuid>,
    pub scheduled_question_fourteen_id: Option<Uuid>,
    pub scheduled_question_fifteen_id: Option<Uuid>,
    pub scheduled_question_sixteen_id: Option<Uuid>,
    pub scheduled_question_seventeen_id: Option<Uuid>,
    pub scheduled_question_eighteen_id: Option<Uuid>,
    pub scheduled_question_nineteen_id: Option<Uuid>,
    pub scheduled_question_twenty_id: Option<Uuid>,
    pub name: String,
    pub last_modified_user: Uuid,
    /// Soft-delete flag. When true the round is treated as deleted and excluded from reads.
    pub del_fl: bool,
    pub division_session_id: Uuid,              // id of the associated (time-bound) division session
}

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug
)]
#[diesel(table_name = crate::schema::rounds)]
pub struct NewRound {
    pub division_session_id: Uuid,             // id of the associated (time-bound) division session
    pub name: String,
    #[serde(default)]
    pub scheduled_start_time: Option<DateTime<Utc>>,
    #[serde(default)]
    pub last_modified_user: Uuid
}

// #[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::rounds)]
#[diesel(primary_key(roundid))]
pub struct RoundChangeset {
    pub name: Option<String>,
    pub scheduled_start_time: Option<DateTime<Utc>>
}

pub fn create(db: &mut database::Connection, item: &NewRound) -> QueryResult<Round> {
    use crate::schema::rounds::dsl::*;
    insert_into(rounds).values(item).get_result::<Round>(db)
}

pub fn exists(db: &mut database::Connection, roundid_val: Uuid) -> bool {
    use crate::schema::rounds::dsl::*;
    rounds
        .find(roundid_val)
        .filter(del_fl.eq(false))
        .get_result::<Round>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<Round> {
    use crate::schema::rounds::dsl::*;
    rounds.filter(roundid.eq(item_id)).filter(del_fl.eq(false)).first::<Round>(db)
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<Round>> {
    use crate::schema::rounds::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    rounds
        .filter(del_fl.eq(false))
        .order(created_at)
        .limit(page_size)
        .offset(offset_val)
        .load::<Round>(db)
}

/// Rounds belonging directly to a division session (the new, time-bound relationship).
pub fn read_all_rounds_of_division_session(
    db: &mut database::Connection,
    session_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<Vec<Round>> {
    use crate::schema::rounds::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    rounds
        .filter(division_session_id.eq(session_id))
        .filter(del_fl.eq(false))
        .order(scheduled_start_time.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<Round>(db)
}

/// Rounds across all of a division's sessions (a round now belongs to a session, not a division).
pub fn read_all_rounds_of_division(
    db: &mut database::Connection,
    division_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<Vec<Round>> {
    use crate::schema::{rounds, division_sessions};

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    rounds::table
        .inner_join(division_sessions::table.on(rounds::division_session_id.eq(division_sessions::division_session_id)))
        .filter(division_sessions::did.eq(division_id))
        .filter(rounds::del_fl.eq(false))
        .order(rounds::scheduled_start_time.asc())
        .limit(page_size)
        .offset(offset_val)
        .select(rounds::all_columns)
        .load::<Round>(db)
}

pub fn read_all_rounds_of_tournament(
    db: &mut database::Connection,
    tour_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<Vec<Round>> {
    use crate::schema::{rounds, division_sessions, divisions};

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    rounds::table
        .inner_join(division_sessions::table.on(rounds::division_session_id.eq(division_sessions::division_session_id)))
        .inner_join(divisions::table.on(division_sessions::did.eq(divisions::did)))
        .filter(divisions::tid.eq(tour_id))
        .filter(rounds::del_fl.eq(false))
        .order(rounds::scheduled_start_time.asc())
        .limit(page_size)
        .offset(offset_val)
        .select(rounds::all_columns)
        .load::<Round>(db)
}

/// One fully-formed row of the rounds data table: the round plus its session and division names, so
/// the whole table is populated from a single API call.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct RoundRow {
    pub roundid: Uuid,
    pub division_session_id: Uuid,
    pub session_name: String,
    pub did: Uuid,
    pub division_name: String,
    pub name: String,
    #[schema(value_type = Option<String>, format = DateTime)]
    pub scheduled_start_time: Option<DateTime<Utc>>,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Utc>,
    pub last_modified_user_name: String,
    pub last_modified_user_id: Uuid,
}

/// Returns one page of round-table rows for the tournament (enriched) and the total round count,
/// ordered by scheduled start time.
pub fn read_round_rows_of_tournament(
    db: &mut database::Connection,
    tour_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<(Vec<RoundRow>, i64)> {
    use crate::schema::{rounds, division_sessions, divisions};

    let total: i64 = rounds::table
        .inner_join(division_sessions::table.on(rounds::division_session_id.eq(division_sessions::division_session_id)))
        .inner_join(divisions::table.on(division_sessions::did.eq(divisions::did)))
        .filter(divisions::tid.eq(tour_id))
        .filter(rounds::del_fl.eq(false))
        .count()
        .get_result(db)?;

    let round_list = read_all_rounds_of_tournament(db, tour_id, pagination)?;
    Ok((build_round_rows(db, round_list)?, total))
}

/// Returns one page of round-table rows for the division (across its sessions) and the total count.
pub fn read_round_rows_of_division(
    db: &mut database::Connection,
    division_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<(Vec<RoundRow>, i64)> {
    use crate::schema::{rounds, division_sessions};

    let total: i64 = rounds::table
        .inner_join(division_sessions::table.on(rounds::division_session_id.eq(division_sessions::division_session_id)))
        .filter(division_sessions::did.eq(division_id))
        .filter(rounds::del_fl.eq(false))
        .count()
        .get_result(db)?;

    let round_list = read_all_rounds_of_division(db, division_id, pagination)?;
    Ok((build_round_rows(db, round_list)?, total))
}

/// Returns one page of round-table rows for a single division session and the total count.
pub fn read_round_rows_of_division_session(
    db: &mut database::Connection,
    session_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<(Vec<RoundRow>, i64)> {
    let total: i64 = {
        use crate::schema::rounds::dsl::*;
        rounds.filter(division_session_id.eq(session_id)).filter(del_fl.eq(false)).count().get_result(db)?
    };
    let round_list = read_all_rounds_of_division_session(db, session_id, pagination)?;
    Ok((build_round_rows(db, round_list)?, total))
}

/// Attaches each round's session name plus its (derived) division id and name.
fn build_round_rows(db: &mut database::Connection, round_list: Vec<Round>) -> QueryResult<Vec<RoundRow>> {
    // Session (name + owning division) for every round's session.
    let session_ids: Vec<Uuid> = round_list.iter().map(|r| r.division_session_id).collect();
    let session_info: HashMap<Uuid, (String, Uuid)> = {
        use crate::schema::division_sessions::dsl::*;
        division_sessions
            .filter(division_session_id.eq_any(&session_ids))
            .select((division_session_id, name, did))
            .load::<(Uuid, String, Uuid)>(db)?
            .into_iter()
            .map(|(id, sname, d)| (id, (sname, d)))
            .collect()
    };
    let did_list: Vec<Uuid> = session_info.values().map(|(_, d)| *d).collect();
    let div_name_by_id: HashMap<Uuid, String> = {
        use crate::schema::divisions::dsl::*;
        divisions.filter(did.eq_any(&did_list)).select((did, dname)).load::<(Uuid, String)>(db)?.into_iter().collect()
    };

    let name_ids: Vec<Uuid> = round_list.iter().map(|r| r.last_modified_user).collect();
    let name_by_id = crate::models::user::read_display_names(db, &name_ids)?;

    Ok(round_list
        .into_iter()
        .map(|r| {
            let (session_name, did) = session_info.get(&r.division_session_id).cloned().unwrap_or_default();
            let division_name = div_name_by_id.get(&did).cloned().unwrap_or_default();
            RoundRow {
                roundid: r.roundid,
                division_session_id: r.division_session_id,
                session_name,
                did,
                division_name,
                name: r.name,
                scheduled_start_time: r.scheduled_start_time,
                created_at: r.created_at,
                updated_at: r.updated_at,
                last_modified_user_name: name_by_id
                    .get(&r.last_modified_user)
                    .cloned()
                    .unwrap_or_else(|| r.last_modified_user.to_string()),
                last_modified_user_id: r.last_modified_user,
            }
        })
        .collect())
}

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &RoundChangeset, modified_by: Uuid) -> QueryResult<Round> {
    use crate::schema::rounds::dsl::*;
    diesel::update(rounds.filter(roundid.eq(item_id)))
        .set((
            item,
            updated_at.eq(diesel::dsl::now),
            last_modified_user.eq(modified_by),
        ))
        .get_result(db)
}

pub fn count(db: &mut database::Connection) -> QueryResult<i64> {
    use crate::schema::rounds::dsl::*;
    rounds.filter(del_fl.eq(false)).count().get_result(db)
}

/// Soft delete: mark the round deleted (excluded from reads) without removing the row.
pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::rounds::dsl::*;
    diesel::update(rounds.filter(roundid.eq(item_id)))
        .set(del_fl.eq(true))
        .execute(db)
}

/// Purge: permanently remove the round row from the database.
pub fn purge(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::rounds::dsl::*;
    diesel::delete(rounds.filter(roundid.eq(item_id))).execute(db)
}
