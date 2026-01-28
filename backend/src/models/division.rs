
use crate::database;
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable,Identifiable};
use serde::{Deserialize, Serialize};
use crate::models::common::*;
use utoipa::ToSchema;
use chrono::{DateTime,Utc};
use uuid::Uuid;

pub struct DivisionBuilder {
    tid: Uuid,
    dname: Option<String>,
    breadcrumb: Option<String>,
    is_public: Option<bool>,
    shortinfo: Option<String>
}

impl DivisionBuilder {
    pub fn new(tid: Uuid) -> Self {
        DivisionBuilder {
            tid: tid,
            dname: None,
            breadcrumb: None,
            is_public: None,
            shortinfo: None
        }
    }

    pub fn new_default(dname: &str, tid: Uuid) -> Self {
        // this mostly intended to be used by tests, not production
        DivisionBuilder {
            tid: tid,
            dname: Some(dname.to_string()),
            breadcrumb: Some("/test/post/for/division/1".to_string()),
            is_public: Some(false),
            shortinfo: Some("Experienced (but still young).".to_string())
        }
    }

    pub fn set_tid(mut self, tid: Uuid) -> Self {
        self.tid = tid;
        self
    }

    pub fn set_dname(mut self, dname_val: &str) -> Self {
        self.dname = Some(dname_val.to_string());
        self
    }

    pub fn set_breadcrumb(mut self, val: &str) -> Self {
        self.breadcrumb = Some(val.to_string());
        self
    }

    pub fn set_is_public(mut self, val: bool) -> Self {
        self.is_public = Some(val);
        self
    }

    pub fn set_shortinfo(mut self, val: String) -> Self {
        self.shortinfo = Some(val);
        self
    }

    fn validate_all_are_some(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.dname.is_none() {
            errors.push("dname is required".to_string());
        }
        if self.breadcrumb.is_none() {
            errors.push("breadcrumb is required".to_string());
        }
        if self.is_public.is_none() {
            errors.push("is_public is required".to_string());
        }
        if self.shortinfo.is_none() {
            errors.push("shortinfo is required".to_string());
        }

        if !errors.is_empty() {
            return Err(errors);
        }
            
        Ok(())
    }

    pub fn build(self) -> Result<NewDivision, Vec<String>> {
        match self.validate_all_are_some() {
            Err(e) => {
                return Err(e);
            },
            Ok(_) => {
                return Ok(
                    NewDivision {
                        tid: self.tid,
                        dname: self.dname.unwrap(),
                        breadcrumb: self.breadcrumb.unwrap(),
                        is_public: self.is_public.unwrap(),
                        shortinfo: self.shortinfo.unwrap()
                    }
                )
            }
        }
    }

    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<Division> {
        let new_division = self.build();
        create(db, &new_division.unwrap())
    }
}

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

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug
)]
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
    pub dname: Option<String>,
    pub breadcrumb: Option<String>,
    pub is_public: Option<bool>,
    pub shortinfo: Option<String>
}

pub fn create(db: &mut database::Connection, item: &NewDivision) -> QueryResult<Division> {
    use crate::schema::divisions::dsl::*;
    insert_into(divisions).values(item).get_result::<Division>(db)
}

pub fn exists(db: &mut database::Connection, did: Uuid) -> bool {
    use crate::schema::divisions::dsl::divisions;
    divisions
        .find(did)
        .get_result::<Division>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<Division> {
    use crate::schema::divisions::dsl::*;
    divisions.filter(did.eq(item_id)).first::<Division>(db)
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

pub fn read_all_divisions_of_tournament(
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

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &DivisionChangeset) -> QueryResult<Division> {
    use crate::schema::divisions::dsl::*;
    diesel::update(divisions.filter(did.eq(item_id)))
        .set((
            item,
            updated_at.eq(diesel::dsl::now),
        ))
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::divisions::dsl::*;
    diesel::delete(divisions.filter(did.eq(item_id))).execute(db)
}
