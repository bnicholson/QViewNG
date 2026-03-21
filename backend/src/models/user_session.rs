use crate::database;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct UserSession {
    pub id: i64,
    pub refresh_token: String,
    pub device: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: Uuid,
}

pub fn create(db: &mut database::Connection, uid: Uuid, token: &str, device_info: Option<&str>) -> QueryResult<UserSession> {
    use crate::schema::user_sessions::dsl::*;
    diesel::insert_into(user_sessions)
        .values((
            user_id.eq(uid),
            refresh_token.eq(token),
            device.eq(device_info),
        ))
        .get_result(db)
}

pub fn find_by_token(db: &mut database::Connection, token_val: &str) -> QueryResult<UserSession> {
    use crate::schema::user_sessions::dsl::*;
    user_sessions.filter(refresh_token.eq(token_val)).first(db)
}

pub fn delete_by_token(db: &mut database::Connection, token_val: &str) -> QueryResult<usize> {
    use crate::schema::user_sessions::dsl::*;
    diesel::delete(user_sessions.filter(refresh_token.eq(token_val))).execute(db)
}

pub fn delete_by_id(db: &mut database::Connection, session_id: i64, uid: Uuid) -> QueryResult<usize> {
    use crate::schema::user_sessions::dsl::*;
    diesel::delete(user_sessions.filter(id.eq(session_id)).filter(user_id.eq(uid))).execute(db)
}

pub fn delete_all_for_user(db: &mut database::Connection, uid: Uuid) -> QueryResult<usize> {
    use crate::schema::user_sessions::dsl::*;
    diesel::delete(user_sessions.filter(user_id.eq(uid))).execute(db)
}

pub fn read_all_for_user(db: &mut database::Connection, uid: Uuid, page: i64, page_size: i64) -> QueryResult<Vec<UserSession>> {
    use crate::schema::user_sessions::dsl::*;
    let ps = page_size.min(100);
    user_sessions
        .filter(user_id.eq(uid))
        .order(created_at.desc())
        .limit(ps)
        .offset(page * ps)
        .load(db)
}

pub fn count_for_user(db: &mut database::Connection, uid: Uuid) -> QueryResult<i64> {
    use crate::schema::user_sessions::dsl::*;
    user_sessions.filter(user_id.eq(uid)).count().get_result(db)
}
