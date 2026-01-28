
use crate::database;
use crate::models::common::PaginationParams;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{Utc,DateTime};

pub struct TeamBuilder {
    did: Uuid,
    coachid: Option<Uuid>,
    name: Option<String>,
    quizzer_one_id: Option<Uuid>,
    quizzer_two_id: Option<Uuid>,
    quizzer_three_id: Option<Uuid>,
    quizzer_four_id: Option<Uuid>,
    quizzer_five_id: Option<Uuid>,
    quizzer_six_id: Option<Uuid>
}

impl TeamBuilder {
    pub fn new(division_id: Uuid) -> Self {
        Self {
            did: division_id,
            coachid: None,
            name: None,
            quizzer_one_id: None,
            quizzer_two_id: None,
            quizzer_three_id: None,
            quizzer_four_id: None,
            quizzer_five_id: None,
            quizzer_six_id: None
        }
    }
    pub fn new_default(division_id: Uuid) -> Self {
        Self {
            did: division_id,
            coachid: None,
            name: None,
            quizzer_one_id: None,
            quizzer_two_id: None,
            quizzer_three_id: None,
            quizzer_four_id: None,
            quizzer_five_id: None,
            quizzer_six_id: None
        }
    }
    pub fn set_coachid(mut self, coachid: Uuid) -> Self {
        self.coachid = Some(coachid);
        self
    }
    pub fn set_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
    pub fn set_quizzer_one_id(mut self, quizzer_one_id: Uuid) -> Self {
        self.quizzer_one_id = Some(quizzer_one_id);
        self
    }
    pub fn set_quizzer_two_id(mut self, quizzer_two_id: Uuid) -> Self {
        self.quizzer_two_id = Some(quizzer_two_id);
        self
    }
    pub fn set_quizzer_three_id(mut self, quizzer_three_id: Uuid) -> Self {
        self.quizzer_three_id = Some(quizzer_three_id);
        self
    }
    pub fn set_quizzer_four_id(mut self, quizzer_four_id: Uuid) -> Self {
        self.quizzer_four_id = Some(quizzer_four_id);
        self
    }
    pub fn set_quizzer_five_id(mut self, quizzer_five_id: Uuid) -> Self {
        self.quizzer_five_id = Some(quizzer_five_id);
        self
    }
    pub fn set_quizzer_six_id(mut self, quizzer_six_id: Uuid) -> Self {
        self.quizzer_six_id = Some(quizzer_six_id);
        self
    }
    fn validate_all_are_some(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.name.is_none() {
            errors.push("name is required".to_string());
        }
        if self.coachid.is_none() {
            errors.push("coachid is required".to_string());
        }
        
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
    pub fn build(self) -> Result<NewTeam, Vec<String>> {
        match self.validate_all_are_some() {
            Err(e) => {
                Err(e)
            },
            Ok(_) => {
                Ok(
                    NewTeam {
                        did: self.did,
                        coachid: self.coachid.unwrap(),
                        name: self.name.unwrap(),
                        quizzer_one_id: self.quizzer_one_id,
                        quizzer_two_id: self.quizzer_two_id,
                        quizzer_three_id: self.quizzer_three_id,
                        quizzer_four_id: self.quizzer_four_id,
                        quizzer_five_id: self.quizzer_five_id,
                        quizzer_six_id: self.quizzer_six_id
                    }
                )
            }
        }
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<Team> {
        let new_team = self.build();
        create(db, &new_team.unwrap())
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

pub fn exists(db: &mut database::Connection, id: Uuid) -> bool {
    use crate::schema::teams::dsl::teams;
    teams
        .find(id)
        .get_result::<Team>(db)
        .is_ok()
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
