
use crate::database;
use crate::models::common::PaginationParams;
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
