
use crate::database;
use crate::models::common::PaginationParams;
use crate::models::roster_coach::RosterCoach;
use crate::models::roster_quizzer::RosterQuizzer;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{Utc,DateTime};

pub struct RosterBuilder {
    name: String,
    description: Option<String>,
    created_by_userid: Uuid,
}

impl RosterBuilder {
    pub fn new(roster_name: &str, created_by_userid: Uuid) -> Self {
        Self {
            name: roster_name.to_string(),
            description: None,
            created_by_userid,
        }
    }
    pub fn new_default(roster_name: &str, created_by_userid: Uuid) -> Self {
        Self {
            name: roster_name.to_string(),
            description: None,
            created_by_userid,
        }
    }
    pub fn set_name(mut self, roster_name: String) -> Self {
        self.name = roster_name;
        self
    }
    pub fn set_description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }
    pub fn set_created_by_userid(mut self, created_by_userid: Uuid) -> Self {
        self.created_by_userid = created_by_userid;
        self
    }
    pub fn build(self) -> Result<NewRoster, Vec<String>> {
        Ok(
            NewRoster {
                name: self.name,
                description: self.description,
                created_by_userid: self.created_by_userid,
            }
        )
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<Roster> {
        let new_roster = self.build();
        create(db, &new_roster.unwrap())
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
#[diesel(table_name = crate::schema::rosters)]
#[diesel(primary_key(rosterid))]
pub struct Roster {
    pub rosterid: Uuid,                            // identifies the roster uniquely
    pub name: String,                              // Name of the roster (human readable)
    pub description: Option<String>,               // Description of the roster
    pub created_by_userid: Uuid,                   // User (Coach) who created the roster
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug
)]
#[diesel(table_name = crate::schema::rosters)]
pub struct NewRoster {
    pub name: String,
    pub description: Option<String>,
    pub created_by_userid: Uuid,
}

// #[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::rosters)]
#[diesel(primary_key(sgid))]
pub struct RosterChangeset {
    pub name: String,
    pub description: Option<String>,
}

pub fn create(db: &mut database::Connection, item: &NewRoster) -> QueryResult<Roster> {
    use crate::schema::rosters::dsl::*;
    insert_into(rosters).values(item).get_result::<Roster>(db)
}

pub fn exists(db: &mut database::Connection, rosterid: Uuid) -> bool {
    use crate::schema::rosters::dsl::rosters;
    rosters
        .find(rosterid)
        .get_result::<Roster>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<Roster> {
    use crate::schema::rosters::dsl::*;
    rosters.filter(rosterid.eq(item_id)).first::<Roster>(db)
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<Roster>> {
    use crate::schema::rosters::dsl::*;
    rosters
        .order(created_at)
        .limit(pagination.page_size)
        .offset(
            pagination.page
                * std::cmp::max(pagination.page_size, PaginationParams::MAX_PAGE_SIZE as i64),
        )
        .load::<Roster>(db)
}

pub fn read_all_rosters_of_coach(db: &mut database::Connection, coach_id: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<Roster>> {
    use crate::schema::rosters_coaches::dsl::*;
    use crate::schema::rosters::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let roster_ids: Vec<Uuid> = 
        rosters_coaches
            .filter(coachid.eq(coach_id))
            .load::<RosterCoach>(db)
            .unwrap()
            .iter()
            .map(|rc| rc.rosterid)
            .collect();

    rosters
        .filter(crate::schema::rosters::dsl::rosterid.eq_any(roster_ids))
        .order(name.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<Roster>(db)
}

pub fn read_all_rosters_containing_quizzer(db: &mut database::Connection, quizzer_id: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<Roster>> {
    use crate::schema::rosters_quizzers::dsl::*;
    use crate::schema::rosters::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let roster_ids: Vec<Uuid> = 
        rosters_quizzers
            .filter(quizzerid.eq(quizzer_id))
            .load::<RosterQuizzer>(db)
            .unwrap()
            .iter()
            .map(|rq| rq.rosterid)
            .collect();

    rosters
        .filter(crate::schema::rosters::dsl::rosterid.eq_any(roster_ids))
        .order(name.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<Roster>(db)
}

pub fn update(db: &mut database::Connection, sg_id: Uuid, item: &RosterChangeset) -> QueryResult<Roster> {
    use crate::schema::rosters::dsl::*;
    diesel::update(rosters.filter(rosterid.eq(sg_id)))
        .set((
            item,
            updated_at.eq(diesel::dsl::now),
        ))
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, sg_id: Uuid) -> QueryResult<usize> {
    use crate::schema::rosters::dsl::*;
    diesel::delete(rosters.filter(rosterid.eq(sg_id))).execute(db)
}
