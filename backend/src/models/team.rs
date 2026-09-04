
use crate::database;
use crate::models::common::PaginationParams;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{NaiveDate, Utc, DateTime};

/// Deserializes an optional nullable field so that:
/// - key absent in JSON  → `None`           → Diesel skips the column
/// - key present, `null` → `Some(None)`     → Diesel sets the column to NULL
/// - key present, value  → `Some(Some(v))`  → Diesel sets the column to v
fn deserialize_optional_option<'de, T, D>(d: D) -> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Ok(Some(Option::deserialize(d)?))
}

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TeamWithCoach {
    pub teamid: Uuid,
    pub did: Uuid,
    pub coachid: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub quizzer_one_id: Option<Uuid>,
    pub quizzer_two_id: Option<Uuid>,
    pub quizzer_three_id: Option<Uuid>,
    pub quizzer_four_id: Option<Uuid>,
    pub quizzer_five_id: Option<Uuid>,
    pub quizzer_six_id: Option<Uuid>,
    pub coach_name: String,
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
    #[serde(default, deserialize_with = "deserialize_optional_option")]
    pub quizzer_one_id: Option<Option<Uuid>>,
    #[serde(default, deserialize_with = "deserialize_optional_option")]
    pub quizzer_two_id: Option<Option<Uuid>>,
    #[serde(default, deserialize_with = "deserialize_optional_option")]
    pub quizzer_three_id: Option<Option<Uuid>>,
    #[serde(default, deserialize_with = "deserialize_optional_option")]
    pub quizzer_four_id: Option<Option<Uuid>>,
    #[serde(default, deserialize_with = "deserialize_optional_option")]
    pub quizzer_five_id: Option<Option<Uuid>>,
    #[serde(default, deserialize_with = "deserialize_optional_option")]
    pub quizzer_six_id: Option<Option<Uuid>>,
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

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    teams
        .order(created_at)
        .limit(page_size)
        .offset(offset_val)
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

pub fn read_all_teams_where_user_is_coach(
    db: &mut database::Connection,
    user_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<Vec<Team>> {
    use crate::schema::teams::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    teams
        .filter(coachid.eq(user_id))
        .order(name.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<Team>(db)
}

pub fn read_all_teams_where_user_is_quizzer(
    db: &mut database::Connection,
    user_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<Vec<Team>> {
    use crate::schema::teams::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    teams
        .filter(
            quizzer_one_id.eq(user_id)
                .or(quizzer_two_id.eq(user_id))
                .or(quizzer_three_id.eq(user_id))
                .or(quizzer_four_id.eq(user_id))
                .or(quizzer_five_id.eq(user_id))
                .or(quizzer_six_id.eq(user_id))
        )
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

pub fn count(db: &mut database::Connection) -> QueryResult<i64> {
    use crate::schema::teams::dsl::*;
    teams.count().get_result(db)
}

pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::teams::dsl::*;
    diesel::delete(teams.filter(teamid.eq(item_id))).execute(db)
}

pub fn count_by_tournament(db: &mut database::Connection, tournament_id: Uuid) -> QueryResult<i64> {
    let division_ids: Vec<Uuid> = {
        use crate::schema::divisions::dsl::*;
        divisions
            .filter(tid.eq(tournament_id))
            .select(did)
            .load::<Uuid>(db)?
    };

    if division_ids.is_empty() {
        return Ok(0);
    }

    use crate::schema::teams::dsl::*;
    teams.filter(did.eq_any(&division_ids)).count().get_result(db)
}

pub fn read_all_teams_of_tournament(
    db: &mut database::Connection,
    tournament_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<Vec<TeamWithCoach>> {
    let division_ids: Vec<Uuid> = {
        use crate::schema::divisions::dsl::*;
        divisions
            .filter(tid.eq(tournament_id))
            .select(did)
            .load::<Uuid>(db)?
    };

    if division_ids.is_empty() {
        return Ok(vec![]);
    }

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let team_list: Vec<Team> = {
        use crate::schema::teams::dsl::*;
        teams
            .filter(did.eq_any(&division_ids))
            .order(name.asc())
            .limit(page_size)
            .offset(offset_val)
            .load::<Team>(db)?
    };

    if team_list.is_empty() {
        return Ok(vec![]);
    }

    let coach_ids: Vec<Uuid> = team_list.iter().map(|t| t.coachid).collect();
    let coach_map: HashMap<Uuid, String> = {
        use crate::schema::users::dsl::*;
        users
            .filter(id.eq_any(&coach_ids))
            .load::<crate::models::user::User>(db)?
            .into_iter()
            .map(|u| (u.id, format!("{} {} {}", u.fname, u.mname, u.lname)))
            .collect()
    };

    Ok(team_list
        .into_iter()
        .map(|t| {
            let coach_name = coach_map.get(&t.coachid).cloned().unwrap_or_default();
            TeamWithCoach {
                teamid: t.teamid,
                did: t.did,
                coachid: t.coachid,
                name: t.name,
                created_at: t.created_at,
                updated_at: t.updated_at,
                quizzer_one_id: t.quizzer_one_id,
                quizzer_two_id: t.quizzer_two_id,
                quizzer_three_id: t.quizzer_three_id,
                quizzer_four_id: t.quizzer_four_id,
                quizzer_five_id: t.quizzer_five_id,
                quizzer_six_id: t.quizzer_six_id,
                coach_name,
            }
        })
        .collect())
}

pub fn read_all_quizzers_of_tournament(
    db: &mut database::Connection,
    tournament_id: Uuid,
) -> QueryResult<Vec<crate::models::user::User>> {
    let division_ids: Vec<Uuid> = {
        use crate::schema::divisions::dsl::*;
        divisions
            .filter(tid.eq(tournament_id))
            .select(did)
            .load::<Uuid>(db)?
    };

    if division_ids.is_empty() {
        return Ok(vec![]);
    }

    let team_list: Vec<Team> = {
        use crate::schema::teams::dsl::*;
        teams
            .filter(did.eq_any(&division_ids))
            .load::<Team>(db)?
    };

    if team_list.is_empty() {
        return Ok(vec![]);
    }

    let quizzer_ids: Vec<Uuid> = team_list
        .iter()
        .flat_map(|t| {
            [
                t.quizzer_one_id,
                t.quizzer_two_id,
                t.quizzer_three_id,
                t.quizzer_four_id,
                t.quizzer_five_id,
                t.quizzer_six_id,
            ]
            .into_iter()
            .flatten()
        })
        .collect();

    if quizzer_ids.is_empty() {
        return Ok(vec![]);
    }

    use crate::schema::users::dsl::*;
    users
        .filter(id.eq_any(&quizzer_ids))
        .order((fname.asc(), mname.asc(), lname.asc()))
        .load::<crate::models::user::User>(db)
}

/// A referenced entity (division or team) shown as a link in the quizzers data table.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct EntityRefDto {
    pub id: Uuid,
    pub name: String,
}

/// One fully-formed row of the quizzers data table: the quizzer's (non-sensitive) user
/// fields plus the divisions and teams they belong to within the requested scope. This lets
/// the data table be populated with a single API call.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct QuizzerRow {
    pub id: Uuid,
    pub username: Option<String>,
    pub email: String,
    pub fname: String,
    pub mname: String,
    pub lname: String,
    pub activated: bool,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Utc>,
    pub divisions: Vec<EntityRefDto>,
    pub teams: Vec<EntityRefDto>,
}

/// Returns the quizzer-table rows for every quizzer rostered on any team in the tournament.
pub fn read_quizzer_rows_of_tournament(
    db: &mut database::Connection,
    tournament_id: Uuid,
) -> QueryResult<Vec<QuizzerRow>> {
    let div_pairs: Vec<(Uuid, String)> = {
        use crate::schema::divisions::dsl::*;
        divisions
            .filter(tid.eq(tournament_id))
            .select((did, dname))
            .load::<(Uuid, String)>(db)?
    };
    let div_ids: Vec<Uuid> = div_pairs.iter().map(|(d, _)| *d).collect();
    let div_name_by_id: HashMap<Uuid, String> = div_pairs.into_iter().collect();

    let team_list: Vec<Team> = if div_ids.is_empty() {
        Vec::new()
    } else {
        use crate::schema::teams::dsl::*;
        teams.filter(did.eq_any(&div_ids)).load::<Team>(db)?
    };

    build_quizzer_rows(db, team_list, &div_name_by_id)
}

/// Returns the quizzer-table rows for every quizzer rostered on any team in the division.
pub fn read_quizzer_rows_of_division(
    db: &mut database::Connection,
    division_id: Uuid,
) -> QueryResult<Vec<QuizzerRow>> {
    let dname_val: String = {
        use crate::schema::divisions::dsl::*;
        divisions.filter(did.eq(division_id)).select(dname).first::<String>(db)?
    };
    let mut div_name_by_id: HashMap<Uuid, String> = HashMap::new();
    div_name_by_id.insert(division_id, dname_val);

    let team_list: Vec<Team> = {
        use crate::schema::teams::dsl::*;
        teams.filter(did.eq(division_id)).load::<Team>(db)?
    };

    build_quizzer_rows(db, team_list, &div_name_by_id)
}

/// Shared assembler: given a set of teams (and a lookup of their division names), builds one
/// `QuizzerRow` per distinct quizzer, aggregating the teams/divisions they belong to and
/// ordering the rows alphabetically by name.
fn build_quizzer_rows(
    db: &mut database::Connection,
    team_list: Vec<Team>,
    div_name_by_id: &HashMap<Uuid, String>,
) -> QueryResult<Vec<QuizzerRow>> {
    use std::collections::HashSet;

    let mut teams_by_q: HashMap<Uuid, Vec<EntityRefDto>> = HashMap::new();
    let mut divs_by_q: HashMap<Uuid, Vec<EntityRefDto>> = HashMap::new();
    let mut quizzer_ids: Vec<Uuid> = Vec::new();
    let mut seen: HashSet<Uuid> = HashSet::new();

    for team in &team_list {
        let div_name = div_name_by_id.get(&team.did).cloned().unwrap_or_default();
        let slots = [
            team.quizzer_one_id, team.quizzer_two_id, team.quizzer_three_id,
            team.quizzer_four_id, team.quizzer_five_id, team.quizzer_six_id,
        ];
        for qid in slots.into_iter().flatten() {
            if seen.insert(qid) {
                quizzer_ids.push(qid);
            }
            teams_by_q.entry(qid).or_default().push(EntityRefDto {
                id: team.teamid,
                name: team.name.clone(),
            });
            let dlist = divs_by_q.entry(qid).or_default();
            if !dlist.iter().any(|e| e.id == team.did) {
                dlist.push(EntityRefDto { id: team.did, name: div_name.clone() });
            }
        }
    }

    if quizzer_ids.is_empty() {
        return Ok(Vec::new());
    }

    let users_list: Vec<crate::models::user::User> = {
        use crate::schema::users::dsl::*;
        users
            .filter(id.eq_any(&quizzer_ids))
            .order((fname.asc(), mname.asc(), lname.asc()))
            .load::<crate::models::user::User>(db)?
    };

    let rows = users_list
        .into_iter()
        .map(|u| {
            let uid = u.id;
            QuizzerRow {
                id: u.id,
                username: u.username,
                email: u.email,
                fname: u.fname,
                mname: u.mname,
                lname: u.lname,
                activated: u.activated,
                created_at: u.created_at,
                updated_at: u.updated_at,
                divisions: divs_by_q.remove(&uid).unwrap_or_default(),
                teams: teams_by_q.remove(&uid).unwrap_or_default(),
            }
        })
        .collect();

    Ok(rows)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TeamWithTournamentInfo {
    pub teamid: Uuid,
    pub name: String,
    pub coachid: Uuid,
    pub coach_name: String,
    pub did: Uuid,
    pub tournament_id: Uuid,
    pub tournament_name: String,
    pub tournament_fromdate: NaiveDate,
    pub tournament_todate: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

fn enrich_teams_with_tournament_info(
    db: &mut database::Connection,
    team_list: Vec<Team>,
) -> QueryResult<Vec<TeamWithTournamentInfo>> {
    if team_list.is_empty() {
        return Ok(vec![]);
    }

    let did_list: Vec<Uuid> = team_list.iter().map(|t| t.did).collect();

    // did -> tid mapping
    let did_to_tid_map: HashMap<Uuid, Uuid> = {
        use crate::schema::divisions::dsl::*;
        divisions
            .filter(did.eq_any(&did_list))
            .load::<crate::models::division::Division>(db)?
            .into_iter()
            .map(|d| (d.did, d.tid))
            .collect()
    };

    let tid_list: Vec<Uuid> = did_to_tid_map.values().copied().collect();
    let tournament_info_map: HashMap<Uuid, (String, NaiveDate, NaiveDate)> = {
        use crate::schema::tournaments::dsl::*;
        tournaments
            .filter(tid.eq_any(&tid_list))
            .load::<crate::models::tournament::Tournament>(db)?
            .into_iter()
            .map(|t| (t.tid, (t.tname, t.fromdate, t.todate)))
            .collect()
    };

    let coach_ids: Vec<Uuid> = team_list.iter().map(|t| t.coachid).collect();
    let coach_map: HashMap<Uuid, String> = {
        use crate::schema::users::dsl::*;
        users
            .filter(id.eq_any(&coach_ids))
            .load::<crate::models::user::User>(db)?
            .into_iter()
            .map(|u| (u.id, format!("{} {} {}", u.fname, u.mname, u.lname).trim().to_string()))
            .collect()
    };

    Ok(team_list.into_iter().filter_map(|t| {
        let tournament_id = *did_to_tid_map.get(&t.did)?;
        let (tournament_name, tournament_fromdate, tournament_todate) =
            tournament_info_map.get(&tournament_id)?;
        let coach_name = coach_map.get(&t.coachid).cloned().unwrap_or_default();
        Some(TeamWithTournamentInfo {
            teamid: t.teamid,
            name: t.name,
            coachid: t.coachid,
            coach_name,
            did: t.did,
            tournament_id,
            tournament_name: tournament_name.clone(),
            tournament_fromdate: *tournament_fromdate,
            tournament_todate: *tournament_todate,
            created_at: t.created_at,
            updated_at: t.updated_at,
        })
    }).collect())
}

pub fn read_all_teams_where_user_is_quizzer_enriched(
    db: &mut database::Connection,
    user_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<Vec<TeamWithTournamentInfo>> {
    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let team_list: Vec<Team> = {
        use crate::schema::teams::dsl::*;
        teams
            .filter(
                quizzer_one_id.eq(user_id)
                    .or(quizzer_two_id.eq(user_id))
                    .or(quizzer_three_id.eq(user_id))
                    .or(quizzer_four_id.eq(user_id))
                    .or(quizzer_five_id.eq(user_id))
                    .or(quizzer_six_id.eq(user_id))
            )
            .order(name.asc())
            .limit(page_size)
            .offset(offset_val)
            .load::<Team>(db)?
    };

    enrich_teams_with_tournament_info(db, team_list)
}

pub fn read_all_teams_where_user_is_coach_enriched(
    db: &mut database::Connection,
    user_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<Vec<TeamWithTournamentInfo>> {
    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let team_list: Vec<Team> = {
        use crate::schema::teams::dsl::*;
        teams
            .filter(coachid.eq(user_id))
            .order(name.asc())
            .limit(page_size)
            .offset(offset_val)
            .load::<Team>(db)?
    };

    enrich_teams_with_tournament_info(db, team_list)
}
