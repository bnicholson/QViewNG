
use crate::database;
use crate::models::common::PaginationParams;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{Utc,DateTime};

pub struct EquipmentSetBuilder {
    equipmentownerid: Uuid,
    is_active: Option<bool>,
    is_default: Option<bool>,
    name: Option<String>,
    description: Option<String>,
}

impl EquipmentSetBuilder {
    pub fn new(equipmentownerid: Uuid) -> Self {
        Self {
            equipmentownerid,
            is_active: None,
            is_default: None,
            name: None,
            description: None,
        }
    }
    pub fn new_default(equipmentownerid: Uuid) -> Self {
        Self {
            equipmentownerid,
            is_active: Some(true),
            is_default: Some(false),
            name: Some("Default Equipment Set".to_string()),
            description: Some("This is the description.".to_string()),
        }
    }
    pub fn set_equipmentownerid(mut self, equipmentownerid: Uuid) -> Self {
        self.equipmentownerid = equipmentownerid;
        self
    }
    pub fn set_is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }
    pub fn set_is_default(mut self, is_default: bool) -> Self {
        self.is_default = Some(is_default);
        self
    }
    pub fn set_name(mut self, equipmentset_name: &str) -> Self {
        self.name = Some(equipmentset_name.to_string());
        self
    }
    pub fn set_description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }
    fn validate_all_are_some(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if self.name.is_none() {
            errors.push("name is required".to_string());
        }
        if self.is_active.is_none() {
            errors.push("is_active is required".to_string());
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
    pub fn build(self) -> Result<NewEquipmentSet, Vec<String>> {
        match self.validate_all_are_some() {
            Err(e) => {
                Err(e)
            },
            Ok(_) => {
                Ok(
                    NewEquipmentSet {
                        equipmentownerid: self.equipmentownerid,
                        is_active: self.is_active.unwrap(),
                        is_default: self.is_default,
                        name: self.name.unwrap(),
                        description: self.description,
                    }
                )
            }
        }
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<EquipmentSet> {
        let new_equipmentset = self.build();
        create(db, &new_equipmentset.unwrap())
    }
}

// #[tsync::tsync]
#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Identifiable,
    ToSchema
)]
#[diesel(table_name = crate::schema::equipmentsets)]
#[diesel(primary_key(id))]
pub struct EquipmentSet {
    pub id: i64,                           // identifies the equipmentset uniquely
    pub equipmentownerid: Uuid,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_default: Option<bool>,
    pub name: String,                      // Name of the equipmentset (human readable)
    pub description: Option<String>,
}

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug
)]
#[diesel(table_name = crate::schema::equipmentsets)]
pub struct NewEquipmentSet {
    pub equipmentownerid: Uuid,
    pub is_active: bool,
    pub is_default: Option<bool>,
    pub name: String,
    pub description: Option<String>,
}

// #[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::equipmentsets)]
#[diesel(primary_key(id))]
pub struct EquipmentSetChangeset {
    pub equipmentownerid: Uuid,
    pub is_active: bool,
    pub is_default: Option<bool>,
    pub name: String,
    pub description: Option<String>,
}

pub fn create(db: &mut database::Connection, item: &NewEquipmentSet) -> QueryResult<EquipmentSet> {
    use crate::schema::equipmentsets::dsl::*;
    insert_into(equipmentsets).values(item).get_result::<EquipmentSet>(db)
}

pub fn exists(db: &mut database::Connection, equipmentsetid: i64) -> bool {
    use crate::schema::equipmentsets::dsl::equipmentsets;
    equipmentsets
        .find(equipmentsetid)
        .get_result::<EquipmentSet>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, item_id: i64) -> QueryResult<EquipmentSet> {
    use crate::schema::equipmentsets::dsl::*;
    equipmentsets.filter(id.eq(item_id)).first::<EquipmentSet>(db)
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<EquipmentSet>> {
    use crate::schema::equipmentsets::dsl::*;
    equipmentsets
        .order(created_at)
        .limit(pagination.page_size)
        .offset(
            pagination.page
                * std::cmp::max(pagination.page_size, PaginationParams::MAX_PAGE_SIZE as i64),
        )
        .load::<EquipmentSet>(db)
}

pub fn update(db: &mut database::Connection, item_id: i64, item: &EquipmentSetChangeset) -> QueryResult<EquipmentSet> {
    use crate::schema::equipmentsets::dsl::*;
    diesel::update(equipmentsets.filter(id.eq(item_id)))
        .set((
            item,
            updated_at.eq(diesel::dsl::now),
        ))
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, item_id: i64) -> QueryResult<usize> {
    use crate::schema::equipmentsets::dsl::*;
    diesel::delete(equipmentsets.filter(id.eq(item_id))).execute(db)
}
