
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
    
    // fn validate_all_are_some(&self) -> Result<(), Vec<String>> {
    //     let mut errors = Vec::new();
    //     if self.role_description.is_none() {
    //         errors.push("role_description is required".to_string());
    //     }
    //     if self.access_lvl.is_none() {
    //         errors.push("access_lvl is required".to_string());
    //     }
        
    //     Ok(())
    // }
    pub fn build(self) -> Result<NewTournamentGroupTournament, Vec<String>> {
        Ok(
            NewTournamentGroupTournament {
                tournamentid: self.tournamentid,
                tournamentgroupid: self.tournamentgroupid,
            }
        )
        // match self.validate_all_are_some() {
        //     Err(e) => {
        //         Err(e)
        //     },
        //     Ok(_) => {
        //         Ok(
        //             NewTournamentGroupTournament {
        //                 tournamentid: self.tournamentid,
        //                 tournamentgroupid: self.tournamentgroupid,
        //             }
        //         )
        //     }
        // }
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

// pub fn update(db: &mut database::Connection, tour_id: Uuid, user_id: Uuid, item: &TournamentGroupTournamentChangeset) -> QueryResult<TournamentGroupTournament> {
//     use crate::schema::tournaments_admins::dsl::*;
//     diesel::update(tournaments_admins
//         .filter(tournamentid.eq(tour_id))
//         .filter(adminid.eq(user_id)))
//         .set((
//             item,
//             updated_at.eq(diesel::dsl::now),
//         ))
//         .get_result(db)
// }

pub fn delete(db: &mut database::Connection, tour_id: Uuid, user_id: Uuid) -> QueryResult<usize> {
    use crate::schema::tournaments_admins::dsl::*;
    diesel::delete(tournaments_admins
        .filter(tournamentid.eq(tour_id))
        .filter(adminid.eq(user_id)))
        .execute(db)
}
