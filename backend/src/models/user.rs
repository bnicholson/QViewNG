
use crate::database;
use crate::models::roster_coach::RosterCoach;
use crate::models::roster_quizzer::RosterQuizzer;
use crate::models::tournament_admin::TournamentAdmin;
use bcrypt::{DEFAULT_COST, hash};
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable,Identifiable};
use serde::{Deserialize, Serialize};
use crate::models::common::*;
use utoipa::ToSchema;
use chrono::{DateTime,Utc};
use uuid::Uuid;

pub struct UserBuilder {
    email: Option<String>,
    hash_password: Option<String>,
    activated: Option<bool>,
    fname: String,
    mname: Option<String>,
    lname: Option<String>,
    username: Option<String>,
    created_by_userid: Option<Uuid>,
}

impl UserBuilder {
    pub fn new(fname: &str) -> Self {
        Self {
            email: None,
            hash_password: None,     
            activated: None,            
            fname: fname.to_string(),
            mname: None,
            lname: None,
            username: None,
            created_by_userid: None,
        }
    }
    pub fn new_default(fname: &str) -> Self {
        Self {
            email: Some("obviously@fakeemail.com".to_string()),
            hash_password: None,
            activated: Some(true),
            fname: fname.to_string(),
            mname: Some("Maurice".to_string()),
            lname: Some("Den".to_string()),
            username: Some("1denmanforthejob1".to_string()),
            created_by_userid: None,
        }
    }
    pub fn set_email(mut self, email: &str) -> Self {
        self.email = Some(email.to_string());
        self
    }
    pub fn set_hash_password(mut self, hash_password: &str) -> Self {
        self.hash_password = Some(hash_password.to_string());
        self
    }
    pub fn set_activated(mut self, activated: bool) -> Self {
        self.activated = Some(activated);
        self
    }
    pub fn set_mname(mut self, mname: &str) -> Self {
        self.mname = Some(mname.to_string());
        self
    }
    pub fn set_lname(mut self, lname: &str) -> Self {
        self.lname = Some(lname.to_string());
        self
    }
    pub fn set_username(mut self, username: &str) -> Self {
        self.username = Some(username.to_string());
        self
    }
    pub fn set_created_by_userid(mut self, created_by_userid: Uuid) -> Self {
        self.created_by_userid = Some(created_by_userid);
        self
    }
    fn validate_all_are_some(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if self.email.is_none() {
            errors.push("email is required".to_string());
        }
        // if self.hash_password.is_none() {
        //     errors.push("hash_password is required".to_string());
        // }
        if self.activated.is_none() {
            errors.push("set_activated is required".to_string());
        }
        if self.lname.is_none() {
            errors.push("lname is required".to_string());
        }
        // if self.username.is_none() {
        //     errors.push("username is required".to_string());
        // }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
    pub fn build(self) -> Result<NewUser, Vec<String>> {
        match self.validate_all_are_some() {
            Err(e) => {
                Err(e)
            },
            Ok(_) => {
                Ok(
                    NewUser {
                        email: self.email.unwrap(),
                        hash_password: self.hash_password.unwrap_or("".to_string()),     
                        activated: self.activated.unwrap(),            
                        fname: self.fname,            
                        mname: self.mname.unwrap_or("".to_string()),           
                        lname: self.lname.unwrap(),
                        username: self.username.unwrap_or("".to_string()),
                        created_by_userid: self.created_by_userid,
                    }
                )
            }
        }
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<User> {
        let new_user = self.build();
        create(db, new_user.unwrap())
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
#[diesel(table_name = crate::schema::users)]
#[diesel(primary_key(id))]
pub struct User {
    pub email: String,
    pub activated: bool,            
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub fname: String,            
    pub mname: String,            
    pub lname: String,            
    pub id: Uuid,            
    pub is_merged_user_id: Option<Uuid>,
    pub when_merged: Option<DateTime<Utc>>,
    pub username: Option<String>,
    pub hash_password: Option<String>,
    pub del_fl: bool,
    pub created_by_userid: Option<Uuid>,
}

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug
)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub email: String,
    pub hash_password: String,     
    pub activated: bool,            
    pub fname: String,            
    pub mname: String,            
    pub lname: String,
    pub username: String,
    // The coach who created this account on someone's behalf; None for self-registration.
    #[serde(default)]
    pub created_by_userid: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(primary_key(id))]
pub struct UserChangeset {
    pub email: Option<String>,
    pub hash_password: Option<String>,     
    pub activated: Option<bool>,            
    pub fname: Option<String>,            
    pub mname: Option<String>,            
    pub lname: Option<String>,            
    pub username: Option<String>,
    pub is_merged_user_id: Option<Uuid>,
    pub when_merged: Option<DateTime<Utc>>
}

fn get_hashed_pwd_version(item: NewUser) -> NewUser {
    let hashed_pwd = hash(&item.hash_password, DEFAULT_COST).expect("Password hashing failed");

    NewUser {
        hash_password: hashed_pwd,
        ..item
    }
}

pub fn create(db: &mut database::Connection, item: NewUser) -> QueryResult<User> {

    let item_with_hashed_password = get_hashed_pwd_version(item);

    use crate::schema::users::dsl::*;
    insert_into(users).values(item_with_hashed_password).get_result::<User>(db)
}

pub fn exists(db: &mut database::Connection, id: Uuid) -> bool {
    use crate::schema::users::dsl::users;
    users
        .find(id)
        .get_result::<User>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<User> {
    use crate::schema::users::dsl::*;
    users.filter(id.eq(item_id)).first::<User>(db)
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<User>> {
    use crate::schema::users::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    // Alphabetical by display name (first, then last, then middle) so consumers such as the
    // Coach dropdown get a sorted list straight from the API.
    users
        .order((fname.asc(), lname.asc(), mname.asc()))
        .limit(page_size)
        .offset(offset_val)
        .load::<User>(db)
}

/// Whether `user_id` has the named role (e.g. "tournament_manager", "super_user").
fn user_has_role(db: &mut database::Connection, user_id_val: Uuid, role_name: &str) -> QueryResult<bool> {
    let role = match crate::models::role::read_by_name(db, role_name) {
        Ok(r) => r,
        Err(_) => return Ok(false),
    };
    use crate::schema::users_roles::dsl::*;
    let count: i64 = users_roles
        .filter(user_id.eq(user_id_val))
        .filter(role_id.eq(role.id))
        .count()
        .get_result(db)?;
    Ok(count > 0)
}

/// Users eligible to be assigned as quizzers on a team, from the perspective of `user_id`:
///   1. the user's "My Quizzers" set — quizzers across every roster they coach (created or shared),
///      plus quizzer accounts they created; and
///   2. if the user is a tournament manager, every participant (team coaches & quizzers, game
///      quizmasters & content judges, tournament admins & owners) of any tournament they manage;
///      a super user gets that same participant set across *all* tournaments.
///
/// Returned non-deleted and ordered alphabetically by display name, so a dropdown can use them
/// directly.
pub fn read_eligible_quizzers_for_user(db: &mut database::Connection, user_id_val: Uuid) -> QueryResult<Vec<User>> {
    use std::collections::HashSet;
    let mut ids: HashSet<Uuid> = HashSet::new();

    // (1) "My Quizzers": quizzers across the coach's rosters, plus quizzer accounts they created.
    {
        let shared_roster_ids: Vec<Uuid> = {
            use crate::schema::rosters_coaches::dsl::*;
            rosters_coaches.filter(coachid.eq(user_id_val)).select(rosterid).load::<Uuid>(db)?
        };
        let roster_ids: Vec<Uuid> = {
            use crate::schema::rosters::dsl::*;
            rosters
                .filter(created_by_userid.eq(user_id_val).or(rosterid.eq_any(&shared_roster_ids)))
                .filter(del_fl.eq(false))
                .select(rosterid)
                .load::<Uuid>(db)?
        };
        let quizzer_ids: Vec<Uuid> = {
            use crate::schema::rosters_quizzers::dsl::*;
            rosters_quizzers.filter(rosterid.eq_any(&roster_ids)).select(quizzerid).load::<Uuid>(db)?
        };
        ids.extend(quizzer_ids);
        let created_ids: Vec<Uuid> = {
            use crate::schema::users::dsl::*;
            users.filter(created_by_userid.eq(user_id_val)).filter(del_fl.eq(false)).select(id).load::<Uuid>(db)?
        };
        ids.extend(created_ids);
    }

    // (2) Participants of managed tournaments (managers) or all tournaments (super users).
    let is_super = user_has_role(db, user_id_val, crate::models::role::AppRole::SuperUser.as_str())?;
    let is_manager = user_has_role(db, user_id_val, crate::models::role::AppRole::TournamentManager.as_str())?;
    if is_super || is_manager {
        let tour_ids: Vec<Uuid> = if is_super {
            use crate::schema::tournaments::dsl::*;
            tournaments.select(tid).load::<Uuid>(db)?
        } else {
            let mut owned: Vec<Uuid> = {
                use crate::schema::tournaments::dsl::*;
                tournaments.filter(owner_id.eq(user_id_val)).select(tid).load::<Uuid>(db)?
            };
            let admin_of: Vec<Uuid> = {
                use crate::schema::tournaments_admins::dsl::*;
                tournaments_admins.filter(adminid.eq(user_id_val)).select(tournamentid).load::<Uuid>(db)?
            };
            owned.extend(admin_of);
            owned.sort();
            owned.dedup();
            owned
        };

        if !tour_ids.is_empty() {
            // Tournament owners.
            {
                use crate::schema::tournaments::dsl::*;
                let owners: Vec<Uuid> = tournaments.filter(tid.eq_any(&tour_ids)).select(owner_id).load::<Uuid>(db)?;
                ids.extend(owners);
            }
            // Tournament admins.
            {
                use crate::schema::tournaments_admins::dsl::*;
                let admins: Vec<Uuid> = tournaments_admins.filter(tournamentid.eq_any(&tour_ids)).select(adminid).load::<Uuid>(db)?;
                ids.extend(admins);
            }
            // Team coaches and quizzers, across the tournaments' divisions.
            let div_ids: Vec<Uuid> = {
                use crate::schema::divisions::dsl::*;
                divisions.filter(tid.eq_any(&tour_ids)).select(did).load::<Uuid>(db)?
            };
            if !div_ids.is_empty() {
                use crate::schema::teams::dsl::*;
                let rows = teams
                    .filter(did.eq_any(&div_ids))
                    .filter(del_fl.eq(false))
                    .select((coachid, quizzer_one_id, quizzer_two_id, quizzer_three_id, quizzer_four_id, quizzer_five_id, quizzer_six_id))
                    .load::<(Uuid, Option<Uuid>, Option<Uuid>, Option<Uuid>, Option<Uuid>, Option<Uuid>, Option<Uuid>)>(db)?;
                for (coach, q1, q2, q3, q4, q5, q6) in rows {
                    ids.insert(coach);
                    for q in [q1, q2, q3, q4, q5, q6].into_iter().flatten() {
                        ids.insert(q);
                    }
                }
            }
            // Game quizmasters and content judges. Games no longer store the tournament, so we
            // scope them through the pool bracket chain: game -> pool_bracket -> division_session,
            // keeping those whose session belongs to one of the tournaments' divisions.
            if !div_ids.is_empty() {
                use crate::schema::{games, pool_brackets, division_sessions};
                let rows = games::table
                    .inner_join(pool_brackets::table.on(games::poolbracket_id.eq(pool_brackets::pool_bracket_id)))
                    .inner_join(division_sessions::table.on(pool_brackets::division_session_id.eq(division_sessions::division_session_id)))
                    .filter(division_sessions::did.eq_any(&div_ids))
                    .filter(games::del_fl.eq(false))
                    .select((games::quizmasterid, games::contentjudgeid))
                    .load::<(Uuid, Option<Uuid>)>(db)?;
                for (qm, cj) in rows {
                    ids.insert(qm);
                    if let Some(c) = cj { ids.insert(c); }
                }
            }
        }
    }

    let id_vec: Vec<Uuid> = ids.into_iter().collect();
    if id_vec.is_empty() {
        return Ok(Vec::new());
    }
    use crate::schema::users::dsl::*;
    users
        .filter(id.eq_any(&id_vec))
        .filter(del_fl.eq(false))
        .order((fname.asc(), lname.asc(), mname.asc()))
        .load::<User>(db)
}

pub fn read_all_admins_of_tournament(
    db: &mut database::Connection,
    tour_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<Vec<User>> {
    use crate::schema::users::dsl::*;
    use crate::schema::tournaments_admins::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let admin_ids: Vec<Uuid> = 
        tournaments_admins
            .filter(tournamentid.eq(tour_id))
            .load::<TournamentAdmin>(db)
            .unwrap()
            .iter()
            .map(|admin| admin.adminid)
            .collect();

    users
        .filter(id.eq_any(admin_ids))
        .order(fname.asc())
        .order(lname.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<User>(db)
}

pub fn read_all_coaches_of_roster(
    db: &mut database::Connection,
    roster_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<Vec<User>> {
    use crate::schema::users::dsl::*;
    use crate::schema::rosters_coaches::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let coach_ids: Vec<Uuid> = 
        rosters_coaches
            .filter(rosterid.eq(roster_id))
            .load::<RosterCoach>(db)
            .unwrap()
            .iter()
            .map(|rostercoach| rostercoach.coachid)
            .collect();

    users
        .filter(id.eq_any(coach_ids))
        .order(fname.asc())
        .order(lname.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<User>(db)
}

pub fn read_all_quizzers_of_roster(
    db: &mut database::Connection,
    roster_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<Vec<User>> {
    use crate::schema::users::dsl::*;
    use crate::schema::rosters_quizzers::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let quizzer_ids: Vec<Uuid> = 
        rosters_quizzers
            .filter(rosterid.eq(roster_id))
            .load::<RosterQuizzer>(db)
            .unwrap()
            .iter()
            .map(|rosterquizzer| rosterquizzer.quizzerid)
            .collect();

    users
        .filter(id.eq_any(quizzer_ids))
        .order(fname.asc())
        .order(lname.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<User>(db)
}

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &UserChangeset) -> QueryResult<User> {
    use crate::schema::users::dsl::*;
    diesel::update(users.filter(id.eq(item_id)))
        .set((
            item,
            updated_at.eq(diesel::dsl::now),
        ))
        .get_result(db)
}

pub fn count(db: &mut database::Connection) -> QueryResult<i64> {
    use crate::schema::users::dsl::*;
    users.count().get_result(db)
}

/// Soft delete: mark the user deactivated (del_fl = true) without removing the row. Their name
/// still resolves in data tables and references; only the profile view is gated on this flag.
pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::users::dsl::*;
    diesel::update(users.filter(id.eq(item_id))).set(del_fl.eq(true)).execute(db)
}

/// Purge: permanently remove the user row from the database.
pub fn purge(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::users::dsl::*;
    diesel::delete(users.filter(id.eq(item_id))).execute(db)
}

pub fn find_by_email_or_username(db: &mut database::Connection, identifier: &str) -> QueryResult<User> {
    use crate::schema::users::dsl::*;

    if identifier.trim().is_empty() {
        return Err(diesel::result::Error::NotFound);
    }

    users
        .filter(email.eq(identifier).or(username.eq(identifier)))
        .first::<User>(db)
}

/// Finds the single user matching BOTH the given username and email. Used by account recovery
/// so a reset can only be requested when the username/email pair identifies one specific account.
pub fn find_by_username_and_email(db: &mut database::Connection, username_val: &str, email_val: &str) -> QueryResult<User> {
    use crate::schema::users::dsl::*;

    if username_val.trim().is_empty() || email_val.trim().is_empty() {
        return Err(diesel::result::Error::NotFound);
    }

    users
        .filter(username.eq(username_val).and(email.eq(email_val)))
        .first::<User>(db)
}

pub fn change_password(db: &mut database::Connection, user_id: Uuid, new_hash: &str) -> QueryResult<User> {
    use crate::schema::users::dsl::*;
    diesel::update(users.filter(id.eq(user_id)))
        .set((hash_password.eq(new_hash), updated_at.eq(diesel::dsl::now)))
        .get_result(db)
}

pub fn activate_user(db: &mut database::Connection, user_id: Uuid) -> QueryResult<User> {
    use crate::schema::users::dsl::*;
    diesel::update(users.filter(id.eq(user_id)))
        .set((activated.eq(true), updated_at.eq(diesel::dsl::now)))
        .get_result(db)
}

/// Batch-loads display names ("fname mname lname", empties skipped) for a set of user ids.
/// Used to enrich data-table rows with a "Last Modified By" name in a single query.
pub fn read_display_names(
    db: &mut database::Connection,
    user_ids: &[uuid::Uuid],
) -> QueryResult<std::collections::HashMap<uuid::Uuid, String>> {
    use crate::schema::users::dsl::*;
    if user_ids.is_empty() {
        return Ok(std::collections::HashMap::new());
    }
    Ok(users
        .filter(id.eq_any(user_ids))
        .load::<User>(db)?
        .into_iter()
        .map(|u| {
            let full = [u.fname, u.mname, u.lname]
                .into_iter()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join(" ");
            (u.id, full)
        })
        .collect())
}
