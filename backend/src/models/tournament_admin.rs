
use crate::database;
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable,Identifiable};
use serde::{Deserialize, Serialize};
use crate::models::common::*;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use uuid::Uuid;

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
#[diesel(table_name = crate::schema::tournaments_admins)]
#[diesel(primary_key(tournamentid,adminid))]
pub struct TournamentAdmin {
    pub tournamentid: Uuid,
    pub adminid: Uuid,     
    pub role_description: Option<String>,            
    pub access_lvl: i32,            
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug
)]
#[diesel(table_name = crate::schema::tournaments_admins)]
pub struct NewTournamentAdmin {
    pub tournamentid: Uuid,
    pub adminid: Uuid,     
    pub role_description: String,            
    pub access_lvl: i32
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::tournaments_admins)]
#[diesel(primary_key(tournamentid,adminid))]
pub struct TournamentAdminChangeSet {
    pub role_description: String,            
    pub access_lvl: i32
}

pub fn create(db: &mut database::Connection, item: &NewTournamentAdmin) -> QueryResult<TournamentAdmin> {
    use crate::schema::tournaments_admins::dsl::*;
    insert_into(tournaments_admins).values(item).get_result::<TournamentAdmin>(db)
}

pub fn read(db: &mut database::Connection, tour_id: Uuid, user_id: Uuid) -> QueryResult<TournamentAdmin> {
    use crate::schema::tournaments_admins::dsl::*;
    tournaments_admins
        .filter(tournamentid.eq(tour_id))
        .filter(adminid.eq(user_id))
        .first::<TournamentAdmin>(db)
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<TournamentAdmin>> {
    use crate::schema::tournaments_admins::dsl::*;
    
    tournaments_admins
        .order(created_at)
        .limit(pagination.page_size)
        .offset(
            pagination.page
                * std::cmp::max(pagination.page_size, PaginationParams::MAX_PAGE_SIZE as i64),
        )
        .load::<TournamentAdmin>(db)
}

pub fn update(db: &mut database::Connection, tour_id: Uuid, user_id: Uuid, item: &TournamentAdminChangeSet) -> QueryResult<TournamentAdmin> {
    use crate::schema::tournaments_admins::dsl::*;
    diesel::update(tournaments_admins
        .filter(tournamentid.eq(tour_id))
        .filter(adminid.eq(user_id)))
        .set(item)
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, tour_id: Uuid, user_id: Uuid) -> QueryResult<usize> {
    use crate::schema::tournaments_admins::dsl::*;
    diesel::delete(tournaments_admins
        .filter(tournamentid.eq(tour_id))
        .filter(adminid.eq(user_id)))
        .execute(db)
}
