
use crate::database;
use crate::models::tournament_admin::TournamentAdmin;
use bcrypt::{DEFAULT_COST, hash};
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable,Identifiable};
use serde::{Deserialize, Serialize};
use crate::models::common::*;
use utoipa::ToSchema;
use chrono::{DateTime,Utc};
use uuid::Uuid;

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
    pub username: String     
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
    pub username: Option<String>  
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

pub fn read_all_users_of_tournament(
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

pub fn update(db: &mut database::Connection, item_id: Uuid, item: &UserChangeset) -> QueryResult<User> {
    use crate::schema::users::dsl::*;
    diesel::update(users.filter(id.eq(item_id)))
        .set(item)
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
    use crate::schema::users::dsl::*;
    diesel::delete(users.filter(id.eq(item_id))).execute(db)
}
