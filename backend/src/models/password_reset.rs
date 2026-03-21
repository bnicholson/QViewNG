use crate::database;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct PasswordResetToken {
    pub token: String,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub used: bool,
    pub created_at: DateTime<Utc>,
}

pub fn create(db: &mut database::Connection, uid: Uuid, tok: &str, exp: DateTime<Utc>) -> QueryResult<PasswordResetToken> {
    use crate::schema::password_reset_tokens::dsl::*;
    diesel::insert_into(password_reset_tokens)
        .values((
            token.eq(tok),
            user_id.eq(uid),
            expires_at.eq(exp),
        ))
        .get_result(db)
}

pub fn find_valid(db: &mut database::Connection, tok: &str) -> QueryResult<PasswordResetToken> {
    use crate::schema::password_reset_tokens::dsl::*;
    password_reset_tokens
        .filter(token.eq(tok))
        .filter(used.eq(false))
        .filter(expires_at.gt(diesel::dsl::now))
        .first(db)
}

pub fn mark_used(db: &mut database::Connection, tok: &str) -> QueryResult<usize> {
    use crate::schema::password_reset_tokens::dsl::*;
    diesel::update(password_reset_tokens.filter(token.eq(tok)))
        .set(used.eq(true))
        .execute(db)
}
