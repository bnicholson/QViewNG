
use crate::{database, models};
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult,Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{Utc,DateTime};

pub struct RosterQuizzerBuilder {
    quizzerid: Uuid,
    rosterid: Uuid,
}

impl RosterQuizzerBuilder {
    pub fn new(quizzerid: Uuid, rosterid: Uuid) -> Self {
        Self {
            quizzerid,
            rosterid,
        }
    }
    pub fn new_default(quizzerid: Uuid, rosterid: Uuid) -> Self {
        Self {
            quizzerid,
            rosterid,
        }
    }
    pub fn build(self) -> Result<NewRosterQuizzer, Vec<String>> {
        Ok(
            NewRosterQuizzer {
                quizzerid: self.quizzerid,
                rosterid: self.rosterid,
            }
        )
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<RosterQuizzer> {
        let new_rostercoach = self.build();
        create(db, &new_rostercoach.unwrap())
    }
}

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
#[diesel(table_name = crate::schema::rosters_quizzers)]
#[diesel(primary_key(quizzerid, rosterid))]
pub struct RosterQuizzer {
    pub quizzerid: Uuid,
    pub rosterid: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug
)]
#[diesel(table_name = crate::schema::rosters_quizzers)]
pub struct NewRosterQuizzer {
    pub quizzerid: Uuid,
    pub rosterid: Uuid,
}

// #[tsync::tsync]
// #[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
// #[diesel(table_name = crate::schema::rosters_quizzers)]
// #[diesel(primary_key(gameid, statsgroupid))]
// pub struct RosterQuizzerChangeset {
//     pub name: String,                              // Name of the rostercoach (human readable)
//     pub description: Option<String>,               // Description of the rostercoach
// }

pub fn create(db: &mut database::Connection, item: &NewRosterQuizzer) -> QueryResult<RosterQuizzer> {
    use crate::schema::rosters_quizzers::dsl::*;

    if !models::roster::exists(db, item.rosterid) {
        println!("Could not find Roster by ID={}", &item.rosterid);
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: Roster with ID {} does not exist", item.rosterid).into()
        ));
    }

    if !models::user::exists(db, item.quizzerid) {
        println!("Could not find Quizzer by ID={}", &item.quizzerid);
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: User (Quizzer) with ID {} does not exist", item.quizzerid).into()
        ));
    }

    insert_into(rosters_quizzers)
        .values(item)
        .get_result::<RosterQuizzer>(db)
}

pub fn delete(db: &mut database::Connection, roster_id: Uuid, quizzer_id: Uuid) -> QueryResult<usize> {
    use crate::schema::rosters_quizzers::dsl::*;
    diesel::delete(
        rosters_quizzers
            .filter(quizzerid.eq(quizzer_id))
            .filter(rosterid.eq(roster_id))
    ).execute(db)
}
