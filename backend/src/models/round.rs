
use crate::database;
use crate::models::common::PaginationParams;
use crate::models::division::Division;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{DateTime, TimeZone, Utc};
use std::collections::HashMap;

pub struct RoundBuilder {
    pub did: Option<Uuid>,                              // id of the associated division
    pub name: Option<String>,
    pub scheduled_start_time: Option<DateTime<Utc>>
}

impl RoundBuilder {
    pub fn new(did: Uuid) -> Self {
        Self {
            did: Some(did),
            name: None,
            scheduled_start_time: None
        }
    }
    pub fn new_default(did: Uuid) -> Self {
        Self {
            did: Some(did),
            name: None,
            scheduled_start_time: Some(Utc.with_ymd_and_hms(2055, 5, 23, 00, 00, 0).unwrap())
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
    fn validate_all_are_some(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if self.did.is_none() {
            errors.push("did is required".to_string());
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
                        did: self.did.unwrap(),
                        name: self.name.unwrap(),
                        scheduled_start_time: self.scheduled_start_time
                    }
                )
            }
        }
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<Round> {
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
    pub did: Uuid,                              // id of the associated division
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
}

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug
)]
#[diesel(table_name = crate::schema::rounds)]
pub struct NewRound {
    pub did: Uuid,                              // id of the associated division
    pub name: String,
    #[serde(default)]
    pub scheduled_start_time: Option<DateTime<Utc>>
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

pub fn exists(db: &mut database::Connection, roundid: Uuid) -> bool {
    use crate::schema::rounds::dsl::rounds;
    rounds
        .find(roundid)
        .get_result::<Round>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<Round> {
    use crate::schema::rounds::dsl::*;
    rounds.filter(roundid.eq(item_id)).first::<Round>(db)
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<Round>> {
    use crate::schema::rounds::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    rounds
        .order(created_at)
        .limit(page_size)
        .offset(offset_val)
        .load::<Round>(db)
}

pub fn read_all_rounds_of_division(
    db: &mut database::Connection,
    division_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<Vec<Round>> {
    use crate::schema::rounds::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    rounds
        .filter(did.eq(division_id))
        .order(scheduled_start_time.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<Round>(db)
}

pub fn read_all_rounds_of_tournament(
    db: &mut database::Connection,
    tour_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<Vec<Round>> {
    use crate::schema::divisions::dsl::*;
    use crate::schema::rounds::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let division_ids: Vec<Uuid> = divisions
        .filter(tid.eq(tour_id))
        .order(dname.asc())
        .load::<Division>(db)
        .unwrap()
        .iter()
        .map(|div| div.did)
        .collect();

    rounds
        .filter(crate::schema::rounds::dsl::did.eq_any(division_ids))
        .order(scheduled_start_time.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<Round>(db)
}

/// One fully-formed row of the rounds data table: the round plus its division name, so the
/// whole table is populated from a single API call.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct RoundRow {
    pub roundid: Uuid,
    pub did: Uuid,
    pub division_name: String,
    pub name: String,
    #[schema(value_type = Option<String>, format = DateTime)]
    pub scheduled_start_time: Option<DateTime<Utc>>,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Utc>,
}

/// Returns one page of round-table rows for the tournament (enriched) and the total round count.
/// Rows are ordered by scheduled start time — the same ordering as the plain rounds endpoint.
pub fn read_round_rows_of_tournament(
    db: &mut database::Connection,
    tour_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<(Vec<RoundRow>, i64)> {
    let div_pairs: Vec<(Uuid, String)> = {
        use crate::schema::divisions::dsl::*;
        divisions.filter(tid.eq(tour_id)).select((did, dname)).load::<(Uuid, String)>(db)?
    };
    let div_ids: Vec<Uuid> = div_pairs.iter().map(|(d, _)| *d).collect();
    let div_name_by_id: HashMap<Uuid, String> = div_pairs.into_iter().collect();

    if div_ids.is_empty() {
        return Ok((Vec::new(), 0));
    }

    let total: i64 = {
        use crate::schema::rounds::dsl::*;
        rounds.filter(did.eq_any(&div_ids)).count().get_result(db)?
    };

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;
    let round_list: Vec<Round> = {
        use crate::schema::rounds::dsl::*;
        rounds
            .filter(did.eq_any(&div_ids))
            .order(scheduled_start_time.asc())
            .limit(page_size)
            .offset(offset_val)
            .load::<Round>(db)?
    };

    Ok((build_round_rows(round_list, &div_name_by_id), total))
}

/// Returns one page of round-table rows for the division (enriched) and the total round count.
pub fn read_round_rows_of_division(
    db: &mut database::Connection,
    division_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<(Vec<RoundRow>, i64)> {
    let dname_val: String = {
        use crate::schema::divisions::dsl::*;
        divisions.filter(did.eq(division_id)).select(dname).first::<String>(db)?
    };
    let mut div_name_by_id: HashMap<Uuid, String> = HashMap::new();
    div_name_by_id.insert(division_id, dname_val);

    let total: i64 = {
        use crate::schema::rounds::dsl::*;
        rounds.filter(did.eq(division_id)).count().get_result(db)?
    };

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;
    let round_list: Vec<Round> = {
        use crate::schema::rounds::dsl::*;
        rounds
            .filter(did.eq(division_id))
            .order(scheduled_start_time.asc())
            .limit(page_size)
            .offset(offset_val)
            .load::<Round>(db)?
    };

    Ok((build_round_rows(round_list, &div_name_by_id), total))
}

/// Attaches each round's division name from the given lookup.
fn build_round_rows(round_list: Vec<Round>, div_name_by_id: &HashMap<Uuid, String>) -> Vec<RoundRow> {
    round_list
        .into_iter()
        .map(|r| RoundRow {
            roundid: r.roundid,
            division_name: div_name_by_id.get(&r.did).cloned().unwrap_or_default(),
            did: r.did,
            name: r.name,
            scheduled_start_time: r.scheduled_start_time,
            created_at: r.created_at,
            updated_at: r.updated_at,
        })
        .collect()
}

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &RoundChangeset) -> QueryResult<Round> {
    use crate::schema::rounds::dsl::*;
    diesel::update(rounds.filter(roundid.eq(item_id)))
        .set((
            item,
            updated_at.eq(diesel::dsl::now),
        ))
        .get_result(db)
}

pub fn count(db: &mut database::Connection) -> QueryResult<i64> {
    use crate::schema::rounds::dsl::*;
    rounds.count().get_result(db)
}

pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::rounds::dsl::*;
    diesel::delete(rounds.filter(roundid.eq(item_id))).execute(db)
}
