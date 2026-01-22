
use crate::database;
use crate::models::common::PaginationParams;
use crate::models::division::Division;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{Utc,DateTime};

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
    pub updated_at: DateTime<Utc>
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
    pub scheduled_start_time: DateTime<Utc>
}

// #[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::rounds)]
#[diesel(primary_key(roundid))]
pub struct RoundChangeset {
    pub scheduled_start_time: Option<DateTime<Utc>>
}

pub fn create(db: &mut database::Connection, item: &NewRound) -> QueryResult<Round> {
    use crate::schema::rounds::dsl::*;
    insert_into(rounds).values(item).get_result::<Round>(db)
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<Round> {
    use crate::schema::rounds::dsl::*;
    rounds.filter(roundid.eq(item_id)).first::<Round>(db)
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<Round>> {
    use crate::schema::rounds::dsl::*;
    rounds
        .order(created_at)
        .limit(pagination.page_size)
        .offset(
            pagination.page
                * std::cmp::max(pagination.page_size, PaginationParams::MAX_PAGE_SIZE as i64),
        )
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

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &RoundChangeset) -> QueryResult<Round> {
    use crate::schema::rounds::dsl::*;
    diesel::update(rounds.filter(roundid.eq(item_id)))
        .set(item)
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::rounds::dsl::*;
    diesel::delete(rounds.filter(roundid.eq(item_id))).execute(db)
}
