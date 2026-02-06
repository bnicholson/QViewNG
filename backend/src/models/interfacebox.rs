
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
pub struct InterfaceBoxBuilder {
    type_: Option<String>,
    serial_number: Option<String>,
    equipmentsetid: i64,
    misc_note: Option<String>,
}

impl InterfaceBoxBuilder {
    pub fn new(equipmentsetid: i64) -> Self {
        Self {
            type_: None,
            serial_number: None,
            equipmentsetid,
            misc_note: None,
        }
    }
    pub fn new_default(equipmentsetid: i64) -> Self {
        Self {
            type_: None,
            serial_number: Some("hajujs8i699li8".to_string()),
            equipmentsetid,
            misc_note: None,
        }
    }
    pub fn set_type_(mut self, type_: Option<String>) -> Self {
        self.type_ = type_;
        self
    }
    pub fn set_serial_number(mut self, serial_number: Option<String>) -> Self {
        self.serial_number = serial_number;
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

        if self.type_.is_none() {
            errors.push("type_ is required".to_string());
        }
        if self.serial_number.is_none() {
            errors.push("serial_number is required".to_string());
        }

        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
    pub fn build(self) -> Result<NewInterfaceBox, Vec<String>> {
        match self.validate() {
            Err(e) => {
                Err(e)
            },
            Ok(_) => {
                Ok(
                    NewInterfaceBox {
                        type_: self.type_.unwrap(),
                        serial_number: self.serial_number,
                        
                        equipmentsetid: self.equipmentsetid,
                        misc_note: self.misc_note,
                    }
                )
            }
        }
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<InterfaceBox> {
        let new_interfacebox = self.build();
        create(db, &new_interfacebox.unwrap())
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
#[diesel(table_name = crate::schema::interfaceboxes)]
#[diesel(primary_key(id))]
struct InterfaceBoxDbo {
    pub id: i64,
    pub type_: String,
    pub serial_number: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl InterfaceBoxDbo {
    pub fn to_model(&self, equipment_dbo: EquipmentDbo) -> InterfaceBox {
        InterfaceBox {
            id: self.id,
            type_: self.type_.clone(),
            serial_number: self.serial_number.clone(),
            
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
pub struct InterfaceBox {
    pub id: i64,
    pub type_: String,
    pub serial_number: Option<String>,
    
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
#[diesel(table_name = crate::schema::interfaceboxes)]
struct NewInterfaceBoxDbo {
    pub type_: String,
    pub serial_number: Option<String>,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone
)]
pub struct NewInterfaceBox {
    pub type_: String,
    pub serial_number: Option<String>,

    pub equipmentsetid: i64,
    pub misc_note: Option<String>,
}
impl NewInterfaceBox {
    fn to_dbo(&self) -> NewInterfaceBoxDbo {
        NewInterfaceBoxDbo {
            type_: self.type_.clone(),
            serial_number: self.serial_number.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::interfaceboxes)]
#[diesel(primary_key(id))]
struct InterfaceBoxDboChangeset {
    pub type_: String,
    pub serial_number: Option<String>,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone
)]
pub struct InterfaceBoxChangeSet {
    pub type_: String,
    pub serial_number: Option<String>,

    pub equipmentsetid: Option<i64>,
    pub misc_note: Option<String>,
}
impl InterfaceBoxChangeSet {
    fn to_dbos(&self) -> (InterfaceBoxDboChangeset, EquipmentDboChangeset) {
        let clone_of_self = self.clone();
        (
            InterfaceBoxDboChangeset {
                type_: self.type_.clone(),
                serial_number: self.serial_number.clone()
            },
            EquipmentDboChangeset {
                misc_note: clone_of_self.misc_note,
                equipmentsetid: clone_of_self.equipmentsetid
            }
        )
    }
}

fn covert_to_model_from_dbos(interfacebox_dbo: InterfaceBoxDbo, equipment_dbo: EquipmentDbo) -> InterfaceBox {
    InterfaceBox {
        id: interfacebox_dbo.id,
        type_: interfacebox_dbo.type_,
        serial_number: interfacebox_dbo.serial_number,

        equipmentid: equipment_dbo.id,
        equipmentsetid: equipment_dbo.equipmentsetid,
        misc_note: equipment_dbo.misc_note,
        created_at: equipment_dbo.created_at,
        updated_at: equipment_dbo.updated_at,
    }
}

pub fn create(db: &mut database::Connection, item: &NewInterfaceBox) -> QueryResult<InterfaceBox> {
    use crate::schema::interfaceboxes::dsl::*;

    let misc_note = item.clone().misc_note;
    let equipmentsetid = item.equipmentsetid;
    let new_interfacebox_dbo = item.to_dbo();

    let interfacebox_dbo_result = 
        insert_into(interfaceboxes)
            .values(new_interfacebox_dbo)
            .get_result::<InterfaceBoxDbo>(db);

    if interfacebox_dbo_result.is_err() {
        return Err(interfacebox_dbo_result.err().unwrap());
    }

    let interfacebox_dbo = interfacebox_dbo_result.unwrap();

    let equipment_dbo_result  = 
        EquipmentDboBuilder::new()
            .set_interfaceboxid(Some(interfacebox_dbo.id))
            .set_misc_note(misc_note)
            .set_equipmentsetid(Some(equipmentsetid))
            .build_and_insert(db);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo = equipment_dbo_result.unwrap();

    Ok(interfacebox_dbo.to_model(equipment_dbo))
}


pub fn exists(db: &mut database::Connection, interfacebox_id: i64) -> bool {
    use crate::schema::interfaceboxes::dsl::*;
    interfaceboxes
        .find(interfacebox_id)
        .get_result::<InterfaceBoxDbo>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, equipment_dbo_id: i64) -> QueryResult<InterfaceBox> {
    use crate::schema::interfaceboxes::dsl::*;

    let equipment_dbo_result = models::equipment_dbo::read(db, equipment_dbo_id);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo = equipment_dbo_result.unwrap();

    if equipment_dbo.interfaceboxid.is_none() {
        println!("EquipmentDbo of ID {} has no id. Could not retrieve interfacebox from DB.", equipment_dbo.id);
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: EquipmentDbo of ID {} has no id. Could not retrieve interfacebox from DB.", equipment_dbo.id).into()
        ));
    }

    let interfacebox_dbo_result = 
        interfaceboxes
            .filter(id.eq(equipment_dbo.interfaceboxid.unwrap()))
            .first::<InterfaceBoxDbo>(db);
    
    if interfacebox_dbo_result.is_err() {
        return Err(interfacebox_dbo_result.err().unwrap());
    }
    
    Ok(interfacebox_dbo_result.unwrap().to_model(equipment_dbo))
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<InterfaceBox>> {
    use crate::schema::interfaceboxes::dsl::*;
    use crate::schema::equipment::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let tuple_result: Result<Vec<(EquipmentDbo, InterfaceBoxDbo)>, result::Error> =
        equipment
            .inner_join(interfaceboxes)
            .order(crate::schema::interfaceboxes::dsl::id)
            .limit(page_size)
            .offset(offset_val)
            .load::<(EquipmentDbo, InterfaceBoxDbo)>(db);
    
    if tuple_result.is_err() {
        return Err(tuple_result.err().unwrap());
    }

    Ok(
        tuple_result
            .unwrap()
            .into_iter()
            .map(|c: (EquipmentDbo, InterfaceBoxDbo)| c.1.to_model(c.0))
            .collect()
    )
}

pub fn update(db: &mut database::Connection, equipment_id: i64, item: &InterfaceBoxChangeSet) -> QueryResult<InterfaceBox> {
    use crate::schema::interfaceboxes::dsl::*;

    let equipment_dbo_result = models::equipment_dbo::read(db, equipment_id);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo: EquipmentDbo = equipment_dbo_result.unwrap();

    let (interfacebox_dbo_changeset, equipment_dbo_changeset) = item.to_dbos();

    if equipment_dbo.interfaceboxid.is_none() {
        println!("EquipmentDbo's id is none. InterfaceBox could not be updated.");
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: EquipmentDbo's id is none. InterfaceBox could not be updated.").into()
        ));
    }

    let interfacebox_dbo_result = 
        diesel::update(
            interfaceboxes
                .filter(id.eq(equipment_dbo.interfaceboxid.unwrap())))
                .set((
                    interfacebox_dbo_changeset,
                    updated_at.eq(diesel::dsl::now),
                )
        )
            .get_result::<InterfaceBoxDbo>(db);

    if interfacebox_dbo_result.is_err() {
        return Err(interfacebox_dbo_result.err().unwrap());
    }

    let equipment_dbo_result = 
        models::equipment_dbo::update(db, equipment_dbo.id, &equipment_dbo_changeset);

    if equipment_dbo_result.is_err() {
        return Err(interfacebox_dbo_result.err().unwrap());
    }

    Ok(
        covert_to_model_from_dbos(
            interfacebox_dbo_result.unwrap(), 
            equipment_dbo_result.unwrap()
        )
    )
}

pub fn delete(db: &mut database::Connection, equipment_id: i64) -> QueryResult<(usize, usize)> {
    use crate::schema::interfaceboxes::dsl::*;

    let equipment_dbo_result = models::equipment_dbo::read(db, equipment_id);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo: EquipmentDbo = equipment_dbo_result.unwrap();

    if equipment_dbo.interfaceboxid.is_none() {
        println!("EquipmentDbo's id is none. InterfaceBox could not be updated.");
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: EquipmentDbo's id is none. InterfaceBox could not be updated.").into()
        ));
    }

    let equipment_dbo_delete_result = models::equipment_dbo::delete(db, equipment_id);

    if equipment_dbo_delete_result.is_err() {
        return Err(equipment_dbo_delete_result.err().unwrap());
    }

    let interfacebox_delete_result = 
        diesel::delete(interfaceboxes.filter(id.eq(equipment_dbo.interfaceboxid.unwrap()))).execute(db);

    if interfacebox_delete_result.is_err() {
        return Err(interfacebox_delete_result.err().unwrap());
    }

    Ok((interfacebox_delete_result.unwrap(), equipment_dbo_delete_result.unwrap()))
}
