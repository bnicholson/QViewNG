
use crate::database;
use crate::models::room::Room;
use crate::models::round::Round;
use crate::models::user::User;
use crate::models::tournament_admin::TournamentAdmin;
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable,Identifiable,Queryable};
use serde::{Deserialize, Serialize};
use crate::models::common::*;
use chrono::{Utc,DateTime,TimeZone};
use utoipa::{ToSchema};
use crate::models::division::Division;
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

pub fn read_divisions(
    db: &mut database::Connection,
    item_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<Vec<Division>> {
    use crate::schema::divisions::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    divisions
        .filter(tid.eq(item_id))
        .order(dname.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<Division>(db)
}

pub fn read_rooms(
    db: &mut database::Connection,
    item_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<Vec<Room>> {
    use crate::schema::rooms::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    rooms
        .filter(tid.eq(item_id))
        .order(name.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<Room>(db)
}

pub fn read_rounds(
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

pub fn read_users(
    db: &mut database::Connection,
    tour_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<Vec<User>> {
    use crate::schema::users::dsl::*;
    use crate::schema::tournaments_admins::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let admin_ids: Vec<Uuid> = 
        tournaments_admins
            .filter(tournamentid.eq(tour_id))
            .load::<TournamentAdmin>(db)
            .unwrap()
            .iter()
            .map(|admin| admin.adminid)
            .collect();

    users
        .filter(id.eq_any(admin_ids))
        .order(fname.asc())
        .order(lname.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<User>(db)
}

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &TournamentChangeset) -> QueryResult<Tournament> {
    use crate::schema::tournaments::dsl::*;
    diesel::update(tournaments.filter(tid.eq(item_id)))
        .set(item)
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::tournaments::dsl::*;
    diesel::delete(tournaments.filter(tid.eq(item_id))).execute(db)
}
