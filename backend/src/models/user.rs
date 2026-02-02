
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
    username: Option<String>
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
            username: None
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
            username: Some("1denmanforthejob1".to_string())
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
    fn validate_all_are_some(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if self.email.is_none() {
            errors.push("email is required".to_string());
        }
        if self.hash_password.is_none() {
            errors.push("hash_password is required".to_string());
        }
        if self.activated.is_none() {
            errors.push("set_activated is required".to_string());
        }
        if self.mname.is_none() {
            errors.push("set_mname is required".to_string());
        }
        if self.lname.is_none() {
            errors.push("lname is required".to_string());
        }
        if self.username.is_none() {
            errors.push("username is required".to_string());
        }
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
                        hash_password: self.hash_password.unwrap(),     
                        activated: self.activated.unwrap(),            
                        fname: self.fname,            
                        mname: self.mname.unwrap(),           
                        lname: self.lname.unwrap(),         
                        username: self.username.unwrap()
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
    pub hash_password: String,     
    pub activated: bool,            
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub fname: String,            
    pub mname: String,            
    pub lname: String,            
    pub id: Uuid,            
    pub username: String,
    pub is_merged_user_id: Option<Uuid>,
    pub when_merged: Option<DateTime<Utc>>
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
    pub username: String  
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
    
    users
        .order(created_at)
        .limit(pagination.page_size)
        .offset(
            pagination.page
                * std::cmp::max(pagination.page_size, PaginationParams::MAX_PAGE_SIZE as i64),
        )
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

pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::users::dsl::*;
    diesel::delete(users.filter(id.eq(item_id))).execute(db)
}
