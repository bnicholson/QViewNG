
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
pub struct ExtensionCordBuilder {
    color: Option<String>,
    length: Option<String>,

    equipmentsetid: i64,
    misc_note: Option<String>,
}

impl ExtensionCordBuilder {
    pub fn new(equipmentsetid: i64) -> Self {
        Self {
            color: None,
            length: None,

            equipmentsetid,
            misc_note: None,
        }
    }
    pub fn new_default(equipmentsetid: i64) -> Self {
        Self {
            color: Some("black".to_string()),
            length: None,

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
    pub fn set_color(mut self, color: Option<String>) -> Self {
        self.color = color;
        self
    }
    pub fn set_length(mut self, length: Option<String>) -> Self {
        self.length = length;
        self
    }
    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.color.is_none() {
            errors.push("color is required".to_string());
        }
        if self.length.is_none() {
            errors.push("length is required".to_string());
        }

        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
    pub fn build(self) -> Result<NewExtensionCord, Vec<String>> {
        match self.validate() {
            Err(e) => {
                Err(e)
            },
            Ok(_) => {
                Ok(
                    NewExtensionCord {
                        color: self.color.unwrap(),
                        length: self.length.unwrap(),
                        
                        equipmentsetid: self.equipmentsetid,
                        misc_note: self.misc_note,
                    }
                )
            }
        }
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<ExtensionCord> {
        let new_extensioncord = self.build();
        create(db, &new_extensioncord.unwrap())
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
#[diesel(table_name = crate::schema::extensioncords)]
#[diesel(primary_key(id))]
struct ExtensionCordDbo {
    pub id: i64,
    pub color: String,
    pub length: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl ExtensionCordDbo {
    pub fn to_model(&self, equipment_dbo: EquipmentDbo) -> ExtensionCord {
        ExtensionCord {
            id: self.id,
            color: self.color.clone(),
            length: self.length.clone(),
            
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
pub struct ExtensionCord {
    pub id: i64,
    pub color: String,
    pub length: String,
    
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
#[diesel(table_name = crate::schema::extensioncords)]
struct NewExtensionCordDbo {
    pub color: String,
    pub length: String,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone
)]
pub struct NewExtensionCord {
    pub color: String,
    pub length: String,

    pub equipmentsetid: i64,
    pub misc_note: Option<String>,
}
impl NewExtensionCord {
    fn to_dbo(&self) -> NewExtensionCordDbo {
        NewExtensionCordDbo {
            color: self.color.clone(),
            length: self.length.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::extensioncords)]
#[diesel(primary_key(id))]
struct ExtensionCordDboChangeset {
    pub color: Option<String>,
    pub length: Option<String>,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone
)]
pub struct ExtensionCordChangeSet {
    pub color: Option<String>,
    pub length: Option<String>,

    pub equipmentsetid: Option<i64>,
    pub misc_note: Option<String>,
}
impl ExtensionCordChangeSet {
    fn to_dbos(&self) -> (ExtensionCordDboChangeset, EquipmentDboChangeset) {
        let clone_of_self = self.clone();
        (
            ExtensionCordDboChangeset {
                color: self.color.clone(),
                length: self.length.clone(),
            },
            EquipmentDboChangeset {
                misc_note: clone_of_self.misc_note,
                equipmentsetid: clone_of_self.equipmentsetid
            }
        )
    }
}

fn covert_to_model_from_dbos(extensioncord_dbo: ExtensionCordDbo, equipment_dbo: EquipmentDbo) -> ExtensionCord {
    ExtensionCord {
        id: extensioncord_dbo.id,
        color: extensioncord_dbo.color,
        length: extensioncord_dbo.length,
        
        created_at: equipment_dbo.created_at,
        updated_at: equipment_dbo.updated_at,
        equipmentid: equipment_dbo.id,
        equipmentsetid: equipment_dbo.equipmentsetid,
        misc_note: equipment_dbo.misc_note,
    }
}

pub fn create(db: &mut database::Connection, item: &NewExtensionCord) -> QueryResult<ExtensionCord> {
    use crate::schema::extensioncords::dsl::*;

    let misc_note = item.clone().misc_note;
    let equipmentsetid = item.equipmentsetid;
    let new_extensioncord_dbo = item.to_dbo();

    let extensioncord_dbo_result = 
        insert_into(extensioncords)
            .values(new_extensioncord_dbo)
            .get_result::<ExtensionCordDbo>(db);

    if extensioncord_dbo_result.is_err() {
        return Err(extensioncord_dbo_result.err().unwrap());
    }

    let extensioncord_dbo = extensioncord_dbo_result.unwrap();

    let equipment_dbo_result  = 
        EquipmentDboBuilder::new()
            .set_extensioncordid(Some(extensioncord_dbo.id))
            .set_misc_note(misc_note)
            .set_equipmentsetid(Some(equipmentsetid))
            .build_and_insert(db);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo = equipment_dbo_result.unwrap();

    Ok(extensioncord_dbo.to_model(equipment_dbo))
}


pub fn exists(db: &mut database::Connection, extensioncord_id: i64) -> bool {
    use crate::schema::extensioncords::dsl::*;
    extensioncords
        .find(extensioncord_id)
        .get_result::<ExtensionCordDbo>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, extensioncord_id: i64) -> QueryResult<ExtensionCord> {
    use crate::schema::extensioncords::dsl::*;
    use crate::schema::equipment::dsl::*;

    let equipment_dbo_result = 
        equipment
            .filter(crate::schema::equipment::dsl::extensioncordid.eq(extensioncord_id))
            .first::<EquipmentDbo>(db);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo = equipment_dbo_result.unwrap();

    if equipment_dbo.extensioncordid.is_none() {
        println!("EquipmentDbo of ID {} has no id. Could not retrieve extensioncord from DB.", equipment_dbo.id);
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: EquipmentDbo of ID {} has no id. Could not retrieve extensioncord from DB.", equipment_dbo.id).into()
        ));
    }

    let extensioncord_dbo_result = 
        extensioncords
            .filter(crate::schema::extensioncords::dsl::id.eq(equipment_dbo.extensioncordid.unwrap()))
            .first::<ExtensionCordDbo>(db);
    
    if extensioncord_dbo_result.is_err() {
        return Err(extensioncord_dbo_result.err().unwrap());
    }
    
    Ok(extensioncord_dbo_result.unwrap().to_model(equipment_dbo))
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<ExtensionCord>> {
    use crate::schema::extensioncords::dsl::*;
    use crate::schema::equipment::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let tuple_result: Result<Vec<(EquipmentDbo, ExtensionCordDbo)>, result::Error> =
        equipment
            .inner_join(extensioncords)
            .order(crate::schema::extensioncords::dsl::id)
            .limit(page_size)
            .offset(offset_val)
            .load::<(EquipmentDbo, ExtensionCordDbo)>(db);
    
    if tuple_result.is_err() {
        return Err(tuple_result.err().unwrap());
    }

    Ok(
        tuple_result
            .unwrap()
            .into_iter()
            .map(|c: (EquipmentDbo, ExtensionCordDbo)| c.1.to_model(c.0))
            .collect()
    )
}

pub fn update(db: &mut database::Connection, equipment_id: i64, item: &ExtensionCordChangeSet) -> QueryResult<ExtensionCord> {
    use crate::schema::extensioncords::dsl::*;

    let equipment_dbo_result = models::equipment_dbo::read(db, equipment_id);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo: EquipmentDbo = equipment_dbo_result.unwrap();

    let (extensioncord_dbo_changeset, equipment_dbo_changeset) = item.to_dbos();

    if equipment_dbo.extensioncordid.is_none() {
        println!("EquipmentDbo's id is none. ExtensionCord could not be updated.");
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: EquipmentDbo's id is none. ExtensionCord could not be updated.").into()
        ));
    }

    let extensioncord_dbo_result = 
        diesel::update(
            extensioncords
                .filter(id.eq(equipment_dbo.extensioncordid.unwrap())))
                .set((
                    extensioncord_dbo_changeset,
                    updated_at.eq(diesel::dsl::now),
                )
        )
            .get_result::<ExtensionCordDbo>(db);

    if extensioncord_dbo_result.is_err() {
        return Err(extensioncord_dbo_result.err().unwrap());
    }

    let equipment_dbo_result = 
        models::equipment_dbo::update(db, equipment_dbo.id, &equipment_dbo_changeset);

    if equipment_dbo_result.is_err() {
        return Err(extensioncord_dbo_result.err().unwrap());
    }

    Ok(
        covert_to_model_from_dbos(
            extensioncord_dbo_result.unwrap(), 
            equipment_dbo_result.unwrap()
        )
    )
}

pub fn delete(db: &mut database::Connection, equipment_id: i64) -> QueryResult<(usize, usize)> {
    use crate::schema::extensioncords::dsl::*;

    let equipment_dbo_result = models::equipment_dbo::read(db, equipment_id);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo: EquipmentDbo = equipment_dbo_result.unwrap();

    if equipment_dbo.extensioncordid.is_none() {
        println!("EquipmentDbo's id is none. ExtensionCord could not be updated.");
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: EquipmentDbo's id is none. ExtensionCord could not be updated.").into()
        ));
    }

    let equipment_dbo_delete_result = models::equipment_dbo::delete(db, equipment_id);

    if equipment_dbo_delete_result.is_err() {
        return Err(equipment_dbo_delete_result.err().unwrap());
    }

    let extensioncord_delete_result = 
        diesel::delete(extensioncords.filter(id.eq(equipment_dbo.extensioncordid.unwrap()))).execute(db);

    if extensioncord_delete_result.is_err() {
        return Err(extensioncord_delete_result.err().unwrap());
    }

    Ok((extensioncord_delete_result.unwrap(), equipment_dbo_delete_result.unwrap()))
}
