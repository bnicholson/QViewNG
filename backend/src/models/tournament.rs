
use crate::database;
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable,Identifiable,Queryable};
use serde::{Deserialize, Serialize};
use crate::models::common::*;
use chrono::{Utc,DateTime,TimeZone};
use utoipa::{ToSchema};
use uuid::Uuid;

// #[tsync::tsync]
#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Identifiable,
    Selectable,
    ToSchema
)]
#[diesel(table_name = crate::schema::tournaments)]
#[diesel(primary_key(tid))]
pub struct Tournament {
    pub tid: Uuid, 
    pub organization: String,
    pub tname: String,             // name of this tournament (humans)
    pub breadcrumb: String,
    #[schema(value_type = String, format = DateTime)]
    pub fromdate: chrono::naive::NaiveDate,
    #[schema(value_type = String, format = DateTime)]
    pub todate: chrono::naive::NaiveDate,
    pub venue: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub contact: String,
    pub contactemail: String,
    pub is_public: bool,
    pub shortinfo : String,
    pub info: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Insertable,
    ToSchema
)]
#[diesel(table_name = crate::schema::tournaments)]
pub struct NewTournament {
    pub organization: String,
    pub tname: String,             // name of this tournament (humans)
    pub breadcrumb: String,
    pub fromdate: chrono::naive::NaiveDate,
    pub todate: chrono::naive::NaiveDate,
    pub venue: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub contact: String,
    pub contactemail: String,
    pub shortinfo : String,
    pub info: String
}

// #[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::tournaments)]
#[diesel(primary_key(tid))]
pub struct TournamentChangeset {   
    pub organization: Option<String>,
    pub tname: Option<String>,
    pub breadcrumb: Option<String>,
    pub fromdate: Option<chrono::naive::NaiveDate>,
    pub todate: Option<chrono::naive::NaiveDate>,
    pub venue: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub country: Option<String>,
    pub contact: Option<String>,
    pub contactemail: Option<String>,
    pub is_public: Option<bool>,
    pub shortinfo: Option<String>,
    pub info: Option<String>
}

pub fn create(db: &mut database::Connection, item: &NewTournament) -> QueryResult<Tournament> {
    use crate::schema::tournaments::dsl::*;
    
    diesel::insert_into(tournaments)
        .values(item)
        .get_result::<Tournament>(db)
}

pub fn exists(db: &mut database::Connection, tid: Uuid) -> bool {
    use crate::schema::tournaments::dsl::tournaments;
    tournaments
        .find(tid)
        .get_result::<Tournament>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<Tournament> {
    use crate::schema::tournaments::dsl::*;
    tournaments.filter(tid.eq(item_id)).first::<Tournament>(db)
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<Tournament>> {
    use crate::schema::tournaments::dsl::*;
    let values = tournaments
        .order(todate)
        .limit(pagination.page_size)
        .offset(
            pagination.page
                * std::cmp::max(pagination.page_size, PaginationParams::MAX_PAGE_SIZE as i64),
        )
        .load::<Tournament>(db);
    values
}

pub fn read_between_dates(db: &mut database::Connection, from_dt: i64, to_dt: i64) -> QueryResult<Vec<Tournament>> {
    use crate::schema::tournaments::dsl::*;
    let dt_from = Utc.timestamp_millis_opt(from_dt ).unwrap().naive_utc().date();
    let dt_to = Utc.timestamp_millis_opt(to_dt).unwrap().naive_utc().date();

    let values = tournaments
        .order(todate)
        .filter(todate.ge(dt_from))
        .filter(fromdate.le(dt_to))
        .load::<Tournament>(db);
    values
}

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &TournamentChangeset) -> QueryResult<Tournament> {
    use crate::schema::tournaments::dsl::*;
    diesel::update(tournaments.filter(tid.eq(item_id)))
        .set((
            item,
            updated_at.eq(diesel::dsl::now),
        ))
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::tournaments::dsl::*;
    diesel::delete(tournaments.filter(tid.eq(item_id))).execute(db)
}
