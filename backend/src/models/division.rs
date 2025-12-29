
use crate::database;
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable,Identifiable};
use serde::{Deserialize, Serialize};
use crate::models::common::*;
use utoipa::ToSchema;
// this import requires this syntax (to appease rustc):
use crate::schema::divisions::dsl::{did,tid,dname,breadcrumb,hide,shortinfo,created_at,updated_at};

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
ToSchema
)]
#[diesel(table_name = crate::schema::divisions)]
#[diesel(primary_key(did))]
pub struct Division {
//    #[diesel(sql_type = Integer)]
    pub did: BigId,                             // identifies the division
    pub tid: BigId,                             // id of the associated tournament
    pub dname: String,                          // Name of the division (human readable)
    pub breadcrumb: String,                     // used as part of short urls
    pub hide: bool,
    pub shortinfo : String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: UTC,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: UTC
}

#[derive(Insertable,Deserialize)]
#[diesel(table_name = crate::schema::divisions)]
pub struct NewDivision {
    pub tid: BigId,
    pub dname: String,
    pub breadcrumb: String,
    pub hide: bool,
    pub shortinfo: String
}

// #[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::divisions)]
#[diesel(primary_key(did))]
pub struct DivisionChangeset {
    pub dname: String,
    pub breadcrumb: String,
    pub hide: bool,
    pub shortinfo: Option<String>
}

pub fn create(db: &mut database::Connection, item: &NewDivision) -> QueryResult<Division> {
    use crate::schema::divisions::dsl::*;
    insert_into(divisions).values(item).get_result::<Division>(db)
}

pub fn read(db: &mut database::Connection, item_id: BigId) -> QueryResult<Division> {
    use crate::schema::divisions::dsl::*;
    divisions.filter(tid.eq(item_id)).first::<Division>(db)
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<Division>> {
    use crate::schema::divisions::dsl::*;
    let values = divisions
        .order(created_at)
        .filter(tid.eq(pagination.page_size))
        .limit(pagination.page_size)
        .offset(
            pagination.page
                * std::cmp::max(pagination.page_size, PaginationParams::MAX_PAGE_SIZE as i64),
        )
        .load::<Division>(db);
        values

}

pub fn update(db: &mut database::Connection, item_id: BigId, item: &DivisionChangeset) -> QueryResult<Division> {
    use crate::schema::divisions::dsl::*;
    diesel::update(divisions.filter(tid.eq(item_id)))
        .set(item)
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, item_id: BigId) -> QueryResult<usize> {
    use crate::schema::divisions::dsl::*;
    diesel::delete(divisions.filter(tid.eq(item_id))).execute(db)
}
