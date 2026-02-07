
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
pub struct MonitorBuilder {
    size: Option<String>,
    brand: Option<String>,
    has_vga_out_port: Option<bool>,
    has_dvi_out_port: Option<bool>,
    has_hdmi_out_port: Option<bool>,
    has_display_port_out: Option<bool>,

    equipmentsetid: i64,
    misc_note: Option<String>,
}

impl MonitorBuilder {
    pub fn new(equipmentsetid: i64) -> Self {
        Self {
            size: None,
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
            size: None,
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
    pub fn set_size(mut self, size: Option<String>) -> Self {
        self.size = size;
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
    pub fn set_misc_note(mut self, misc_note: Option<String>) -> Self {
        self.misc_note = misc_note;
        self
    }
    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.size.is_none() {
            errors.push("size is required".to_string());
        }
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
    pub fn build(self) -> Result<NewMonitor, Vec<String>> {
        match self.validate() {
            Err(e) => {
                Err(e)
            },
            Ok(_) => {
                Ok(
                    NewMonitor {
                        size: self.size.unwrap(),
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
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<Monitor> {
        let new_monitor = self.build();
        create(db, &new_monitor.unwrap())
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
#[diesel(table_name = crate::schema::monitors)]
#[diesel(primary_key(id))]
struct MonitorDbo {
    pub id: i64,
    pub size: String,
    pub brand: String,
    pub has_vga_out_port: bool,
    pub has_dvi_out_port: bool,
    pub has_hdmi_out_port: bool,
    pub has_display_port_out: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl MonitorDbo {
    pub fn to_model(&self, equipment_dbo: EquipmentDbo) -> Monitor {
        Monitor {
            id: self.id,
            size: self.size.clone(),
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
pub struct Monitor {
    pub id: i64,
    pub size: String,
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
#[diesel(table_name = crate::schema::monitors)]
struct NewMonitorDbo {
    pub size: String,
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
pub struct NewMonitor {
    pub size: String,
    pub brand: String,
    pub has_vga_out_port: bool,
    pub has_dvi_out_port: bool,
    pub has_hdmi_out_port: bool,
    pub has_display_port_out: bool,

    pub equipmentsetid: i64,
    pub misc_note: Option<String>,
}
impl NewMonitor {
    fn to_dbo(&self) -> NewMonitorDbo {
        NewMonitorDbo {
            size: self.size.clone(),
            brand: self.brand.clone(),
            has_vga_out_port: self.has_vga_out_port,
            has_dvi_out_port: self.has_dvi_out_port,
            has_hdmi_out_port: self.has_hdmi_out_port,
            has_display_port_out: self.has_display_port_out,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::monitors)]
#[diesel(primary_key(id))]
struct MonitorDboChangeset {
    pub size: Option<String>,
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
pub struct MonitorChangeSet {
    pub size: Option<String>,
    pub brand: Option<String>,
    pub has_vga_out_port: Option<bool>,
    pub has_dvi_out_port: Option<bool>,
    pub has_hdmi_out_port: Option<bool>,
    pub has_display_port_out: Option<bool>,

    pub equipmentsetid: Option<i64>,
    pub misc_note: Option<String>,
}
impl MonitorChangeSet {
    fn to_dbos(&self) -> (MonitorDboChangeset, EquipmentDboChangeset) {
        let clone_of_self = self.clone();
        (
            MonitorDboChangeset {
                size: self.size.clone(),
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

fn covert_to_model_from_dbos(monitor_dbo: MonitorDbo, equipment_dbo: EquipmentDbo) -> Monitor {
    Monitor {
        id: monitor_dbo.id,
        size: monitor_dbo.size,
        brand: monitor_dbo.brand,
        has_vga_out_port: monitor_dbo.has_vga_out_port,
        has_dvi_out_port: monitor_dbo.has_dvi_out_port,
        has_hdmi_out_port: monitor_dbo.has_hdmi_out_port,
        has_display_port_out: monitor_dbo.has_display_port_out,

        equipmentid: equipment_dbo.id,
        equipmentsetid: equipment_dbo.equipmentsetid,
        misc_note: equipment_dbo.misc_note,
        created_at: equipment_dbo.created_at,
        updated_at: equipment_dbo.updated_at,
    }
}

pub fn create(db: &mut database::Connection, item: &NewMonitor) -> QueryResult<Monitor> {
    use crate::schema::monitors::dsl::*;

    let misc_note = item.clone().misc_note;
    let equipmentsetid = item.equipmentsetid;
    let new_monitor_dbo = item.to_dbo();

    let monitor_dbo_result = 
        insert_into(monitors)
            .values(new_monitor_dbo)
            .get_result::<MonitorDbo>(db);

    if monitor_dbo_result.is_err() {
        return Err(monitor_dbo_result.err().unwrap());
    }

    let monitor_dbo = monitor_dbo_result.unwrap();

    let equipment_dbo_result  = 
        EquipmentDboBuilder::new()
            .set_monitorid(Some(monitor_dbo.id))
            .set_misc_note(misc_note)
            .set_equipmentsetid(Some(equipmentsetid))
            .build_and_insert(db);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo = equipment_dbo_result.unwrap();

    Ok(monitor_dbo.to_model(equipment_dbo))
}


pub fn exists(db: &mut database::Connection, monitor_id: i64) -> bool {
    use crate::schema::monitors::dsl::*;
    monitors
        .find(monitor_id)
        .get_result::<MonitorDbo>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, monitor_id: i64) -> QueryResult<Monitor> {
    use crate::schema::monitors::dsl::*;
    use crate::schema::equipment::dsl::*;

    let equipment_dbo_result = 
        equipment
            .filter(crate::schema::equipment::dsl::monitorid.eq(monitor_id))
            .first::<EquipmentDbo>(db);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo = equipment_dbo_result.unwrap();

    if equipment_dbo.monitorid.is_none() {
        println!("EquipmentDbo of ID {} has no id. Could not retrieve monitor from DB.", equipment_dbo.id);
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: EquipmentDbo of ID {} has no id. Could not retrieve monitor from DB.", equipment_dbo.id).into()
        ));
    }

    let monitor_dbo_result = 
        monitors
            .filter(crate::schema::monitors::dsl::id.eq(equipment_dbo.monitorid.unwrap()))
            .first::<MonitorDbo>(db);
    
    if monitor_dbo_result.is_err() {
        return Err(monitor_dbo_result.err().unwrap());
    }
    
    Ok(monitor_dbo_result.unwrap().to_model(equipment_dbo))
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<Monitor>> {
    use crate::schema::monitors::dsl::*;
    use crate::schema::equipment::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let tuple_result: Result<Vec<(EquipmentDbo, MonitorDbo)>, result::Error> =
        equipment
            .inner_join(monitors)
            .order(crate::schema::monitors::dsl::id)
            .limit(page_size)
            .offset(offset_val)
            .load::<(EquipmentDbo, MonitorDbo)>(db);
    
    if tuple_result.is_err() {
        return Err(tuple_result.err().unwrap());
    }

    Ok(
        tuple_result
            .unwrap()
            .into_iter()
            .map(|c: (EquipmentDbo, MonitorDbo)| c.1.to_model(c.0))
            .collect()
    )
}

pub fn update(db: &mut database::Connection, equipment_id: i64, item: &MonitorChangeSet) -> QueryResult<Monitor> {
    use crate::schema::monitors::dsl::*;

    let equipment_dbo_result = models::equipment_dbo::read(db, equipment_id);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo: EquipmentDbo = equipment_dbo_result.unwrap();

    let (monitor_dbo_changeset, equipment_dbo_changeset) = item.to_dbos();

    if equipment_dbo.monitorid.is_none() {
        println!("EquipmentDbo's id is none. Monitor could not be updated.");
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: EquipmentDbo's id is none. Monitor could not be updated.").into()
        ));
    }

    let monitor_dbo_result = 
        diesel::update(
            monitors
                .filter(id.eq(equipment_dbo.monitorid.unwrap())))
                .set((
                    monitor_dbo_changeset,
                    updated_at.eq(diesel::dsl::now),
                )
        )
            .get_result::<MonitorDbo>(db);

    if monitor_dbo_result.is_err() {
        return Err(monitor_dbo_result.err().unwrap());
    }

    let equipment_dbo_result = 
        models::equipment_dbo::update(db, equipment_dbo.id, &equipment_dbo_changeset);

    if equipment_dbo_result.is_err() {
        return Err(monitor_dbo_result.err().unwrap());
    }

    Ok(
        covert_to_model_from_dbos(
            monitor_dbo_result.unwrap(), 
            equipment_dbo_result.unwrap()
        )
    )
}

pub fn delete(db: &mut database::Connection, equipment_id: i64) -> QueryResult<(usize, usize)> {
    use crate::schema::monitors::dsl::*;

    let equipment_dbo_result = models::equipment_dbo::read(db, equipment_id);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo: EquipmentDbo = equipment_dbo_result.unwrap();

    if equipment_dbo.monitorid.is_none() {
        println!("EquipmentDbo's id is none. Monitor could not be updated.");
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: EquipmentDbo's id is none. Monitor could not be updated.").into()
        ));
    }

    let equipment_dbo_delete_result = models::equipment_dbo::delete(db, equipment_id);

    if equipment_dbo_delete_result.is_err() {
        return Err(equipment_dbo_delete_result.err().unwrap());
    }

    let monitor_delete_result = 
        diesel::delete(monitors.filter(id.eq(equipment_dbo.monitorid.unwrap()))).execute(db);

    if monitor_delete_result.is_err() {
        return Err(monitor_delete_result.err().unwrap());
    }

    Ok((monitor_delete_result.unwrap(), equipment_dbo_delete_result.unwrap()))
}
