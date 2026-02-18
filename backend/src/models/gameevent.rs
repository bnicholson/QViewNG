use chrono::DateTime;
use diesel::{prelude::*, insert_into};
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::{database, models::common::PaginationParams};
use utoipa::ToSchema;

pub struct GameEventBuilder {
    gid: Uuid,
    question: Option<i32>,
    eventnum: Option<i32>,
    name: Option<String>,
    team: Option<i32>,
    quizzer: Option<i32>,
    event: Option<String>,
    parm1: Option<String>,
    parm2: Option<String>,
    clientts: Option<DateTime<Utc>>,
    md5digest: Option<String>,
}

impl GameEventBuilder {
    pub fn new(game_id: Uuid) -> Self {
        Self {
            gid: game_id,
            question: None,
            eventnum: None,
            name: None,
            team: None,
            quizzer: None,
            event: None,
            parm1: None,
            parm2: None,
            clientts: None,
            md5digest: None,
        }
    }
    pub fn new_default(game_id: Uuid) -> Self {
        Self {
            gid: game_id,
            question: None,
            eventnum: None,
            name: None,
            team: None,
            quizzer: None,
            event: None,
            parm1: Some("".to_string()),
            parm2: Some("".to_string()),
            clientts: Some(Utc::now()),
            md5digest: Some("".to_string()),
        }
    }
    pub fn new_empty() -> Self {
        Self {
            gid: Uuid::nil(),
            question: Some(-1),
            eventnum: Some(-1),
            name: Some("".to_string()),
            team: Some(-1),
            quizzer: Some(-1),
            event: Some("".to_string()),
            parm1: Some("".to_string()),
            parm2: Some("".to_string()),
            clientts: Some(Utc::now()),
            md5digest: Some("".to_string())
        }
    }
    pub fn set_gid(mut self, val: Uuid) -> Self {
        self.gid = val;
        self
    }
    pub fn set_question(mut self, val: Option<i32>) -> Self {
        self.question = val;
        self
    }
    pub fn set_eventnum(mut self, val: Option<i32>) -> Self {
        self.eventnum = val;
        self
    }
    pub fn set_name(mut self, val: Option<String>) -> Self {
        self.name = val;
        self
    }
    pub fn set_team(mut self, val: Option<i32>) -> Self {
        self.team = val;
        self
    }
    pub fn set_quizzer(mut self, val: Option<i32>) -> Self {
        self.quizzer = val;
        self
    }
    pub fn set_event(mut self, val: Option<String>) -> Self {
        self.event = val;
        self
    }
    pub fn set_parm1(mut self, val: Option<String>) -> Self {
        self.parm1 = val;
        self
    }
    pub fn set_parm2(mut self, val: Option<String>) -> Self {
        self.parm2 = val;
        self
    }
    pub fn set_clientts(mut self, val: Option<DateTime<Utc>>) -> Self {
        self.clientts = val;
        self
    }
    pub fn set_md5digest(mut self, val: Option<String>) -> Self {
        self.md5digest = val;
        self
    }
    fn validate_all_are_some(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.question.is_none() {
            errors.push("question is required".to_string());
        }
        if self.eventnum.is_none() {
            errors.push("eventnum is required".to_string());
        }
        if self.name.is_none() {
            errors.push("name is required".to_string());
        }
        if self.team.is_none() {
            errors.push("team is required".to_string());
        }
        if self.quizzer.is_none() {
            errors.push("quizzer is required".to_string());
        }
        if self.event.is_none() {
            errors.push("org is required".to_string());
        }
        // if self.parm1.is_none() {
        //     errors.push("parm1 is required".to_string());
        // }
        // if self.parm2.is_none() {
        //     errors.push("parm2 is required".to_string());
        // }
        if self.clientts.is_none() {
            errors.push("clientts is required".to_string());
        }
        if self.md5digest.is_none() {
            errors.push("md5digest is required".to_string());
        }

        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
    pub fn build(self) -> Result<NewGameEvent, Vec<String>> {
        match self.validate_all_are_some() {
            Err(e) => {
                Err(e)
            },
            Ok(_) => {
                Ok(
                    NewGameEvent {
                        gid: self.gid,
                        question: self.question.unwrap(),
                        eventnum: self.eventnum.unwrap(),
                        name: self.name.unwrap(),
                        team: self.team.unwrap(),
                        quizzer: self.quizzer.unwrap(),
                        event: self.event.unwrap(),
                        parm1: self.parm1.unwrap_or_else(|| "".to_string()),
                        parm2: self.parm2.unwrap_or_else(|| "".to_string()),
                        clientts: self.clientts.unwrap(),
                        serverts: Utc::now(),
                        md5digest: self.md5digest.unwrap(),
                    }
                )
            }
        }
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<GameEvent> {
        let new_entity = self.build();
        create(db, &new_entity.unwrap())
    }
}

// Now define the tables that will store each game event
// // #[tsync::tsync]
#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Identifiable,
    AsChangeset,
    ToSchema
)]
#[diesel(table_name = crate::schema::gameevents)]
#[diesel(primary_key(gid,question,eventnum))]
pub struct GameEvent {
    pub gid: Uuid,
    pub question: i32,
    pub eventnum: i32,
    pub name: String,
    pub team: i32,
    pub quizzer: i32,
    pub event: String,
    pub parm1: String,
    pub parm2: String,
    pub clientts: DateTime<Utc>,
    pub serverts: DateTime<Utc>,
    pub md5digest: String,
}

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug,
    Clone
)]
#[diesel(table_name = crate::schema::gameevents)]
pub struct NewGameEvent {
    pub gid: Uuid,
    pub question: i32,
    pub eventnum: i32,
    pub name: String,
    pub team: i32,
    pub quizzer: i32,
    pub event: String,
    pub parm1: String,
    pub parm2: String,
    pub clientts: DateTime<Utc>,
    pub serverts: DateTime<Utc>,
    pub md5digest: String,
}

