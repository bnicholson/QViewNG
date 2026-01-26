
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
#[diesel(table_name = crate::schema::teams)]
#[diesel(primary_key(teamid))]
pub struct Team {
    pub teamid: Uuid,                           // identifies the team uniquely
    pub did: Uuid,                           
    pub coachid: Uuid,
    pub name: String,                           // Name of the team (human readable)
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub quizzer_one_id: Option<Uuid>,
    pub quizzer_two_id: Option<Uuid>,
    pub quizzer_three_id: Option<Uuid>,
    pub quizzer_four_id: Option<Uuid>,
    pub quizzer_five_id: Option<Uuid>,
    pub quizzer_six_id: Option<Uuid>
}

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug
)]
#[diesel(table_name = crate::schema::teams)]
pub struct NewTeam {
    pub did: Uuid,
    pub coachid: Uuid,
    pub name: String,                           // Name of the team (human readable)
    pub quizzer_one_id: Option<Uuid>,
    pub quizzer_two_id: Option<Uuid>,
    pub quizzer_three_id: Option<Uuid>,
    pub quizzer_four_id: Option<Uuid>,
    pub quizzer_five_id: Option<Uuid>,
    pub quizzer_six_id: Option<Uuid>
}

// #[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::teams)]
#[diesel(primary_key(teamid))]
pub struct TeamChangeset {
    pub coachid: Option<Uuid>,
    pub name: Option<String>,           // Name of the team (human readable)
    pub quizzer_one_id: Option<Uuid>,
    pub quizzer_two_id: Option<Uuid>,
    pub quizzer_three_id: Option<Uuid>,
    pub quizzer_four_id: Option<Uuid>,
    pub quizzer_five_id: Option<Uuid>,
    pub quizzer_six_id: Option<Uuid>
}

pub fn create(db: &mut database::Connection, item: &NewTeam) -> QueryResult<Team> {
    use crate::schema::teams::dsl::*;
    insert_into(teams).values(item).get_result::<Team>(db)
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<Team> {
    use crate::schema::teams::dsl::*;
    teams.filter(teamid.eq(item_id)).first::<Team>(db)
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<Team>> {
    use crate::schema::teams::dsl::*;
    teams
        .order(created_at)
        .limit(pagination.page_size)
        .offset(
            pagination.page
                * std::cmp::max(pagination.page_size, PaginationParams::MAX_PAGE_SIZE as i64),
        )
        .load::<Team>(db)
}

pub fn read_all_teams_of_division(
    db: &mut database::Connection,
    item_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<Vec<Team>> {
    use crate::schema::teams::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    teams
        .filter(did.eq(item_id))
        .order(name.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<Team>(db)
}

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &TeamChangeset) -> QueryResult<Team> {
    use crate::schema::teams::dsl::*;
    diesel::update(teams.filter(teamid.eq(item_id)))
        .set((
            item,
            updated_at.eq(diesel::dsl::now),
        ))
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::teams::dsl::*;
    diesel::delete(teams.filter(teamid.eq(item_id))).execute(db)
}
