
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
    last_modified_user: Option<Uuid>,
}

impl RosterBuilder {
    pub fn new(roster_name: &str, created_by_userid: Uuid) -> Self {
        Self {
            name: roster_name.to_string(),
            description: None,
            created_by_userid,
            last_modified_user: None,
        }
    }
    pub fn new_default(roster_name: &str, created_by_userid: Uuid) -> Self {
        Self {
            name: roster_name.to_string(),
            description: None,
            created_by_userid,
            last_modified_user: None,
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
    pub fn set_last_modified_user(mut self, user_id: Uuid) -> Self {
        self.last_modified_user = Some(user_id);
        self
    }
    pub fn build(self) -> Result<NewRoster, Vec<String>> {
        Ok(
            NewRoster {
                name: self.name,
                description: self.description,
                created_by_userid: self.created_by_userid,
                last_modified_user: self.last_modified_user.unwrap_or(self.created_by_userid),
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
    pub last_modified_user: Uuid,
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
    #[serde(default)]
    pub last_modified_user: Uuid,
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

pub fn count(db: &mut database::Connection) -> QueryResult<i64> {
    use crate::schema::rosters::dsl::*;
    rosters.count().get_result(db)
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

/// One row of the user's "My Rosters" quizzers table: a quizzer that appears on any roster the
/// user (coach) created. No audit columns — a quizzer isn't an audited domain entity.
#[derive(Debug, Serialize, Deserialize, Clone, utoipa::ToSchema)]
pub struct UserRosterQuizzerRow {
    pub quizzer_id: Uuid,
    pub fname: String,
    pub mname: String,
    pub lname: String,
    pub email: String,
}

/// Returns one page of the distinct quizzers across all rosters created by `coach_id`, plus the
/// total distinct count — a single scoped, paginated call replacing the per-roster fan-out.
pub fn read_roster_quizzer_rows_of_coach(
    db: &mut database::Connection,
    coach_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<(Vec<UserRosterQuizzerRow>, i64)> {
    let roster_ids: Vec<Uuid> = {
        use crate::schema::rosters::dsl::*;
        rosters.filter(created_by_userid.eq(coach_id)).select(rosterid).load::<Uuid>(db)?
    };
    let mut quizzer_ids: Vec<Uuid> = {
        use crate::schema::rosters_quizzers::dsl::*;
        rosters_quizzers
            .filter(rosterid.eq_any(&roster_ids))
            .select(quizzerid)
            .distinct()
            .load::<Uuid>(db)?
    };
    quizzer_ids.sort();
    quizzer_ids.dedup();
    let total = quizzer_ids.len() as i64;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;
    let rows: Vec<UserRosterQuizzerRow> = {
        use crate::schema::users::dsl::*;
        users
            .filter(id.eq_any(&quizzer_ids))
            .order((lname.asc(), fname.asc()))
            .limit(page_size)
            .offset(offset_val)
            .load::<crate::models::user::User>(db)?
            .into_iter()
            .map(|u| UserRosterQuizzerRow {
                quizzer_id: u.id,
                fname: u.fname,
                mname: u.mname,
                lname: u.lname,
                email: u.email,
            })
            .collect()
    };
    Ok((rows, total))
}

pub fn update(db: &mut database::Connection, sg_id: Uuid, item: &RosterChangeset, modified_by: Uuid) -> QueryResult<Roster> {
    use crate::schema::rosters::dsl::*;
    diesel::update(rosters.filter(rosterid.eq(sg_id)))
        .set((
            item,
            updated_at.eq(diesel::dsl::now),
            last_modified_user.eq(modified_by),
        ))
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, sg_id: Uuid) -> QueryResult<usize> {
    use crate::schema::rosters::dsl::*;
    diesel::delete(rosters.filter(rosterid.eq(sg_id))).execute(db)
}
