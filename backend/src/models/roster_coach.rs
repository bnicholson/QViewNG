
use crate::{database, models};
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult,Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{Utc,DateTime};

pub struct RosterCoachBuilder {
    coachid: Uuid,
    rosterid: Uuid,
}

impl RosterCoachBuilder {
    pub fn new(coachid: Uuid, rosterid: Uuid) -> Self {
        Self {
            coachid,
            rosterid,
        }
    }
    pub fn new_default(coachid: Uuid, rosterid: Uuid) -> Self {
        Self {
            coachid,
            rosterid,
        }
    }
    pub fn build(self) -> Result<NewRosterCoach, Vec<String>> {
        Ok(
            NewRosterCoach {
                coachid: self.coachid,
                rosterid: self.rosterid,
            }
        )
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<RosterCoach> {
        let new_rostercoach = self.build();
        create(db, new_rostercoach.unwrap())
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
#[diesel(table_name = crate::schema::rosters_coaches)]
#[diesel(primary_key(coachid, rosterid))]
pub struct RosterCoach {
    pub coachid: Uuid,
    pub rosterid: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug
)]
#[diesel(table_name = crate::schema::rosters_coaches)]
pub struct NewRosterCoach {
    pub coachid: Uuid,
    pub rosterid: Uuid,
}

pub fn create(db: &mut database::Connection, item: NewRosterCoach) -> QueryResult<RosterCoach> {
    use crate::schema::rosters_coaches::dsl::*;

    if !models::roster::exists(db, item.rosterid) {
        println!("Could not find Roster by ID={}", &item.rosterid);
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: Roster with ID {} does not exist", item.rosterid).into()
        ));
    }

    if !models::user::exists(db, item.coachid) {
        println!("Could not find Quizzer by ID={}", &item.coachid);
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: User (Quizzer) with ID {} does not exist", item.coachid).into()
        ));
    }

    insert_into(rosters_coaches)
        .values(item)
        .get_result::<RosterCoach>(db)
}

pub fn delete(db: &mut database::Connection, statsgroup_id: Uuid, game_id: Uuid) -> QueryResult<usize> {
    use crate::schema::rosters_coaches::dsl::*;
    diesel::delete(
        rosters_coaches
            .filter(coachid.eq(statsgroup_id))
            .filter(rosterid.eq(game_id))
    ).execute(db)
}
