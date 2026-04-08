use crate::database;
use crate::models::common::PaginationParams;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult, AsChangeset, Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

pub struct CreateTournamentApplicantBuilder {
    user_id: Uuid,
    request_context: Option<String>,
    status: String,
    last_modified_user_id: Uuid,
}

impl CreateTournamentApplicantBuilder {
    pub fn new(user_id: Uuid, status: &str, last_modified_user_id: Uuid) -> Self {
        Self {
            user_id,
            request_context: None,
            status: status.to_string(),
            last_modified_user_id,
        }
    }
    pub fn new_default(user_id: Uuid, last_modified_user_id: Uuid) -> Self {
        Self {
            user_id,
            request_context: None,
            status: "pending".to_string(),
            last_modified_user_id,
        }
    }
    pub fn set_request_context(mut self, request_context: Option<String>) -> Self {
        self.request_context = request_context;
        self
    }
    pub fn set_status(mut self, status: &str) -> Self {
        self.status = status.to_string();
        self
    }
    pub fn build(self) -> NewCreateTournamentApplicant {
        NewCreateTournamentApplicant {
            user_id: self.user_id,
            request_context: self.request_context,
            status: self.status,
            last_modified_user_id: self.last_modified_user_id,
        }
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<CreateTournamentApplicant> {
        let new_item = self.build();
        create(db, &new_item)
    }
}

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
#[diesel(table_name = crate::schema::create_tournament_applicants)]
#[diesel(primary_key(id))]
pub struct CreateTournamentApplicant {
    pub id: Uuid,
    pub user_id: Uuid,
    pub request_context: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub last_modified_user_id: Uuid,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::create_tournament_applicants)]
pub struct NewCreateTournamentApplicant {
    pub user_id: Uuid,
    pub request_context: Option<String>,
    pub status: String,
    pub last_modified_user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::create_tournament_applicants)]
pub struct CreateTournamentApplicantChangeset {
    pub request_context: Option<String>,
    pub status: Option<String>,
    pub last_modified_user_id: Option<Uuid>,
}

pub fn create(db: &mut database::Connection, item: &NewCreateTournamentApplicant) -> QueryResult<CreateTournamentApplicant> {
    use crate::schema::create_tournament_applicants::dsl::*;
    insert_into(create_tournament_applicants).values(item).get_result::<CreateTournamentApplicant>(db)
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<CreateTournamentApplicant> {
    use crate::schema::create_tournament_applicants::dsl::*;
    create_tournament_applicants.filter(id.eq(item_id)).first::<CreateTournamentApplicant>(db)
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<CreateTournamentApplicant>> {
    use crate::schema::create_tournament_applicants::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    create_tournament_applicants
        .order(created_at.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<CreateTournamentApplicant>(db)
}

pub fn read_by_user(db: &mut database::Connection, uid: Uuid) -> QueryResult<Vec<CreateTournamentApplicant>> {
    use crate::schema::create_tournament_applicants::dsl::*;
    create_tournament_applicants
        .filter(user_id.eq(uid))
        .order(created_at.asc())
        .load::<CreateTournamentApplicant>(db)
}

pub fn count(db: &mut database::Connection) -> QueryResult<i64> {
    use crate::schema::create_tournament_applicants::dsl::*;
    create_tournament_applicants.count().get_result(db)
}

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &CreateTournamentApplicantChangeset) -> QueryResult<CreateTournamentApplicant> {
    use crate::schema::create_tournament_applicants::dsl::*;
    diesel::update(create_tournament_applicants.filter(id.eq(item_id)))
        .set((
            item,
            modified_at.eq(diesel::dsl::now),
        ))
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::create_tournament_applicants::dsl::*;
    diesel::delete(create_tournament_applicants.filter(id.eq(item_id))).execute(db)
}
