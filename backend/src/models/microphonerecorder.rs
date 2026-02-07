
use crate::{database, models};
use crate::models::common::PaginationParams;
use crate::models::equipment_dbo::{EquipmentDbo, EquipmentDboBuilder, EquipmentDboChangeset};
use diesel::dsl::insert_into;
use diesel::{prelude::*, result};
use diesel::{QueryResult,AsChangeset,Insertable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{Utc,DateTime};

#[derive(Debug, Clone)]
pub struct MicrophoneRecorderBuilder {
    type_: Option<String>,

    equipmentsetid: i64,
    misc_note: Option<String>,
}

impl MicrophoneRecorderBuilder {
    pub fn new(equipmentsetid: i64) -> Self {
        Self {
            type_: None,

            equipmentsetid,
            misc_note: None,
        }
    }
    pub fn new_default(equipmentsetid: i64) -> Self {
        Self {
            type_: None,

            equipmentsetid,
            misc_note: Some("".to_string()),
        }
    }
    pub fn set_equipmentsetid(mut self, equipmentsetid: i64) -> Self {
        self.equipmentsetid = equipmentsetid;
        self
    }
    pub fn set_type_(mut self, type_: Option<String>) -> Self {
        self.type_ = type_;
        self
    }
    pub fn set_misc_note(mut self, misc_note: Option<String>) -> Self {
        self.misc_note = misc_note;
        self
    }
    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.type_.is_none() {
            errors.push("type_ is required".to_string());
        }

        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
    pub fn build(self) -> Result<NewMicrophoneRecorder, Vec<String>> {
        match self.validate() {
            Err(e) => {
                Err(e)
            },
            Ok(_) => {
                Ok(
                    NewMicrophoneRecorder {
                        type_: self.type_.unwrap(),
                        
                        equipmentsetid: self.equipmentsetid,
                        misc_note: self.misc_note,
                    }
                )
            }
        }
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<MicrophoneRecorder> {
        let new_microphonerecorder = self.build();
        create(db, &new_microphonerecorder.unwrap())
    }
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    QueryableByName,
    Selectable,
    Identifiable,
    ToSchema
)]
#[diesel(check_for_backend(diesel::pg::Pg))]  // this shows which field has incorrect type compared to the schema.rs file
#[diesel(table_name = crate::schema::microphonerecorders)]
#[diesel(primary_key(id))]
struct MicrophoneRecorderDbo {
    pub id: i64,
    pub type_: String,  // enum options: 'External' and 'Built-In'

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl MicrophoneRecorderDbo {
    pub fn to_model(&self, equipment_dbo: EquipmentDbo) -> MicrophoneRecorder {
        MicrophoneRecorder {
            id: self.id,
            type_: self.type_.clone(),
            
            created_at: self.created_at,
            updated_at: self.updated_at,
            equipmentid: equipment_dbo.id,
            equipmentsetid: equipment_dbo.equipmentsetid,
            misc_note: equipment_dbo.misc_note,
        }
    }
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone
)]
pub struct MicrophoneRecorder {
    pub id: i64,
    pub type_: String,  // enum options: 'External' and 'Built-In'
    
    pub equipmentid: i64,
    pub equipmentsetid: i64,
    pub misc_note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug
)]
#[diesel(table_name = crate::schema::microphonerecorders)]
struct NewMicrophoneRecorderDbo {
    pub type_: String,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone
)]
pub struct NewMicrophoneRecorder {
    pub type_: String,

    pub equipmentsetid: i64,
    pub misc_note: Option<String>,
}
impl NewMicrophoneRecorder {
    fn to_dbo(&self) -> NewMicrophoneRecorderDbo {
        NewMicrophoneRecorderDbo {
            type_: self.type_.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::microphonerecorders)]
#[diesel(primary_key(id))]
struct MicrophoneRecorderDboChangeset {
    pub type_: String,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone
)]
pub struct MicrophoneRecorderChangeSet {
    pub type_: String,

    pub equipmentsetid: Option<i64>,
    pub misc_note: Option<String>,
}
impl MicrophoneRecorderChangeSet {
    fn to_dbos(&self) -> (MicrophoneRecorderDboChangeset, EquipmentDboChangeset) {
        let clone_of_self = self.clone();
        (
            MicrophoneRecorderDboChangeset {
                type_: self.type_.clone(),
            },
            EquipmentDboChangeset {
                misc_note: clone_of_self.misc_note,
                equipmentsetid: clone_of_self.equipmentsetid
            }
        )
    }
}

fn covert_to_model_from_dbos(microphonerecorder_dbo: MicrophoneRecorderDbo, equipment_dbo: EquipmentDbo) -> MicrophoneRecorder {
    MicrophoneRecorder {
        id: microphonerecorder_dbo.id,
        type_: microphonerecorder_dbo.type_.clone(),
        
        created_at: equipment_dbo.created_at,
        updated_at: equipment_dbo.updated_at,
        equipmentid: equipment_dbo.id,
        equipmentsetid: equipment_dbo.equipmentsetid,
        misc_note: equipment_dbo.misc_note,
    }
}

pub fn create(db: &mut database::Connection, item: &NewMicrophoneRecorder) -> QueryResult<MicrophoneRecorder> {
    use crate::schema::microphonerecorders::dsl::*;

    let misc_note = item.clone().misc_note;
    let equipmentsetid = item.equipmentsetid;
    let new_microphonerecorder_dbo = item.to_dbo();

    let microphonerecorder_dbo_result = 
        insert_into(microphonerecorders)
            .values(new_microphonerecorder_dbo)
            .get_result::<MicrophoneRecorderDbo>(db);

    if microphonerecorder_dbo_result.is_err() {
        return Err(microphonerecorder_dbo_result.err().unwrap());
    }

    let microphonerecorder_dbo = microphonerecorder_dbo_result.unwrap();

    let equipment_dbo_result  = 
        EquipmentDboBuilder::new()
            .set_microphonerecorderid(Some(microphonerecorder_dbo.id))
            .set_misc_note(misc_note)
            .set_equipmentsetid(Some(equipmentsetid))
            .build_and_insert(db);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo = equipment_dbo_result.unwrap();

    Ok(microphonerecorder_dbo.to_model(equipment_dbo))
}


