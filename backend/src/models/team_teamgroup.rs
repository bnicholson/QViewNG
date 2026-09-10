use crate::database;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult, Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

pub struct TeamTeamgroupBuilder {
    teamid: Uuid,
    team_group_id: Uuid,
    creator_userid: Option<Uuid>,
    last_modified_userid: Option<Uuid>,
}

impl TeamTeamgroupBuilder {
    pub fn new(teamid: Uuid, team_group_id: Uuid) -> Self {
        Self { teamid, team_group_id, creator_userid: None, last_modified_userid: None }
    }
    pub fn new_default(teamid: Uuid, team_group_id: Uuid) -> Self {
        Self::new(teamid, team_group_id)
    }
    pub fn set_creator_userid(mut self, user_id: Uuid) -> Self {
        self.creator_userid = Some(user_id);
        self
    }
    pub fn set_last_modified_userid(mut self, user_id: Uuid) -> Self {
        self.last_modified_userid = Some(user_id);
        self
    }
    pub fn build(self) -> Result<NewTeamTeamgroup, Vec<String>> {
        let creator = match self.creator_userid {
            Some(c) => c,
            None => return Err(vec!["creator_userid is required".to_string()]),
        };
        Ok(NewTeamTeamgroup {
            teamid: self.teamid,
            team_group_id: self.team_group_id,
            creator_userid: creator,
            last_modified_userid: self.last_modified_userid.unwrap_or(creator),
        })
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<TeamTeamgroup> {
        create(db, &self.build().unwrap())
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
#[diesel(table_name = crate::schema::team_teamgroups)]
#[diesel(primary_key(teamid, team_group_id))]
pub struct TeamTeamgroup {
    pub teamid: Uuid,
    pub team_group_id: Uuid,
    pub created_date: DateTime<Utc>,
    pub creator_userid: Uuid,
    pub last_modified_date: DateTime<Utc>,
    pub last_modified_userid: Uuid,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::team_teamgroups)]
pub struct NewTeamTeamgroup {
    pub teamid: Uuid,
    pub team_group_id: Uuid,
    pub creator_userid: Uuid,
    pub last_modified_userid: Uuid,
}

pub fn create(db: &mut database::Connection, item: &NewTeamTeamgroup) -> QueryResult<TeamTeamgroup> {
    use crate::schema::team_teamgroups::dsl::*;
    insert_into(team_teamgroups).values(item).get_result::<TeamTeamgroup>(db)
}

pub fn exists(db: &mut database::Connection, team_id: Uuid, group_id: Uuid) -> bool {
    use crate::schema::team_teamgroups::dsl::*;
    team_teamgroups
        .filter(teamid.eq(team_id))
        .filter(team_group_id.eq(group_id))
        .get_result::<TeamTeamgroup>(db)
        .is_ok()
}

pub fn read_all(db: &mut database::Connection) -> QueryResult<Vec<TeamTeamgroup>> {
    use crate::schema::team_teamgroups::dsl::*;
    team_teamgroups.order(created_date).load::<TeamTeamgroup>(db)
}

/// All bridge rows for the given team group (i.e. its team memberships).
pub fn read_all_of_team_group(db: &mut database::Connection, group_id: Uuid) -> QueryResult<Vec<TeamTeamgroup>> {
    use crate::schema::team_teamgroups::dsl::*;
    team_teamgroups.filter(team_group_id.eq(group_id)).order(created_date).load::<TeamTeamgroup>(db)
}

/// All bridge rows for the given team (i.e. the groups a team belongs to).
pub fn read_all_of_team(db: &mut database::Connection, team_id: Uuid) -> QueryResult<Vec<TeamTeamgroup>> {
    use crate::schema::team_teamgroups::dsl::*;
    team_teamgroups.filter(teamid.eq(team_id)).order(created_date).load::<TeamTeamgroup>(db)
}

pub fn delete(db: &mut database::Connection, team_id: Uuid, group_id: Uuid) -> QueryResult<usize> {
    use crate::schema::team_teamgroups::dsl::*;
    diesel::delete(
        team_teamgroups
            .filter(teamid.eq(team_id))
            .filter(team_group_id.eq(group_id)),
    ).execute(db)
}
