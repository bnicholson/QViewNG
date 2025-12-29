// use crate::database;
// use diesel::prelude::*;
// use diesel::*;
// use diesel::{QueryResult,AsChangeset,Insertable};
// use serde::{Deserialize, Serialize};
// use crate::models::common::*;
// // this import requires this syntax (to appease rustc):
// // use crate::schema::rooms::dsl::{};

// // #[tsync::tsync]
// #[derive(
// Debug,
// Serialize,
// Deserialize,
// Clone,
// Queryable,
// Insertable,
// Identifiable,
// AsChangeset,
// )]
// #[diesel(table_name = crate::schema::rooms)]
// #[diesel(primary_key(roomid))]
// pub struct Round {
//     pub roundid: Int8,
//     pub division: Division,
//     pub scheduled_start_time: Option<DateTime<Utc>>,
//     pub created_at: DateTime<Utc>,
//     pub updated_at: DateTime<Utc>,
// }