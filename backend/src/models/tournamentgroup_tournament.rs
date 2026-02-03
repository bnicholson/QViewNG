
use crate::{database, models};
use diesel::*;
use diesel::{QueryResult,Insertable,Identifiable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct TournamentGroupTournamentBuilder {
    tournamentid: Uuid,
    tournamentgroupid: Uuid,
}

impl TournamentGroupTournamentBuilder {
    pub fn new(tg_id: Uuid, tid: Uuid) -> Self {
        Self {
            tournamentid: tid,
            tournamentgroupid: tg_id,
        }
    }
    pub fn new_default(tg_id: Uuid, tid: Uuid) -> Self {
        Self {
            tournamentid: tid,
            tournamentgroupid: tg_id,
        }
    }
    pub fn set_tournamentid(mut self, tid: Uuid) -> Self {
        self.tournamentid = tid;
        self
    }
    pub fn set_tournamentgroupid(mut self, tournamentgroupid: Uuid) -> Self {
        self.tournamentgroupid = tournamentgroupid;
        self
    }
    pub fn build(self) -> Result<NewTournamentGroupTournament, Vec<String>> {
        Ok(
            NewTournamentGroupTournament {
                tournamentid: self.tournamentid,
                tournamentgroupid: self.tournamentgroupid,
            }
        )
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<TournamentGroupTournament> {
        let new_tournamentgrouptournament = self.build();
        create(db, new_tournamentgrouptournament.unwrap())
    }
}

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
#[diesel(table_name = crate::schema::tournamentgroups_tournaments)]
#[diesel(primary_key(tournamentid,tournamentgroupid))]
pub struct TournamentGroupTournament {
    pub tournamentid: Uuid,
    pub tournamentgroupid: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug
)]
#[diesel(table_name = crate::schema::tournamentgroups_tournaments)]
pub struct NewTournamentGroupTournament {
    pub tournamentid: Uuid,
    pub tournamentgroupid: Uuid,
}

// #[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
// #[diesel(table_name = crate::schema::tournaments_admins)]
// #[diesel(primary_key(tournamentid,adminid))]
// pub struct TournamentGroupTournamentChangeset {
//     pub tournamentid: Uuid,
//     pub tournamentgroupid: Uuid,
//     pub created_at: DateTime<Utc>,
// }

pub fn create(db: &mut database::Connection, item: NewTournamentGroupTournament) -> QueryResult<TournamentGroupTournament> {
    use crate::schema::tournamentgroups_tournaments::dsl::*;

    if !models::tournament::exists(db, item.tournamentid) {
        println!("Could not find Tournament by ID={}", &item.tournamentid);
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: Tournament with ID {} does not exist", item.tournamentid).into()
        ));
    }

    if !models::tournamentgroup::exists(db, item.tournamentgroupid) {
        println!("Could not find TournamentGroup by ID={}", &item.tournamentgroupid);
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: TournamentGroup with ID {} does not exist", item.tournamentgroupid).into()
        ));
    }

    insert_into(tournamentgroups_tournaments).values(item).get_result::<TournamentGroupTournament>(db)
}

pub fn delete(db: &mut database::Connection, tg_id: Uuid, tour_id: Uuid) -> QueryResult<usize> {
    use crate::schema::tournamentgroups_tournaments::dsl::*;
    diesel::delete(tournamentgroups_tournaments
        .filter(tournamentgroupid.eq(tg_id)))
        .filter(tournamentid.eq(tour_id))
        .execute(db)
}
