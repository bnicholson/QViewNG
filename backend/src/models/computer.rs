
use crate::database;
use crate::models::common::PaginationParams;
// use crate::models::equipment::EquipmentBuilder;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{Utc,DateTime};

#[derive(Debug, Clone)]
pub struct ComputerBuilder {
    // equipmentsetid: i64,
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
    // misc_note: Option<String>,
}

impl ComputerBuilder {
    // pub fn new(equipmentsetid: i64) -> Self {
    pub fn new() -> Self {
        Self {
            // equipmentsetid,
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
            // misc_note: None,
        }
    }
    // pub fn new_default(equipmentsetid: i64) -> Self {
    pub fn new_default() -> Self {
        Self {
            // equipmentsetid,
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
            // misc_note: None,
        }
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
    // pub fn set_misc_note(mut self, misc_note: Option<String>) -> Self {
    //     self.misc_note = misc_note;
    //     self
    // }
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
        // if self.misc_note.is_none() {
        //     errors.push("misc_note is required".to_string());
        // }

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
                        // equipmentsetid: self.equipmentsetid,
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
                        // misc_note: self.misc_note,
                    }
                )
            }
        }
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<Computer> {
        // let equipmentsetid = self.equipmentsetid;
        // let misc_note = self.clone().misc_note.unwrap();

        let new_computer = self.build();
        create(db, &new_computer.unwrap())
        // let computer_dbo = create(db, &new_computer_dbo.unwrap()).unwrap();

        // EquipmentBuilder::new()
        //     .set_computerid(Some(computer_dbo.computerid))
        //     .set_misc_note(Some(misc_note))
        //     .set_equipmentsetid(Some(equipmentsetid))
        //     .build_and_insert(db)
        //     .unwrap();

        // Ok(computer_dbo)
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
    pub fn to_model(&self) -> Computer {
        Computer {
            // equipmentid: 0, // to be filled in later
            computerid: self.computerid,
            // equipmentsetid: 0, // to be filled in later
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
            // misc_note: None, // to be filled in later
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
    // pub equipmentid: i64,
    pub computerid: i64,
    // pub equipmentsetid: i64,
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
    // pub misc_note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl Computer {
    pub fn to_dbo(&self) -> ComputerDbo {
        ComputerDbo {
            computerid: self.computerid,
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
impl NewComputerDbo {
    fn to_model(&self) -> NewComputer {
        NewComputer {
            // equipmentsetid: 0, // to be filled in later
            brand: self.brand.clone(),
            operating_system: self.operating_system.clone(),
            quizmachine_version: self.quizmachine_version.clone(),
            wifi_capabilities: self.wifi_capabilities.clone(),
            login_username: self.login_username.clone().unwrap_or_default(),
            login_password: self.login_password.clone().unwrap_or_default(),
            has_vga_out_port: self.has_vga_out_port,
            has_dvi_out_port: self.has_dvi_out_port,
            has_hdmi_out_port: self.has_hdmi_out_port,
            has_display_port_out: self.has_display_port_out,
            has_usb_port: self.has_usb_port,
            // misc_note: None, // to be filled in later
        }
    }
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone
)]
pub struct NewComputer {
    // pub equipmentsetid: i64,
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
    // pub misc_note: Option<String>,
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
    fn to_dbo(&self) -> ComputerDboChangeset {
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
        }
    }
}

pub fn create(db: &mut database::Connection, item: &NewComputer) -> QueryResult<Computer> {
    use crate::schema::computers::dsl::*;

    let new_computer_dbo = item.to_dbo();

    let computer_dbo = insert_into(computers)
        .values(new_computer_dbo)
        .get_result::<ComputerDbo>(db)?;
    Ok(computer_dbo.to_model())
}


pub fn exists(db: &mut database::Connection, computer_id: i64) -> bool {
    use crate::schema::computers::dsl::*;
    computers
        .find(computer_id)
        .get_result::<ComputerDbo>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, item_id: i64) -> QueryResult<Computer> {
    use crate::schema::computers::dsl::*;
    let computer_dbo = computers.filter(computerid.eq(item_id)).first::<ComputerDbo>(db).unwrap();
    Ok(computer_dbo.to_model())
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<Computer>> {
    use crate::schema::computers::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    Ok(
        computers
            .order(created_at)
            .limit(page_size)
            .offset(offset_val)
            .load::<ComputerDbo>(db)
            .unwrap()
            .into_iter()
            .map(|dbo| dbo.to_model())
            .collect()
    )
}

pub fn update(db: &mut database::Connection, item_id: i64, item: &ComputerChangeSet) -> QueryResult<Computer> {
    use crate::schema::computers::dsl::*;

    let computer_dbo = 
        diesel::update(computers.filter(computerid.eq(item_id)))
            .set((
                item.to_dbo(),
                updated_at.eq(diesel::dsl::now),
            ))
            .get_result::<ComputerDbo>(db);

    if computer_dbo.is_err() {
        return Err(computer_dbo.err().unwrap());
    }

    Ok(computer_dbo.unwrap().to_model())
}

pub fn delete(db: &mut database::Connection, item_id: i64) -> QueryResult<usize> {
    use crate::schema::computers::dsl::*;
    diesel::delete(computers.filter(computerid.eq(item_id))).execute(db)
}