pub fn exists(db: &mut database::Connection, microphonerecorder_id: i64) -> bool {
    use crate::schema::microphonerecorders::dsl::*;
    microphonerecorders
        .find(microphonerecorder_id)
        .get_result::<MicrophoneRecorderDbo>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, microphonerecorder_id: i64) -> QueryResult<MicrophoneRecorder> {
    use crate::schema::microphonerecorders::dsl::*;
    use crate::schema::equipment::dsl::*;

    let equipment_dbo_result = 
        equipment
            .filter(crate::schema::equipment::dsl::microphonerecorderid.eq(microphonerecorder_id))
            .first::<EquipmentDbo>(db);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo = equipment_dbo_result.unwrap();

    if equipment_dbo.microphonerecorderid.is_none() {
        println!("EquipmentDbo of ID {} has no id. Could not retrieve microphonerecorder from DB.", equipment_dbo.id);
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: EquipmentDbo of ID {} has no id. Could not retrieve microphonerecorder from DB.", equipment_dbo.id).into()
        ));
    }

    let microphonerecorder_dbo_result = 
        microphonerecorders
            .filter(crate::schema::microphonerecorders::dsl::id.eq(equipment_dbo.microphonerecorderid.unwrap()))
            .first::<MicrophoneRecorderDbo>(db);
    
    if microphonerecorder_dbo_result.is_err() {
        return Err(microphonerecorder_dbo_result.err().unwrap());
    }
    
    Ok(microphonerecorder_dbo_result.unwrap().to_model(equipment_dbo))
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<MicrophoneRecorder>> {
    use crate::schema::microphonerecorders::dsl::*;
    use crate::schema::equipment::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let tuple_result: Result<Vec<(EquipmentDbo, MicrophoneRecorderDbo)>, result::Error> =
        equipment
            .inner_join(microphonerecorders)
            .order(crate::schema::microphonerecorders::dsl::id)
            .limit(page_size)
            .offset(offset_val)
            .load::<(EquipmentDbo, MicrophoneRecorderDbo)>(db);
    
    if tuple_result.is_err() {
        return Err(tuple_result.err().unwrap());
    }

    Ok(
        tuple_result
            .unwrap()
            .into_iter()
            .map(|c: (EquipmentDbo, MicrophoneRecorderDbo)| c.1.to_model(c.0))
            .collect()
    )
}

pub fn update(db: &mut database::Connection, equipment_id: i64, item: &MicrophoneRecorderChangeSet) -> QueryResult<MicrophoneRecorder> {
    use crate::schema::microphonerecorders::dsl::*;

    let equipment_dbo_result = models::equipment_dbo::read(db, equipment_id);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo: EquipmentDbo = equipment_dbo_result.unwrap();

    let (microphonerecorder_dbo_changeset, equipment_dbo_changeset) = item.to_dbos();

    if equipment_dbo.microphonerecorderid.is_none() {
        println!("EquipmentDbo's id is none. MicrophoneRecorder could not be updated.");
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: EquipmentDbo's id is none. MicrophoneRecorder could not be updated.").into()
        ));
    }

    let microphonerecorder_dbo_result = 
        diesel::update(
            microphonerecorders
                .filter(id.eq(equipment_dbo.microphonerecorderid.unwrap())))
                .set((
                    microphonerecorder_dbo_changeset,
                    updated_at.eq(diesel::dsl::now),
                )
        )
            .get_result::<MicrophoneRecorderDbo>(db);

    if microphonerecorder_dbo_result.is_err() {
        return Err(microphonerecorder_dbo_result.err().unwrap());
    }

    let equipment_dbo_result = 
        models::equipment_dbo::update(db, equipment_dbo.id, &equipment_dbo_changeset);

    if equipment_dbo_result.is_err() {
        return Err(microphonerecorder_dbo_result.err().unwrap());
    }

    Ok(
        covert_to_model_from_dbos(
            microphonerecorder_dbo_result.unwrap(), 
            equipment_dbo_result.unwrap()
        )
    )
}

pub fn delete(db: &mut database::Connection, equipment_id: i64) -> QueryResult<(usize, usize)> {
    use crate::schema::microphonerecorders::dsl::*;

    let equipment_dbo_result = models::equipment_dbo::read(db, equipment_id);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo: EquipmentDbo = equipment_dbo_result.unwrap();

    if equipment_dbo.microphonerecorderid.is_none() {
        println!("EquipmentDbo's id is none. MicrophoneRecorder could not be updated.");
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: EquipmentDbo's id is none. MicrophoneRecorder could not be updated.").into()
        ));
    }

    let equipment_dbo_delete_result = models::equipment_dbo::delete(db, equipment_id);

    if equipment_dbo_delete_result.is_err() {
        return Err(equipment_dbo_delete_result.err().unwrap());
    }

    let microphonerecorder_delete_result = 
        diesel::delete(microphonerecorders.filter(id.eq(equipment_dbo.microphonerecorderid.unwrap()))).execute(db);

    if microphonerecorder_delete_result.is_err() {
        return Err(microphonerecorder_delete_result.err().unwrap());
    }

    Ok((microphonerecorder_delete_result.unwrap(), equipment_dbo_delete_result.unwrap()))
}
