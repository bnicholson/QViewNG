
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
pub struct ProjectorBuilder {
    brand: Option<String>,
    has_vga_out_port: Option<bool>,
    has_dvi_out_port: Option<bool>,
    has_hdmi_out_port: Option<bool>,
    has_display_port_out: Option<bool>,

    equipmentsetid: i64,
    misc_note: Option<String>,
}

impl ProjectorBuilder {
    pub fn new(equipmentsetid: i64) -> Self {
        Self {
            brand: None,
            has_vga_out_port: None,
            has_dvi_out_port: None,
            has_hdmi_out_port: None,
            has_display_port_out: None,

            equipmentsetid,
            misc_note: None,
        }
    }
    pub fn new_default(equipmentsetid: i64) -> Self {
        Self {
            brand: None,
            has_vga_out_port: Some(false),
            has_dvi_out_port: Some(false),
            has_hdmi_out_port: Some(false),
            has_display_port_out: Some(false),

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
    pub fn set_brand(mut self, brand: Option<String>) -> Self {
        self.brand = brand;
        self
    }
    pub fn set_has_vga_out_port(mut self, has_vga_out_port: Option<bool>) -> Self {
        self.has_vga_out_port = has_vga_out_port;
        self
    }
    pub fn set_has_dvi_out_port(mut self, has_dvi_out_port: Option<bool>) -> Self {
        self.has_dvi_out_port = has_dvi_out_port;
        self
    }
    pub fn set_has_hdmi_out_port(mut self, has_hdmi_out_port: Option<bool>) -> Self {
        self.has_hdmi_out_port = has_hdmi_out_port;
        self
    }
    pub fn set_has_display_port_out(mut self, has_display_port_out: Option<bool>) -> Self {
        self.has_display_port_out = has_display_port_out;
        self
    }
    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.brand.is_none() {
            errors.push("brand is required".to_string());
        }
        if self.has_vga_out_port.is_none() {
            errors.push("has_vga_out_port is required".to_string());
        }
        if self.has_dvi_out_port.is_none() {
            errors.push("has_dvi_out_port is required".to_string());
        }
        if self.has_hdmi_out_port.is_none() {
            errors.push("has_hdmi_out_port is required".to_string());
        }
        if self.has_display_port_out.is_none() {
            errors.push("has_display_port_out is required".to_string());
        }

        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
    pub fn build(self) -> Result<NewProjector, Vec<String>> {
        match self.validate() {
            Err(e) => {
                Err(e)
            },
            Ok(_) => {
                Ok(
                    NewProjector {
                        brand: self.brand.unwrap(),
                        has_vga_out_port: self.has_vga_out_port.unwrap(),
                        has_dvi_out_port: self.has_dvi_out_port.unwrap(),
                        has_hdmi_out_port: self.has_hdmi_out_port.unwrap(),
                        has_display_port_out: self.has_display_port_out.unwrap(),
                        
                        equipmentsetid: self.equipmentsetid,
                        misc_note: self.misc_note,
                    }
                )
            }
        }
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<Projector> {
        let new_projector = self.build();
        create(db, &new_projector.unwrap())
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
#[diesel(table_name = crate::schema::projectors)]
#[diesel(primary_key(id))]
struct ProjectorDbo {
    pub id: i64,
    pub brand: String,
    pub has_vga_out_port: bool,
    pub has_dvi_out_port: bool,
    pub has_hdmi_out_port: bool,
    pub has_display_port_out: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl ProjectorDbo {
    pub fn to_model(&self, equipment_dbo: EquipmentDbo) -> Projector {
        Projector {
            id: self.id,
            brand: self.brand.clone(),
            has_vga_out_port: self.has_vga_out_port,
            has_dvi_out_port: self.has_dvi_out_port,
            has_hdmi_out_port: self.has_hdmi_out_port,
            has_display_port_out: self.has_display_port_out,
            
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
pub struct Projector {
    pub id: i64,
    pub brand: String,
    pub has_vga_out_port: bool,
    pub has_dvi_out_port: bool,
    pub has_hdmi_out_port: bool,
    pub has_display_port_out: bool,
    
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
#[diesel(table_name = crate::schema::projectors)]
struct NewProjectorDbo {
    pub brand: String,
    pub has_vga_out_port: bool,
    pub has_dvi_out_port: bool,
    pub has_hdmi_out_port: bool,
    pub has_display_port_out: bool,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone
)]
pub struct NewProjector {
    pub brand: String,
    pub has_vga_out_port: bool,
    pub has_dvi_out_port: bool,
    pub has_hdmi_out_port: bool,
    pub has_display_port_out: bool,

    pub equipmentsetid: i64,
    pub misc_note: Option<String>,
}
impl NewProjector {
    fn to_dbo(&self) -> NewProjectorDbo {
        NewProjectorDbo {
            brand: self.brand.clone(),
            has_vga_out_port: self.has_vga_out_port,
            has_dvi_out_port: self.has_dvi_out_port,
            has_hdmi_out_port: self.has_hdmi_out_port,
            has_display_port_out: self.has_display_port_out,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::projectors)]
#[diesel(primary_key(id))]
struct ProjectorDboChangeset {
    pub brand: Option<String>,
    pub has_vga_out_port: Option<bool>,
    pub has_dvi_out_port: Option<bool>,
    pub has_hdmi_out_port: Option<bool>,
    pub has_display_port_out: Option<bool>,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone
)]
pub struct ProjectorChangeSet {
    pub brand: Option<String>,
    pub has_vga_out_port: Option<bool>,
    pub has_dvi_out_port: Option<bool>,
    pub has_hdmi_out_port: Option<bool>,
    pub has_display_port_out: Option<bool>,

    pub equipmentsetid: Option<i64>,
    pub misc_note: Option<String>,
}
impl ProjectorChangeSet {
    fn to_dbos(&self) -> (ProjectorDboChangeset, EquipmentDboChangeset) {
        let clone_of_self = self.clone();
        (
            ProjectorDboChangeset {
                brand: self.brand.clone(),
                has_vga_out_port: self.has_vga_out_port,
                has_dvi_out_port: self.has_dvi_out_port,
                has_hdmi_out_port: self.has_hdmi_out_port,
                has_display_port_out: self.has_display_port_out,
            },
            EquipmentDboChangeset {
                misc_note: clone_of_self.misc_note,
                equipmentsetid: clone_of_self.equipmentsetid
            }
        )
    }
}

fn covert_to_model_from_dbos(projector_dbo: ProjectorDbo, equipment_dbo: EquipmentDbo) -> Projector {
    Projector {
        id: projector_dbo.id,
        brand: projector_dbo.brand.clone(),
        has_vga_out_port: projector_dbo.has_vga_out_port,
        has_dvi_out_port: projector_dbo.has_dvi_out_port,
        has_hdmi_out_port: projector_dbo.has_hdmi_out_port,
        has_display_port_out: projector_dbo.has_display_port_out,
        
        created_at: equipment_dbo.created_at,
        updated_at: equipment_dbo.updated_at,
        equipmentid: equipment_dbo.id,
        equipmentsetid: equipment_dbo.equipmentsetid,
        misc_note: equipment_dbo.misc_note,
    }
}

pub fn create(db: &mut database::Connection, item: &NewProjector) -> QueryResult<Projector> {
    use crate::schema::projectors::dsl::*;

    let misc_note = item.clone().misc_note;
    let equipmentsetid = item.equipmentsetid;
    let new_projector_dbo = item.to_dbo();

    let projector_dbo_result = 
        insert_into(projectors)
            .values(new_projector_dbo)
            .get_result::<ProjectorDbo>(db);

    if projector_dbo_result.is_err() {
        return Err(projector_dbo_result.err().unwrap());
    }

    let projector_dbo = projector_dbo_result.unwrap();

    let equipment_dbo_result  = 
        EquipmentDboBuilder::new()
            .set_projectorid(Some(projector_dbo.id))
            .set_misc_note(misc_note)
            .set_equipmentsetid(Some(equipmentsetid))
            .build_and_insert(db);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo = equipment_dbo_result.unwrap();

    Ok(projector_dbo.to_model(equipment_dbo))
}


pub fn exists(db: &mut database::Connection, projector_id: i64) -> bool {
    use crate::schema::projectors::dsl::*;
    projectors
        .find(projector_id)
        .get_result::<ProjectorDbo>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, equipment_dbo_id: i64) -> QueryResult<Projector> {
    use crate::schema::projectors::dsl::*;

    let equipment_dbo_result = models::equipment_dbo::read(db, equipment_dbo_id);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo = equipment_dbo_result.unwrap();

    if equipment_dbo.projectorid.is_none() {
        println!("EquipmentDbo of ID {} has no id. Could not retrieve projector from DB.", equipment_dbo.id);
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: EquipmentDbo of ID {} has no id. Could not retrieve projector from DB.", equipment_dbo.id).into()
        ));
    }

    let projector_dbo_result = 
        projectors
            .filter(id.eq(equipment_dbo.projectorid.unwrap()))
            .first::<ProjectorDbo>(db);
    
    if projector_dbo_result.is_err() {
        return Err(projector_dbo_result.err().unwrap());
    }
    
    Ok(projector_dbo_result.unwrap().to_model(equipment_dbo))
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<Projector>> {
    use crate::schema::projectors::dsl::*;
    use crate::schema::equipment::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let tuple_result: Result<Vec<(EquipmentDbo, ProjectorDbo)>, result::Error> =
        equipment
            .inner_join(projectors)
            .order(crate::schema::projectors::dsl::id)
            .limit(page_size)
            .offset(offset_val)
            .load::<(EquipmentDbo, ProjectorDbo)>(db);
    
    if tuple_result.is_err() {
        return Err(tuple_result.err().unwrap());
    }

    Ok(
        tuple_result
            .unwrap()
            .into_iter()
            .map(|c: (EquipmentDbo, ProjectorDbo)| c.1.to_model(c.0))
            .collect()
    )
}

pub fn update(db: &mut database::Connection, equipment_id: i64, item: &ProjectorChangeSet) -> QueryResult<Projector> {
    use crate::schema::projectors::dsl::*;

    let equipment_dbo_result = models::equipment_dbo::read(db, equipment_id);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo: EquipmentDbo = equipment_dbo_result.unwrap();

    let (projector_dbo_changeset, equipment_dbo_changeset) = item.to_dbos();

    if equipment_dbo.projectorid.is_none() {
        println!("EquipmentDbo's id is none. Projector could not be updated.");
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: EquipmentDbo's id is none. Projector could not be updated.").into()
        ));
    }

    let projector_dbo_result = 
        diesel::update(
            projectors
                .filter(id.eq(equipment_dbo.projectorid.unwrap())))
                .set((
                    projector_dbo_changeset,
                    updated_at.eq(diesel::dsl::now),
                )
        )
            .get_result::<ProjectorDbo>(db);

    if projector_dbo_result.is_err() {
        return Err(projector_dbo_result.err().unwrap());
    }

    let equipment_dbo_result = 
        models::equipment_dbo::update(db, equipment_dbo.id, &equipment_dbo_changeset);

    if equipment_dbo_result.is_err() {
        return Err(projector_dbo_result.err().unwrap());
    }

    Ok(
        covert_to_model_from_dbos(
            projector_dbo_result.unwrap(), 
            equipment_dbo_result.unwrap()
        )
    )
}

pub fn delete(db: &mut database::Connection, equipment_id: i64) -> QueryResult<(usize, usize)> {
    use crate::schema::projectors::dsl::*;

    let equipment_dbo_result = models::equipment_dbo::read(db, equipment_id);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo: EquipmentDbo = equipment_dbo_result.unwrap();

    if equipment_dbo.projectorid.is_none() {
        println!("EquipmentDbo's id is none. Projector could not be updated.");
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: EquipmentDbo's id is none. Projector could not be updated.").into()
        ));
    }

    let equipment_dbo_delete_result = models::equipment_dbo::delete(db, equipment_id);

    if equipment_dbo_delete_result.is_err() {
        return Err(equipment_dbo_delete_result.err().unwrap());
    }

    let projector_delete_result = 
        diesel::delete(projectors.filter(id.eq(equipment_dbo.projectorid.unwrap()))).execute(db);

    if projector_delete_result.is_err() {
        return Err(projector_delete_result.err().unwrap());
    }

    Ok((projector_delete_result.unwrap(), equipment_dbo_delete_result.unwrap()))
}
