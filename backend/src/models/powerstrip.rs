
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
pub struct PowerStripBuilder {
    make: Option<String>,
    model: Option<String>,
    color: Option<String>,
    num_of_plugs: Option<i32>,

    equipmentsetid: i64,
    misc_note: Option<String>,
}

impl PowerStripBuilder {
    pub fn new(equipmentsetid: i64) -> Self {
        Self {
            make: None,
            model: None,
            color: None,
            num_of_plugs: None,

            equipmentsetid,
            misc_note: None,
        }
    }
    pub fn new_default(equipmentsetid: i64) -> Self {
        Self {
            make: None,
            model: None,
            color: Some("black".to_string()),
            num_of_plugs: None,

            equipmentsetid,
            misc_note: Some("".to_string()),
        }
    }
    pub fn set_equipmentsetid(mut self, equipmentsetid: i64) -> Self {
        self.equipmentsetid = equipmentsetid;
        self
    }
    pub fn set_misc_note(mut self, misc_note: Option<String>) -> Self {
        self.misc_note = misc_note;
        self
    }
    pub fn set_make(mut self, make: Option<String>) -> Self {
        self.make = make;
        self
    }
    pub fn set_model(mut self, model: Option<String>) -> Self {
        self.model = model;
        self
    }
    pub fn set_color(mut self, color: Option<String>) -> Self {
        self.color = color;
        self
    }
    pub fn set_num_of_plugs(mut self, num_of_plugs: Option<i32>) -> Self {
        self.num_of_plugs = num_of_plugs;
        self
    }
    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.make.is_none() {
            errors.push("make is required".to_string());
        }
        if self.model.is_none() {
            errors.push("model is required".to_string());
        }
        if self.color.is_none() {
            errors.push("color is required".to_string());
        }
        if self.num_of_plugs.is_none() {
            errors.push("num_of_plugs is required".to_string());
        }

        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
    pub fn build(self) -> Result<NewPowerStrip, Vec<String>> {
        match self.validate() {
            Err(e) => {
                Err(e)
            },
            Ok(_) => {
                Ok(
                    NewPowerStrip {
                        make: self.make.unwrap(),
                        model: self.model.unwrap(),
                        color: self.color.unwrap(),
                        num_of_plugs: self.num_of_plugs.unwrap(),
                        
                        equipmentsetid: self.equipmentsetid,
                        misc_note: self.misc_note,
                    }
                )
            }
        }
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<PowerStrip> {
        let new_powerstrip = self.build();
        create(db, &new_powerstrip.unwrap())
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
#[diesel(table_name = crate::schema::powerstrips)]
#[diesel(primary_key(id))]
struct PowerStripDbo {
    pub id: i64,
    pub make: String,
    pub model: String,
    pub color: String,
    pub num_of_plugs: i32,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl PowerStripDbo {
    pub fn to_model(&self, equipment_dbo: EquipmentDbo) -> PowerStrip {
        PowerStrip {
            id: self.id.clone(),
            make: self.make.clone(),
            model: self.model.clone(),
            color: self.color.clone(),
            num_of_plugs: self.num_of_plugs,
            
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
pub struct PowerStrip {
    pub id: i64,
    pub make: String,
    pub model: String,
    pub color: String,
    pub num_of_plugs: i32,
    
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
#[diesel(table_name = crate::schema::powerstrips)]
struct NewPowerStripDbo {
    pub make: String,
    pub model: String,
    pub color: String,
    pub num_of_plugs: i32,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone
)]
pub struct NewPowerStrip {
    pub make: String,
    pub model: String,
    pub color: String,
    pub num_of_plugs: i32,

    pub equipmentsetid: i64,
    pub misc_note: Option<String>,
}
impl NewPowerStrip {
    fn to_dbo(&self) -> NewPowerStripDbo {
        NewPowerStripDbo {
            make: self.make.clone(),
            model: self.model.clone(),
            color: self.color.clone(),
            num_of_plugs: self.num_of_plugs,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::powerstrips)]
#[diesel(primary_key(id))]
struct PowerStripDboChangeset {
    pub make: Option<String>,
    pub model: Option<String>,
    pub color: Option<String>,
    pub num_of_plugs: Option<i32>,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone
)]
pub struct PowerStripChangeSet {
    pub make: Option<String>,
    pub model: Option<String>,
    pub color: Option<String>,
    pub num_of_plugs: Option<i32>,

    pub equipmentsetid: Option<i64>,
    pub misc_note: Option<String>,
}
impl PowerStripChangeSet {
    fn to_dbos(&self) -> (PowerStripDboChangeset, EquipmentDboChangeset) {
        let clone_of_self = self.clone();
        (
            PowerStripDboChangeset {
                make: self.make.clone(),
                model: self.model.clone(),
                color: self.color.clone(),
                num_of_plugs: self.num_of_plugs,
            },
            EquipmentDboChangeset {
                misc_note: clone_of_self.misc_note,
                equipmentsetid: clone_of_self.equipmentsetid
            }
        )
    }
}

fn covert_to_model_from_dbos(powerstrip_dbo: PowerStripDbo, equipment_dbo: EquipmentDbo) -> PowerStrip {
    PowerStrip {
        id: powerstrip_dbo.id,
        make: powerstrip_dbo.make,
        model: powerstrip_dbo.model,
        color: powerstrip_dbo.color,
        num_of_plugs: powerstrip_dbo.num_of_plugs,
        
        created_at: equipment_dbo.created_at,
        updated_at: equipment_dbo.updated_at,
        equipmentid: equipment_dbo.id,
        equipmentsetid: equipment_dbo.equipmentsetid,
        misc_note: equipment_dbo.misc_note,
    }
}

pub fn create(db: &mut database::Connection, item: &NewPowerStrip) -> QueryResult<PowerStrip> {
    use crate::schema::powerstrips::dsl::*;

    let misc_note = item.clone().misc_note;
    let equipmentsetid = item.equipmentsetid;
    let new_powerstrip_dbo = item.to_dbo();

    let powerstrip_dbo_result = 
        insert_into(powerstrips)
            .values(new_powerstrip_dbo)
            .get_result::<PowerStripDbo>(db);

    if powerstrip_dbo_result.is_err() {
        return Err(powerstrip_dbo_result.err().unwrap());
    }

    let powerstrip_dbo = powerstrip_dbo_result.unwrap();

    let equipment_dbo_result  = 
        EquipmentDboBuilder::new()
            .set_powerstripid(Some(powerstrip_dbo.id))
            .set_misc_note(misc_note)
            .set_equipmentsetid(Some(equipmentsetid))
            .build_and_insert(db);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo = equipment_dbo_result.unwrap();

    Ok(powerstrip_dbo.to_model(equipment_dbo))
}


pub fn exists(db: &mut database::Connection, powerstrip_id: i64) -> bool {
    use crate::schema::powerstrips::dsl::*;
    powerstrips
        .find(powerstrip_id)
        .get_result::<PowerStripDbo>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, equipment_dbo_id: i64) -> QueryResult<PowerStrip> {
    use crate::schema::powerstrips::dsl::*;

    let equipment_dbo_result = models::equipment_dbo::read(db, equipment_dbo_id);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo = equipment_dbo_result.unwrap();

    if equipment_dbo.powerstripid.is_none() {
        println!("EquipmentDbo of ID {} has no id. Could not retrieve powerstrip from DB.", equipment_dbo.id);
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: EquipmentDbo of ID {} has no id. Could not retrieve powerstrip from DB.", equipment_dbo.id).into()
        ));
    }

    let powerstrip_dbo_result = 
        powerstrips
            .filter(id.eq(equipment_dbo.powerstripid.unwrap()))
            .first::<PowerStripDbo>(db);
    
    if powerstrip_dbo_result.is_err() {
        return Err(powerstrip_dbo_result.err().unwrap());
    }
    
    Ok(powerstrip_dbo_result.unwrap().to_model(equipment_dbo))
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<PowerStrip>> {
    use crate::schema::powerstrips::dsl::*;
    use crate::schema::equipment::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let tuple_result: Result<Vec<(EquipmentDbo, PowerStripDbo)>, result::Error> =
        equipment
            .inner_join(powerstrips)
            .order(crate::schema::powerstrips::dsl::id)
            .limit(page_size)
            .offset(offset_val)
            .load::<(EquipmentDbo, PowerStripDbo)>(db);
    
    if tuple_result.is_err() {
        return Err(tuple_result.err().unwrap());
    }

    Ok(
        tuple_result
            .unwrap()
            .into_iter()
            .map(|c: (EquipmentDbo, PowerStripDbo)| c.1.to_model(c.0))
            .collect()
    )
}

pub fn update(db: &mut database::Connection, equipment_id: i64, item: &PowerStripChangeSet) -> QueryResult<PowerStrip> {
    use crate::schema::powerstrips::dsl::*;

    let equipment_dbo_result = models::equipment_dbo::read(db, equipment_id);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo: EquipmentDbo = equipment_dbo_result.unwrap();

    let (powerstrip_dbo_changeset, equipment_dbo_changeset) = item.to_dbos();

    if equipment_dbo.powerstripid.is_none() {
        println!("EquipmentDbo's id is none. PowerStrip could not be updated.");
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: EquipmentDbo's id is none. PowerStrip could not be updated.").into()
        ));
    }

    let powerstrip_dbo_result = 
        diesel::update(
            powerstrips
                .filter(id.eq(equipment_dbo.powerstripid.unwrap())))
                .set((
                    powerstrip_dbo_changeset,
                    updated_at.eq(diesel::dsl::now),
                )
        )
            .get_result::<PowerStripDbo>(db);

    if powerstrip_dbo_result.is_err() {
        return Err(powerstrip_dbo_result.err().unwrap());
    }

    let equipment_dbo_result = 
        models::equipment_dbo::update(db, equipment_dbo.id, &equipment_dbo_changeset);

    if equipment_dbo_result.is_err() {
        return Err(powerstrip_dbo_result.err().unwrap());
    }

    Ok(
        covert_to_model_from_dbos(
            powerstrip_dbo_result.unwrap(), 
            equipment_dbo_result.unwrap()
        )
    )
}

pub fn delete(db: &mut database::Connection, equipment_id: i64) -> QueryResult<(usize, usize)> {
    use crate::schema::powerstrips::dsl::*;

    let equipment_dbo_result = models::equipment_dbo::read(db, equipment_id);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo: EquipmentDbo = equipment_dbo_result.unwrap();

    if equipment_dbo.powerstripid.is_none() {
        println!("EquipmentDbo's id is none. PowerStrip could not be updated.");
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: EquipmentDbo's id is none. PowerStrip could not be updated.").into()
        ));
    }

    let equipment_dbo_delete_result = models::equipment_dbo::delete(db, equipment_id);

    if equipment_dbo_delete_result.is_err() {
        return Err(equipment_dbo_delete_result.err().unwrap());
    }

    let powerstrip_delete_result = 
        diesel::delete(powerstrips.filter(id.eq(equipment_dbo.powerstripid.unwrap()))).execute(db);

    if powerstrip_delete_result.is_err() {
        return Err(powerstrip_delete_result.err().unwrap());
    }

    Ok((powerstrip_delete_result.unwrap(), equipment_dbo_delete_result.unwrap()))
}
