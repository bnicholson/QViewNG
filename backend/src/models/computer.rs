
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
pub struct ComputerBuilder {
    equipmentsetid: i64,
    brand: Option<String>,
    operating_system: Option<String>,
    quizmachine_version: Option<String>,
    wifi_capabilities: Option<String>,
    login_username: Option<String>,
    login_password: Option<String>,
    has_vga_out_port: Option<bool>,
    has_dvi_out_port: Option<bool>,
    has_hdmi_out_port: Option<bool>,
    has_display_port_out: Option<bool>,
    has_usb_port: Option<bool>,
    misc_note: Option<String>,
}

impl ComputerBuilder {
    pub fn new(equipmentsetid: i64) -> Self {
        Self {
            equipmentsetid,
            brand: None,
            operating_system: None,
            quizmachine_version: None,
            wifi_capabilities: None,
            login_username: None,
            login_password: None,
            has_vga_out_port: None,
            has_dvi_out_port: None,
            has_hdmi_out_port: None,
            has_display_port_out: None,
            has_usb_port: None,
            misc_note: None,
        }
    }
    pub fn new_default(equipmentsetid: i64) -> Self {
        Self {
            equipmentsetid,
            brand: Some(String::new()),
            operating_system: Some(String::new()),
            quizmachine_version: Some(String::new()),
            wifi_capabilities: Some(String::new()),
            login_username: Some(String::new()),
            login_password: Some(String::new()),
            has_vga_out_port: Some(false),
            has_dvi_out_port: Some(false),
            has_hdmi_out_port: Some(false),
            has_display_port_out: Some(false),
            has_usb_port: Some(false),
            misc_note: None,
        }
    }
    pub fn set_equipmentsetid(mut self, equipmentsetid: i64) -> Self {
        self.equipmentsetid = equipmentsetid;
        self
    }
    pub fn set_brand(mut self, brand: Option<String>) -> Self {
        self.brand = brand;
        self
    }
    pub fn set_operating_system(mut self, operating_system: Option<String>) -> Self {
        self.operating_system = operating_system;
        self
    }
    pub fn set_quizmachine_version(mut self, quizmachine_version: Option<String>) -> Self {
        self.quizmachine_version = quizmachine_version;
        self
    }
    pub fn set_wifi_capabilities(mut self, wifi_capabilities: Option<String>) -> Self {
        self.wifi_capabilities = wifi_capabilities;
        self
    }
    pub fn set_login_username(mut self, login_username: Option<String>) -> Self {
        self.login_username = login_username;
        self
    }
    pub fn set_login_password(mut self, login_password: Option<String>) -> Self {
        self.login_password = login_password;
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
    pub fn set_has_usb_port(mut self, has_usb_port: Option<bool>) -> Self {
        self.has_usb_port = has_usb_port;
        self
    }
    pub fn set_misc_note(mut self, misc_note: Option<String>) -> Self {
        self.misc_note = misc_note;
        self
    }
    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.brand.is_none() {
            errors.push("brand is required".to_string());
        }
        if self.operating_system.is_none() {
            errors.push("operating_system is required".to_string());
        }
        if self.quizmachine_version.is_none() {
            errors.push("quizmachine_version is required".to_string());
        }
        if self.wifi_capabilities.is_none() {
            errors.push("wifi_capabilities is required".to_string());
        }
        if self.login_username.is_none() {
            errors.push("login_username is required".to_string());
        }
        if self.login_password.is_none() {
            errors.push("login_password is required".to_string());
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
        if self.has_usb_port.is_none() {
            errors.push("has_usb_port is required".to_string());
        }

        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
    pub fn build(self) -> Result<NewComputer, Vec<String>> {
        match self.validate() {
            Err(e) => {
                Err(e)
            },
            Ok(_) => {
                Ok(
                    NewComputer {
                        equipmentsetid: self.equipmentsetid,
                        brand: self.brand.unwrap(),
                        operating_system: self.operating_system.unwrap(),
                        quizmachine_version: self.quizmachine_version.unwrap(),
                        wifi_capabilities: self.wifi_capabilities.unwrap(),
                        login_username: self.login_username.unwrap(),
                        login_password: self.login_password.unwrap(),
                        has_vga_out_port: self.has_vga_out_port.unwrap(),
                        has_dvi_out_port: self.has_dvi_out_port.unwrap(),
                        has_hdmi_out_port: self.has_hdmi_out_port.unwrap(),
                        has_display_port_out: self.has_display_port_out.unwrap(),
                        has_usb_port: self.has_usb_port.unwrap(),
                        misc_note: self.misc_note,
                    }
                )
            }
        }
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<Computer> {
        let new_computer = self.build();
        create(db, &new_computer.unwrap())
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
#[diesel(table_name = crate::schema::computers)]
#[diesel(primary_key(computerid))]
struct ComputerDbo {
    pub computerid: i64,
    pub brand: String,
    pub operating_system: String,
    pub quizmachine_version: String,
    pub wifi_capabilities: String,
    pub login_username: String,
    pub login_password: String,
    pub has_vga_out_port: bool,
    pub has_dvi_out_port: bool,
    pub has_hdmi_out_port: bool,
    pub has_display_port_out: bool,
    pub has_usb_port: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl ComputerDbo {
    pub fn to_model(&self, equipment_dbo: EquipmentDbo) -> Computer {
        Computer {
            equipmentid: equipment_dbo.id,
            computerid: self.computerid,
            equipmentsetid: equipment_dbo.equipmentsetid,
            brand: self.brand.clone(),
            operating_system: self.operating_system.clone(),
            quizmachine_version: self.quizmachine_version.clone(),
            wifi_capabilities: self.wifi_capabilities.clone(),
            login_username: self.login_username.clone(),
            login_password: self.login_password.clone(),
            has_vga_out_port: self.has_vga_out_port,
            has_dvi_out_port: self.has_dvi_out_port,
            has_hdmi_out_port: self.has_hdmi_out_port,
            has_display_port_out: self.has_display_port_out,
            has_usb_port: self.has_usb_port,
            misc_note: equipment_dbo.misc_note,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone
)]
pub struct Computer {
    pub equipmentid: i64,
    pub computerid: i64,
    pub equipmentsetid: i64,
    pub brand: String,
    pub operating_system: String,
    pub quizmachine_version: String,
    pub wifi_capabilities: String,
    pub login_username: String,
    pub login_password: String,
    pub has_vga_out_port: bool,
    pub has_dvi_out_port: bool,
    pub has_hdmi_out_port: bool,
    pub has_display_port_out: bool,
    pub has_usb_port: bool,
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
#[diesel(table_name = crate::schema::computers)]
struct NewComputerDbo {
    pub brand: String,
    pub operating_system: String,
    pub quizmachine_version: String,
    pub wifi_capabilities: String,
    pub login_username: Option<String>,
    pub login_password: Option<String>,
    pub has_vga_out_port: bool,
    pub has_dvi_out_port: bool,
    pub has_hdmi_out_port: bool,
    pub has_display_port_out: bool,
    pub has_usb_port: bool,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone
)]
pub struct NewComputer {
    pub equipmentsetid: i64,
    pub brand: String,
    pub operating_system: String,
    pub quizmachine_version: String,
    pub wifi_capabilities: String,
    pub login_username: String,
    pub login_password: String,
    pub has_vga_out_port: bool,
    pub has_dvi_out_port: bool,
    pub has_hdmi_out_port: bool,
    pub has_display_port_out: bool,
    pub has_usb_port: bool,
    pub misc_note: Option<String>,
}
impl NewComputer {
    fn to_dbo(&self) -> NewComputerDbo {
        NewComputerDbo {
            brand: self.brand.clone(),
            operating_system: self.operating_system.clone(),
            quizmachine_version: self.quizmachine_version.clone(),
            wifi_capabilities: self.wifi_capabilities.clone(),
            login_username: Some(self.login_username.clone()),
            login_password: Some(self.login_password.clone()),
            has_vga_out_port: self.has_vga_out_port,
            has_dvi_out_port: self.has_dvi_out_port,
            has_hdmi_out_port: self.has_hdmi_out_port,
            has_display_port_out: self.has_display_port_out,
            has_usb_port: self.has_usb_port,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::computers)]
#[diesel(primary_key(computerid))]
struct ComputerDboChangeset {
    pub brand: Option<String>,
    pub operating_system: Option<String>,
    pub quizmachine_version: Option<String>,
    pub wifi_capabilities: Option<String>,
    pub login_username: Option<String>,
    pub login_password: Option<String>,
    pub has_vga_out_port: Option<bool>,
    pub has_dvi_out_port: Option<bool>,
    pub has_hdmi_out_port: Option<bool>,
    pub has_display_port_out: Option<bool>,
    pub has_usb_port: Option<bool>,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone
)]
pub struct ComputerChangeSet {
    pub equipmentsetid: Option<i64>,
    pub brand: Option<String>,
    pub operating_system: Option<String>,
    pub quizmachine_version: Option<String>,
    pub wifi_capabilities: Option<String>,
    pub login_username: Option<String>,
    pub login_password: Option<String>,
    pub has_vga_out_port: Option<bool>,
    pub has_dvi_out_port: Option<bool>,
    pub has_hdmi_out_port: Option<bool>,
    pub has_display_port_out: Option<bool>,
    pub has_usb_port: Option<bool>,
    pub misc_note: Option<String>,
}
impl ComputerChangeSet {
    fn to_dbos(&self) -> (ComputerDboChangeset, EquipmentDboChangeset) {
        let clone_of_self = self.clone();
        (
            ComputerDboChangeset {
                brand: self.brand.clone(),
                operating_system: self.operating_system.clone(),
                quizmachine_version: self.quizmachine_version.clone(),
                wifi_capabilities: self.wifi_capabilities.clone(),
                login_username: self.login_username.clone(),
                login_password: self.login_password.clone(),
                has_vga_out_port: self.has_vga_out_port,
                has_dvi_out_port: self.has_dvi_out_port,
                has_hdmi_out_port: self.has_hdmi_out_port,
                has_display_port_out: self.has_display_port_out,
                has_usb_port: self.has_usb_port,
            },
            EquipmentDboChangeset {
                misc_note: clone_of_self.misc_note,
                equipmentsetid: clone_of_self.equipmentsetid
            }
        )
    }
}

fn covert_to_model_from_dbos(computer_dbo: ComputerDbo, equipment_dbo: EquipmentDbo) -> Computer {
    Computer {
        computerid: computer_dbo.computerid,
        brand: computer_dbo.brand,
        operating_system: computer_dbo.operating_system,
        quizmachine_version: computer_dbo.quizmachine_version,
        wifi_capabilities: computer_dbo.wifi_capabilities,
        login_username: computer_dbo.login_username,
        login_password: computer_dbo.login_password,
        has_vga_out_port: computer_dbo.has_vga_out_port,
        has_dvi_out_port: computer_dbo.has_dvi_out_port,
        has_hdmi_out_port: computer_dbo.has_hdmi_out_port,
        has_display_port_out: computer_dbo.has_display_port_out,
        has_usb_port: computer_dbo.has_usb_port,

        equipmentid: equipment_dbo.id,
        equipmentsetid: equipment_dbo.equipmentsetid,
        misc_note: equipment_dbo.misc_note,
        created_at: equipment_dbo.created_at,
        updated_at: equipment_dbo.updated_at,
    }
}

pub fn create(db: &mut database::Connection, item: &NewComputer) -> QueryResult<Computer> {
    use crate::schema::computers::dsl::*;

    let misc_note = item.clone().misc_note;
    let equipmentsetid = item.equipmentsetid;
    let new_computer_dbo = item.to_dbo();

    let computer_dbo_result = 
        insert_into(computers)
            .values(new_computer_dbo)
            .get_result::<ComputerDbo>(db);

    if computer_dbo_result.is_err() {
        return Err(computer_dbo_result.err().unwrap());
    }

    let computer_dbo = computer_dbo_result.unwrap();

    let equipment_dbo_result  = 
        EquipmentDboBuilder::new()
            .set_computerid(Some(computer_dbo.computerid))
            .set_misc_note(misc_note)
            .set_equipmentsetid(Some(equipmentsetid))
            .build_and_insert(db);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo = equipment_dbo_result.unwrap();

    Ok(computer_dbo.to_model(equipment_dbo))
}


pub fn exists(db: &mut database::Connection, computer_id: i64) -> bool {
    use crate::schema::computers::dsl::*;
    computers
        .find(computer_id)
        .get_result::<ComputerDbo>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, equipment_dbo_id: i64) -> QueryResult<Computer> {
    use crate::schema::computers::dsl::*;

    let equipment_dbo_result = models::equipment_dbo::read(db, equipment_dbo_id);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo = equipment_dbo_result.unwrap();

    if equipment_dbo.computerid.is_none() {
        println!("EquipmentDbo of ID {} has no computerid. Could not retrieve computer from DB.", equipment_dbo.id);
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: EquipmentDbo of ID {} has no computerid. Could not retrieve computer from DB.", equipment_dbo.id).into()
        ));
    }

    let computer_dbo_result = 
        computers
            .filter(computerid.eq(equipment_dbo.computerid.unwrap()))
            .first::<ComputerDbo>(db);
    
    if computer_dbo_result.is_err() {
        return Err(computer_dbo_result.err().unwrap());
    }
    
    Ok(computer_dbo_result.unwrap().to_model(equipment_dbo))
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<Computer>> {
    use crate::schema::computers::dsl::*;
    use crate::schema::equipment::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    let tuple_result: Result<Vec<(EquipmentDbo, ComputerDbo)>, result::Error> =
        equipment
            .inner_join(computers)
            .order(crate::schema::computers::dsl::computerid)
            .limit(page_size)
            .offset(offset_val)
            .load::<(EquipmentDbo, ComputerDbo)>(db);
    
    if tuple_result.is_err() {
        return Err(tuple_result.err().unwrap());
    }

    Ok(
        tuple_result
            .unwrap()
            .into_iter()
            .map(|c: (EquipmentDbo, ComputerDbo)| c.1.to_model(c.0))
            .collect()
    )
}

pub fn update(db: &mut database::Connection, equipment_id: i64, item: &ComputerChangeSet) -> QueryResult<Computer> {
    use crate::schema::computers::dsl::*;

    let equipment_dbo_result = models::equipment_dbo::read(db, equipment_id);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo: EquipmentDbo = equipment_dbo_result.unwrap();

    let (computer_dbo_changeset, equipment_dbo_changeset) = item.to_dbos();

    if equipment_dbo.computerid.is_none() {
        println!("EquipmentDbo's computerid is none. Computer could not be updated.");
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: EquipmentDbo's computerid is none. Computer could not be updated.").into()
        ));
    }

    let computer_dbo_result = 
        diesel::update(
            computers
                .filter(computerid.eq(equipment_dbo.computerid.unwrap())))
                .set((
                    computer_dbo_changeset,
                    updated_at.eq(diesel::dsl::now),
                )
        )
            .get_result::<ComputerDbo>(db);

    if computer_dbo_result.is_err() {
        return Err(computer_dbo_result.err().unwrap());
    }

    let equipment_dbo_result = 
        models::equipment_dbo::update(db, equipment_dbo.id, &equipment_dbo_changeset);

    if equipment_dbo_result.is_err() {
        return Err(computer_dbo_result.err().unwrap());
    }

    Ok(
        covert_to_model_from_dbos(
            computer_dbo_result.unwrap(), 
            equipment_dbo_result.unwrap()
        )
    )
}

pub fn delete(db: &mut database::Connection, equipment_id: i64) -> QueryResult<(usize, usize)> {
    use crate::schema::computers::dsl::*;

    let equipment_dbo_result = models::equipment_dbo::read(db, equipment_id);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo: EquipmentDbo = equipment_dbo_result.unwrap();

    if equipment_dbo.computerid.is_none() {
        println!("EquipmentDbo's computerid is none. Computer could not be updated.");
        return Err(diesel::result::Error::QueryBuilderError(
            format!("Error: EquipmentDbo's computerid is none. Computer could not be updated.").into()
        ));
    }

    let equipment_dbo_delete_result = models::equipment_dbo::delete(db, equipment_id);

    if equipment_dbo_delete_result.is_err() {
        return Err(equipment_dbo_delete_result.err().unwrap());
    }

    let computer_delete_result = 
        diesel::delete(computers.filter(computerid.eq(equipment_dbo.computerid.unwrap()))).execute(db);

    if computer_delete_result.is_err() {
        return Err(computer_delete_result.err().unwrap());
    }

    Ok((computer_delete_result.unwrap(), equipment_dbo_delete_result.unwrap()))
}
