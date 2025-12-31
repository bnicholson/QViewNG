
use crate::database;
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable,Identifiable};
use serde::{Deserialize, Serialize};
use crate::models::common::*;
use utoipa::ToSchema;
use chrono::{DateTime,Utc};
// this import requires this syntax (to appease rustc):
use crate::schema::divisions::dsl::{did,tid,dname,breadcrumb,is_public,shortinfo,created_at,updated_at};
use uuid::Uuid;

// #[tsync::tsync]
#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Insertable,
    Identifiable,
    AsChangeset,
    Selectable,
    ToSchema
)]
#[diesel(table_name = crate::schema::divisions)]
#[diesel(primary_key(did))]
pub struct Division {
    pub did: Uuid,                              // identifies the division
    pub tid: Uuid,                              // id of the associated tournament
    pub dname: String,                          // Name of the division (human readable)
    pub breadcrumb: String,                     // used as part of short urls
    pub is_public: bool,
    pub shortinfo : String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(Insertable,Deserialize)]
#[diesel(table_name = crate::schema::divisions)]
pub struct NewDivision {
    pub tid: Uuid,
    pub dname: String,
    pub breadcrumb: String,
    pub is_public: bool,
    pub shortinfo: String
}

// #[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::divisions)]
#[diesel(primary_key(did))]
pub struct DivisionChangeset {
    pub dname: String,
    pub breadcrumb: String,
    pub is_public: bool,
    pub shortinfo: Option<String>
}

pub fn create(db: &mut database::Connection, item: &NewDivision) -> QueryResult<Division> {
    use crate::schema::divisions::dsl::*;
    insert_into(divisions).values(item).get_result::<Division>(db)
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<Division> {
    use crate::schema::divisions::dsl::*;
    divisions.filter(tid.eq(item_id)).first::<Division>(db)
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<Division>> {
    use crate::schema::divisions::dsl::*;
    
    divisions
        .order(created_at)
        .limit(pagination.page_size)
        .offset(
            pagination.page
                * std::cmp::max(pagination.page_size, PaginationParams::MAX_PAGE_SIZE as i64),
        )
        .load::<Division>(db)
}

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &DivisionChangeset) -> QueryResult<Division> {
    use crate::schema::divisions::dsl::*;
    diesel::update(divisions.filter(tid.eq(item_id)))
        .set(item)
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::divisions::dsl::*;
    diesel::delete(divisions.filter(tid.eq(item_id))).execute(db)
}
