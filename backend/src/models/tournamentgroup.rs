
use crate::database;
use crate::models::common::PaginationParams;
use crate::models::tournamentgroup_tournament::TournamentGroupTournament;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{Utc,DateTime};

pub struct TournamentGroupBuilder {
    name: String,
    description: Option<String>,
    creator_id: Option<Uuid>,
    owner_id: Option<Uuid>,
}

impl TournamentGroupBuilder {
    pub fn new(tournamentgroup_name: &str) -> Self {
        Self {
            name: tournamentgroup_name.to_string(),
            description: None,
            creator_id: None,
            owner_id: None,
        }
    }
    pub fn new_default(tournamentgroup_name: &str) -> Self {
        Self {
            name: tournamentgroup_name.to_string(),
            description: Some("".to_string()),
            creator_id: None,
            owner_id: None,
        }
    }
    pub fn set_creator_id(mut self, id: Uuid) -> Self {
        self.creator_id = Some(id);
        self
    }
    pub fn set_owner_id(mut self, id: Uuid) -> Self {
        self.owner_id = Some(id);
        self
    }
    pub fn set_name(mut self, tournamentgroup_name: String) -> Self {
        self.name = tournamentgroup_name;
        self
    }
    pub fn set_description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }
    pub fn build(self) -> Result<NewTournamentGroup, Vec<String>> {
        let mut errors = Vec::new();
        if self.creator_id.is_none() { errors.push("creator_id is required".to_string()); }
        if self.owner_id.is_none()   { errors.push("owner_id is required".to_string()); }
        if !errors.is_empty() { return Err(errors); }
        Ok(NewTournamentGroup {
            name: self.name,
            description: self.description,
            creator_id: self.creator_id.unwrap(),
            owner_id: self.owner_id.unwrap(),
        })
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<TournamentGroup> {
        let new_tournamentgroup = self.build();
        create(db, &new_tournamentgroup.unwrap())
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
#[diesel(table_name = crate::schema::tournamentgroups)]
#[diesel(primary_key(tgid))]
pub struct TournamentGroup {
    pub tgid: Uuid,
    pub name: String,                           // Name of the tournamentgroup (human readable)
    pub description: Option<String>,            // Description of the tournamentgroup
    pub created_at: DateTime<Utc>,              // When was this tournamentgroup created
    pub updated_at: DateTime<Utc>,              // When was this tournamentgroup last updated
    pub creator_id: Uuid,                      // User who created this group
    pub owner_id: Uuid,                        // User who owns this group
}

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug
)]
#[diesel(table_name = crate::schema::tournamentgroups)]
pub struct NewTournamentGroup {
    pub name: String,                           // Name of the tournamentgroup (human readable)
    pub description: Option<String>,            // Description of the tournamentgroup
    pub creator_id: Uuid,
    pub owner_id: Uuid,
}

/// Payload accepted from the frontend (no creator_id/owner_id — injected server-side).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewTournamentGroupPayload {
    pub name: String,
    pub description: Option<String>,
}

// #[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::tournamentgroups)]
#[diesel(primary_key(tournamentgroupid))]
pub struct TournamentGroupChangeset {
    pub name: String,                           // Name of the tournamentgroup (human readable)
    pub description: Option<String>,            // Description of the tournamentgroup
}

pub fn create(db: &mut database::Connection, item: &NewTournamentGroup) -> QueryResult<TournamentGroup> {
    use crate::schema::tournamentgroups::dsl::*;
    insert_into(tournamentgroups).values(item).get_result::<TournamentGroup>(db)
}

pub fn exists(db: &mut database::Connection, tournamentgroupid: Uuid) -> bool {
    use crate::schema::tournamentgroups::dsl::tournamentgroups;
    tournamentgroups
        .find(tournamentgroupid)
        .get_result::<TournamentGroup>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<TournamentGroup> {
    use crate::schema::tournamentgroups::dsl::*;
    tournamentgroups.filter(tgid.eq(item_id)).first::<TournamentGroup>(db)
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<TournamentGroup>> {
    use crate::schema::tournamentgroups::dsl::*;
    tournamentgroups
        .order(created_at)
        .limit(pagination.page_size)
        .offset(
            pagination.page
                * std::cmp::max(pagination.page_size, PaginationParams::MAX_PAGE_SIZE as i64),
        )
        .load::<TournamentGroup>(db)
}

pub fn count(db: &mut database::Connection) -> QueryResult<i64> {
    use crate::schema::tournamentgroups::dsl::*;
    tournamentgroups.count().get_result(db)
}

pub fn read_all_tournamentgroups_of_tournament(db: &mut database::Connection, tour_id: Uuid, pagination: &PaginationParams) -> QueryResult<Vec<TournamentGroup>> {
    use crate::schema::tournamentgroups_tournaments::dsl::*;
    use crate::schema::tournamentgroups::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let tg_ids: Vec<Uuid> = 
        tournamentgroups_tournaments
            .filter(tournamentid.eq(tour_id))
            .load::<TournamentGroupTournament>(db)
            .unwrap()
            .iter()
            .map(|tg_tour| tg_tour.tournamentgroupid)
            .collect();

    tournamentgroups
        .filter(tgid.eq_any(tg_ids))
        .order(name.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<TournamentGroup>(db)
}

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &TournamentGroupChangeset) -> QueryResult<TournamentGroup> {
    use crate::schema::tournamentgroups::dsl::*;
    diesel::update(tournamentgroups.filter(tgid.eq(item_id)))
        .set((
            item,
            updated_at.eq(diesel::dsl::now),
        ))
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::tournamentgroups::dsl::*;
    diesel::delete(tournamentgroups.filter(tgid.eq(item_id))).execute(db)
}
