
use crate::database;
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable,Identifiable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct TournamentAdminBuilder {
    tournamentid: Uuid,
    adminid: Uuid,     
    role_description: Option<String>,            
    access_lvl: Option<i32>
}

impl TournamentAdminBuilder {
    pub fn new(tid: Uuid, admin_id: Uuid) -> Self {
        Self {
            tournamentid: tid,
            adminid: admin_id,     
            role_description: None,            
            access_lvl: None
        }
    }
    pub fn new_default(tid: Uuid, admin_id: Uuid) -> Self {
        Self {
            tournamentid: tid,
            adminid: admin_id,     
            role_description: Some("".to_string()),            
            access_lvl: Some(0)
        }
    }
    pub fn set_tournamentid(mut self, tid: Uuid) -> Self {
        self.tournamentid = tid;
        self
    }
    pub fn set_adminid(mut self, adminid: Uuid) -> Self {
        self.adminid = adminid;
        self
    }
    pub fn set_role_description(mut self, role_description: &str) -> Self {
        self.role_description = Some(role_description.to_string());
        self
    }
    pub fn set_access_lvl(mut self, access_lvl: i32) -> Self {
        self.access_lvl = Some(access_lvl);
        self
    }
    
    fn validate_all_are_some(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if self.role_description.is_none() {
            errors.push("role_description is required".to_string());
        }
        if self.access_lvl.is_none() {
            errors.push("access_lvl is required".to_string());
        }
        
        Ok(())
    }
    pub fn build(self) -> Result<NewTournamentAdmin, Vec<String>> {
        match self.validate_all_are_some() {
            Err(e) => {
                Err(e)
            },
            Ok(_) => {
                Ok(
                    NewTournamentAdmin {
                        tournamentid: self.tournamentid,
                        adminid: self.adminid,     
                        role_description: self.role_description.unwrap(),            
                        access_lvl: self.access_lvl.unwrap()
                    }
                )
            }
        }
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<TournamentAdmin> {
        let new_tournamentadmin = self.build();
        create(db, &new_tournamentadmin.unwrap())
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
#[diesel(table_name = crate::schema::tournaments_admins)]
#[diesel(primary_key(tournamentid,adminid))]
pub struct TournamentAdmin {
    pub tournamentid: Uuid,
    pub adminid: Uuid,     
    pub role_description: Option<String>,            
    pub access_lvl: i32,            
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug
)]
#[diesel(table_name = crate::schema::tournaments_admins)]
pub struct NewTournamentAdmin {
    pub tournamentid: Uuid,
    pub adminid: Uuid,     
    pub role_description: String,            
    pub access_lvl: i32
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::tournaments_admins)]
#[diesel(primary_key(tournamentid,adminid))]
pub struct TournamentAdminChangeset {
    pub role_description: String,            
    pub access_lvl: i32
}

pub fn create(db: &mut database::Connection, item: &NewTournamentAdmin) -> QueryResult<TournamentAdmin> {
    use crate::schema::tournaments_admins::dsl::*;
    insert_into(tournaments_admins).values(item).get_result::<TournamentAdmin>(db)
}

pub fn update(db: &mut database::Connection, tour_id: Uuid, user_id: Uuid, item: &TournamentAdminChangeset) -> QueryResult<TournamentAdmin> {
    use crate::schema::tournaments_admins::dsl::*;
    diesel::update(tournaments_admins
        .filter(tournamentid.eq(tour_id))
        .filter(adminid.eq(user_id)))
        .set((
            item,
            updated_at.eq(diesel::dsl::now),
        ))
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, tour_id: Uuid, user_id: Uuid) -> QueryResult<usize> {
    use crate::schema::tournaments_admins::dsl::*;
    diesel::delete(tournaments_admins
        .filter(tournamentid.eq(tour_id))
        .filter(adminid.eq(user_id)))
        .execute(db)
}