// What use case would bring us to want to modify an event stream record for Games? Commenting out until further notice:
// // #[tsync::tsync]
// #[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
// #[diesel(table_name = crate::schema::gameevents)]
// #[diesel(primary_key(gid,question,eventnum))]
// pub struct GameEventChangeset {   
//     pub name: String,
//     pub team: i32,
//     pub quizzer: i32,
//     pub event: String,
//     pub parm1: String,
//     pub parm2: String,
//     pub clientts: DateTime<Utc>,
//     pub serverts: DateTime<Utc>,
//     pub md5digest: String,  
// }

// What use case would bring us to want to modify an event stream record for Games? Commenting out until further notice:
// pub fn empty_changeset() -> GameEventChangeset {
//     return GameEventChangeset {   
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

pub fn create(db: &mut database::Connection, item: &NewGameEvent) -> QueryResult<GameEvent> {
    use crate::schema::gameevents::dsl::*;
    insert_into(gameevents).values(item).get_result::<GameEvent>(db)
}

pub fn read(db: &mut database::Connection, item_id: Uuid) -> QueryResult<GameEvent> {
    use crate::schema::gameevents::dsl::*;
    gameevents.filter(gid.eq(item_id)).first::<GameEvent>(db)
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<GameEvent>> {
    use crate::schema::gameevents::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    gameevents
        .order(gid)
        .limit(page_size)
        .offset(offset_val)
        .load::<GameEvent>(db)
}

// Not sure what advantage this offers over fn 'create_game_event' above. Commenting out for now:
// pub fn create_update_game_event(db: &mut database::Connection, item: &GameEvent) -> QueryResult<GameEvent> {
//     use crate::schema::gameevents::dsl::*;
//     insert_into(gameevents).values(item).on_conflict(on_constraint(
//         "gameevents_pkey1"))
//         .do_update()
//         .set(item)
//         .get_result::<GameEvent>(db)
// }

// Not including a Delete fn until it is apparent that it is needed.
