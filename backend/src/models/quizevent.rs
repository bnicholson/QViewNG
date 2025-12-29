// use diesel::prelude::*;
// use diesel::upsert::*;
// use diesel::insert_into;
// use crate::models::common::{BigId,UTC};
// use chrono::{ Utc, TimeZone };
// use serde::{Deserialize, Serialize};
// use crate::database;
// use utoipa::ToSchema;
// // this import requires this syntax (to appease rustc):
// use crate::schema::quizevents::dsl::{
//     gid,question,eventnum,name,team,quizzer,event,parm1,parm2,clientts,serverts,md5digest
// };

// // Now define the tables that will store each quiz event
// // // #[tsync::tsync]
// #[derive(
//     Debug,
//     Serialize,
//     Deserialize,
//     Clone,
//     Queryable,
//     Insertable,
//     Identifiable,
//     AsChangeset,
//     ToSchema
// )]
// #[diesel(table_name = crate::schema::quizevents)]
// #[diesel(primary_key(gid,question,eventnum))]
// pub struct QuizEvent {
//     pub gid: BigId,
//     pub question: i32,
//     pub eventnum: i32,
//     pub name: String,
//     pub team: i32,
//     pub quizzer: i32,
//     pub event: String,
//     pub parm1: String,
//     pub parm2: String,
//     #[schema(value_type = String, format = DateTime)]
//     pub clientts: UTC,
//     #[schema(value_type = String, format = DateTime)]
//     pub serverts: UTC,
//     pub md5digest: String,
// }

// // // #[tsync::tsync]
// #[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
// #[diesel(table_name = crate::schema::quizevents)]
// #[diesel(primary_key(gid,question,eventnum))]
// pub struct QuizEventChangeset {   
//     pub name: String,
//     pub team: i32,
//     pub quizzer: i32,
//     pub event: String,
//     pub parm1: String,
//     pub parm2: String,
//     pub clientts: UTC,
//     pub serverts: UTC,
//     pub md5digest: String,  
// }

// pub fn empty() -> QuizEvent {
//     // Now populate the quizzes event
//     return QuizEvent {
//         gid: -1,
//         question: -1,
//         eventnum: -1,
//         name: "".to_string(),
//         team: -1,
//         quizzer: -1,
//         event: "".to_string(),
//         parm1: "".to_string(),
//         parm2: "".to_string(),
//         clientts: Utc::now(),
//         serverts: Utc::now(),
//         md5digest: "".to_string()
//     }
// }

// pub fn empty_changeset() -> QuizEventChangeset {
//     return QuizEventChangeset {   
//         name: "".to_string(),
//         team: -1,
//         quizzer: -1,
//         event: "".to_string(),
//         parm1: "".to_string(),
//         parm2: "".to_string(),
//         clientts: Utc::now(),
//         serverts: Utc::now(),
//         md5digest: "".to_string()
//     }
// }

// pub fn create_quiz_event(db: &mut database::Connection, item: &QuizEvent) -> QueryResult<QuizEvent> {
//     use crate::schema::quizevents::dsl::*;
//     insert_into(quizevents).values(item).get_result::<QuizEvent>(db)
// }

// pub fn create_update_quiz_event(db: &mut database::Connection, item: &QuizEvent) -> QueryResult<QuizEvent> {
//     use crate::schema::quizevents::dsl::*;
//     insert_into(quizevents).values(item).on_conflict(on_constraint(
//         "quizevents_pkey1"))
//         .do_update()
//         .set(item)
//         .get_result::<QuizEvent>(db)
// }