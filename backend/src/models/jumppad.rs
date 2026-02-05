
use crate::{database, models};
use crate::models::common::PaginationParams;
use crate::models::equipment_dbo::{EquipmentDbo, EquipmentDboBuilder, EquipmentDboChangeset, NewEquipmentDbo};
use diesel::dsl::insert_into;
use diesel::{prelude::*, result};
use diesel::{QueryResult,AsChangeset,Insertable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{Utc,DateTime};

#[derive(Debug, Clone)]
pub struct JumpPadBuilder {
    color: Option<String>,
    equipmentsetid: i64,
    misc_note: Option<String>,
}

impl JumpPadBuilder {
    pub fn new(equipmentsetid: i64) -> Self {
        Self {
            color: None,
            equipmentsetid,
            misc_note: None,
        }
    }
    pub fn new_default(equipmentsetid: i64) -> Self {
        Self {
            color: Some(String::new()),
            equipmentsetid,
            misc_note: None,
        }
    }
    pub fn set_color(mut self, color: Option<String>) -> Self {
        self.color = color;
        self
    }
    pub fn set_equipmentsetid(mut self, equipmentsetid: i64) -> Self {
        self.equipmentsetid = equipmentsetid;
        self
    }
    pub fn set_misc_note(mut self, misc_note: Option<String>) -> Self {
        self.misc_note = misc_note;
        self
    }
    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.color.is_none() {
            errors.push("color is required".to_string());
        }

        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
    pub fn build(self) -> Result<NewJumpPad, Vec<String>> {
        match self.validate() {
            Err(e) => {
                Err(e)
            },
            Ok(_) => {
                Ok(
                    NewJumpPad {
                        color: self.color.unwrap(),
                        
                        equipmentsetid: self.equipmentsetid,
                        misc_note: self.misc_note,
                    }
                )
            }
        }
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<JumpPad> {
        let new_jumppad = self.build();
        create(db, &new_jumppad.unwrap())
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
#[diesel(table_name = crate::schema::jumppads)]
#[diesel(primary_key(jumppadid))]
struct JumpPadDbo {
    pub jumppadid: i64,
    pub color: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl JumpPadDbo {
    pub fn to_model(&self, equipment_dbo: EquipmentDbo) -> JumpPad {
        JumpPad {
            jumppadid: self.jumppadid,
            color: self.color.clone(),
            
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
pub struct JumpPad {
    pub jumppadid: i64,
    pub color: String,
    
    pub equipmentid: i64,
    pub equipmentsetid: i64,
    pub misc_note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl JumpPad {
    pub fn to_dbo(&self) -> JumpPadDbo {
        JumpPadDbo {
            jumppadid: self.jumppadid.clone(),
            color: self.color.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug
)]
#[diesel(table_name = crate::schema::jumppads)]
struct NewJumpPadDbo {
    pub color: String,
}
impl NewJumpPadDbo {
    fn to_model(&self, new_equipment_dbo: NewEquipmentDbo) -> NewJumpPad {
        NewJumpPad {
            equipmentsetid: new_equipment_dbo.equipmentsetid,
            color: self.color.clone(),
            misc_note: new_equipment_dbo.misc_note,
        }
    }
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone
)]
pub struct NewJumpPad {
    pub color: String,

    pub equipmentsetid: i64,
    pub misc_note: Option<String>,
}
impl NewJumpPad {
    fn to_dbo(&self) -> NewJumpPadDbo {
        NewJumpPadDbo {
            color: self.color.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::jumppads)]
#[diesel(primary_key(jumppadid))]
struct JumpPadDboChangeset {
    pub color: Option<String>,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone
)]
pub struct JumpPadChangeSet {
    pub color: Option<String>,

    pub equipmentsetid: Option<i64>,
    pub misc_note: Option<String>,
}
impl JumpPadChangeSet {
    fn to_dbos(&self) -> (JumpPadDboChangeset, EquipmentDboChangeset) {
        let clone_of_self = self.clone();
        (
            JumpPadDboChangeset {
                color: self.color.clone(),
            },
            EquipmentDboChangeset {
                misc_note: clone_of_self.misc_note,
                equipmentsetid: clone_of_self.equipmentsetid
            }
        )
    }
}

pub fn covert_to_model_from_dbos(jumppad_dbo: JumpPadDbo, equipment_dbo: EquipmentDbo) -> JumpPad {
    JumpPad {
        jumppadid: jumppad_dbo.jumppadid,
        color: jumppad_dbo.color,

        equipmentid: equipment_dbo.id,
        equipmentsetid: equipment_dbo.equipmentsetid,
        misc_note: equipment_dbo.misc_note,
        created_at: equipment_dbo.created_at,
        updated_at: equipment_dbo.updated_at,
    }
}

pub fn create(db: &mut database::Connection, item: &NewJumpPad) -> QueryResult<JumpPad> {
    use crate::schema::jumppads::dsl::*;

    let misc_note = item.clone().misc_note;
    let equipmentsetid = item.equipmentsetid;
    let new_jumppad_dbo = item.to_dbo();

    let jumppad_dbo_result = 
        insert_into(jumppads)
            .values(new_jumppad_dbo)
            .get_result::<JumpPadDbo>(db);

    if jumppad_dbo_result.is_err() {
        return Err(jumppad_dbo_result.err().unwrap());
    }

    let jumppad_dbo = jumppad_dbo_result.unwrap();

    let equipment_dbo_result  = 
        EquipmentDboBuilder::new()
            .set_jumppadid(Some(jumppad_dbo.jumppadid))
            .set_misc_note(misc_note)
            .set_equipmentsetid(Some(equipmentsetid))
            .build_and_insert(db);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo = equipment_dbo_result.unwrap();

    Ok(jumppad_dbo.to_model(equipment_dbo))
}


pub fn exists(db: &mut database::Connection, jumppad_id: i64) -> bool {
    use crate::schema::jumppads::dsl::*;
    jumppads
        .find(jumppad_id)
        .get_result::<JumpPadDbo>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, equipment_dbo_id: i64) -> QueryResult<JumpPad> {
    use crate::schema::jumppads::dsl::*;

    let equipment_dbo_result = models::equipment_dbo::read(db, equipment_dbo_id);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo = equipment_dbo_result.unwrap();

    if equipment_dbo.jumppadid.is_none() {
        println!("EquipmentDbo of ID {} has no jumppadid. Could not retrieve jumppad from DB.", equipment_dbo.id);
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: EquipmentDbo of ID {} has no jumppadid. Could not retrieve jumppad from DB.", equipment_dbo.id).into()
        ));
    }

    let jumppad_dbo_result = 
        jumppads
            .filter(jumppadid.eq(equipment_dbo.jumppadid.unwrap()))
            .first::<JumpPadDbo>(db);
    
    if jumppad_dbo_result.is_err() {
        return Err(jumppad_dbo_result.err().unwrap());
    }
    
    Ok(jumppad_dbo_result.unwrap().to_model(equipment_dbo))
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<JumpPad>> {
    use crate::schema::jumppads::dsl::*;
    use crate::schema::equipment::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let tuple_result: Result<Vec<(EquipmentDbo, JumpPadDbo)>, result::Error> =
        equipment
            .inner_join(jumppads)
            .order(crate::schema::jumppads::dsl::jumppadid)
            .limit(page_size)
            .offset(offset_val)
            .load::<(EquipmentDbo, JumpPadDbo)>(db);
    
    if tuple_result.is_err() {
        return Err(tuple_result.err().unwrap());
    }

    Ok(
        tuple_result
            .unwrap()
            .into_iter()
            .map(|c: (EquipmentDbo, JumpPadDbo)| c.1.to_model(c.0))
            .collect()
    )
}

pub fn update(db: &mut database::Connection, equipment_id: i64, item: &JumpPadChangeSet) -> QueryResult<JumpPad> {
    use crate::schema::jumppads::dsl::*;

    let equipment_dbo_result = models::equipment_dbo::read(db, equipment_id);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo: EquipmentDbo = equipment_dbo_result.unwrap();

    let (jumppad_dbo_changeset, equipment_dbo_changeset) = item.to_dbos();

    if equipment_dbo.jumppadid.is_none() {
        println!("EquipmentDbo's jumppadid is none. JumpPad could not be updated.");
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: EquipmentDbo's jumppadid is none. JumpPad could not be updated.").into()
        ));
    }

    let jumppad_dbo_result = 
        diesel::update(
            jumppads
                .filter(jumppadid.eq(equipment_dbo.jumppadid.unwrap())))
                .set((
                    jumppad_dbo_changeset,
                    updated_at.eq(diesel::dsl::now),
                )
        )
            .get_result::<JumpPadDbo>(db);

    if jumppad_dbo_result.is_err() {
        return Err(jumppad_dbo_result.err().unwrap());
    }

    let equipment_dbo_result = 
        models::equipment_dbo::update(db, equipment_dbo.id, &equipment_dbo_changeset);

    if equipment_dbo_result.is_err() {
        return Err(jumppad_dbo_result.err().unwrap());
    }

    Ok(
        covert_to_model_from_dbos(
            jumppad_dbo_result.unwrap(), 
            equipment_dbo_result.unwrap()
        )
    )
}

pub fn delete(db: &mut database::Connection, equipment_id: i64) -> QueryResult<(usize, usize)> {
    use crate::schema::jumppads::dsl::*;

    let equipment_dbo_result = models::equipment_dbo::read(db, equipment_id);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo: EquipmentDbo = equipment_dbo_result.unwrap();

    if equipment_dbo.jumppadid.is_none() {
        println!("EquipmentDbo's jumppadid is none. JumpPad could not be updated.");
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: EquipmentDbo's jumppadid is none. JumpPad could not be updated.").into()
        ));
    }

    let equipment_dbo_delete_result = models::equipment_dbo::delete(db, equipment_id);

    if equipment_dbo_delete_result.is_err() {
        return Err(equipment_dbo_delete_result.err().unwrap());
    }

    let jumppad_delete_result = 
        diesel::delete(jumppads.filter(jumppadid.eq(equipment_dbo.jumppadid.unwrap()))).execute(db);

    if jumppad_delete_result.is_err() {
        return Err(jumppad_delete_result.err().unwrap());
    }

    Ok((jumppad_delete_result.unwrap(), equipment_dbo_delete_result.unwrap()))
}
