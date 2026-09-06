
use crate::database;
use crate::models::common::PaginationParams;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{Utc,DateTime};
use uuid::Uuid;

pub struct EquipmentDboBuilder {
    pub computerid: Option<i64>,
    pub jumppadid: Option<i64>,
    pub interfaceboxid: Option<i64>,
    pub monitorid: Option<i64>,
    pub microphonerecorderid: Option<i64>,
    pub projectorid: Option<i64>,
    pub powerstripid: Option<i64>,
    pub extensioncordid: Option<i64>,
    pub misc_note: Option<String>,
    pub equipmentsetid: Option<i64>,
    pub creator_id: Option<Uuid>,
    pub last_modified_user: Option<Uuid>,
}

impl EquipmentDboBuilder {
    pub fn new() -> Self {
        Self {
            computerid: None,
            jumppadid: None,
            interfaceboxid: None,
            monitorid: None,
            microphonerecorderid: None,
            projectorid: None,
            powerstripid: None,
            extensioncordid: None,
            misc_note: None,
            equipmentsetid: None,
            creator_id: None,
            last_modified_user: None,
        }
    }
    pub fn new_default() -> Self {
        Self {
            computerid: None,
            jumppadid: None,
            interfaceboxid: None,
            monitorid: None,
            microphonerecorderid: None,
            projectorid: None,
            powerstripid: None,
            extensioncordid: None,
            misc_note: Some("".to_string()),
            equipmentsetid: None,
            creator_id: None,
            last_modified_user: None,
        }
    }
    pub fn set_computerid(mut self, computerid: Option<i64>) -> Self {
        self.computerid = computerid;
        self
    }
    pub fn set_jumppadid(mut self, jumppadid: Option<i64>) -> Self {
        self.jumppadid = jumppadid;
        self
    }
    pub fn set_interfaceboxid(mut self, interfaceboxid: Option<i64>) -> Self {
        self.interfaceboxid = interfaceboxid;
        self
    }
    pub fn set_monitorid(mut self, monitorid: Option<i64>) -> Self {
        self.monitorid = monitorid;
        self
    }
    pub fn set_microphonerecorderid(mut self, microphonerecorderid: Option<i64>) -> Self {
        self.microphonerecorderid = microphonerecorderid;
        self
    }
    pub fn set_projectorid(mut self, projectorid: Option<i64>) -> Self {
        self.projectorid = projectorid;
        self
    }
    pub fn set_powerstripid(mut self, powerstripid: Option<i64>) -> Self {
        self.powerstripid = powerstripid;
        self
    }
    pub fn set_extensioncordid(mut self, extensioncordid: Option<i64>) -> Self {
        self.extensioncordid = extensioncordid;
        self
    }
    pub fn set_misc_note(mut self, misc_note: Option<String>) -> Self {
        self.misc_note = misc_note;
        self
    }
    pub fn set_creator_id(mut self, user_id: Uuid) -> Self {
        self.creator_id = Some(user_id);
        self
    }
    pub fn set_last_modified_user(mut self, user_id: Uuid) -> Self {
        self.last_modified_user = Some(user_id);
        self
    }
    pub fn set_equipmentsetid(mut self, equipmentsetid: Option<i64>) -> Self {
        self.equipmentsetid = equipmentsetid;
        self
    }
    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        let mut equipment_type_id_counter = 0;
        for equipment_type_id in [
            &self.computerid,
            &self.jumppadid,
            &self.interfaceboxid,
            &self.monitorid,
            &self.microphonerecorderid,
            &self.projectorid,
            &self.powerstripid,
            &self.extensioncordid,
        ] {
            if equipment_type_id.is_some() {
                equipment_type_id_counter += 1;
            }
        }
        if equipment_type_id_counter != 1 {
            errors.push("exactly one equipment type ID is required".to_string());
        }
        if self.equipmentsetid.is_none() {
            errors.push("equipmentsetid is required".to_string());
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
    pub fn build(self) -> Result<NewEquipmentDbo, Vec<String>> {
        match self.validate() {
            Err(e) => {
                Err(e)
            },
            Ok(_) => {
                Ok(
                    NewEquipmentDbo {
                        computerid: self.computerid,
                        jumppadid: self.jumppadid,
                        interfaceboxid: self.interfaceboxid,
                        monitorid: self.monitorid,
                        microphonerecorderid: self.microphonerecorderid,
                        projectorid: self.projectorid,
                        powerstripid: self.powerstripid,
                        extensioncordid: self.extensioncordid,
                        misc_note: self.misc_note,
                        equipmentsetid: self.equipmentsetid.unwrap(),
                        creator_id: self.creator_id.unwrap_or_else(Uuid::nil),
                        last_modified_user: self.last_modified_user.unwrap_or_else(Uuid::nil),
                    }
                )
            }
        }
    }
    pub fn build_and_insert(mut self, db: &mut database::Connection) -> QueryResult<EquipmentDbo> {
        // For seed/test convenience: attribute to the owner of the parent equipment set when unset.
        if self.creator_id.is_none() || self.last_modified_user.is_none() {
            if let Some(set_id) = self.equipmentsetid {
                if let Ok(owner) = owner_of_set(db, set_id) {
                    self.creator_id = self.creator_id.or(Some(owner));
                    self.last_modified_user = self.last_modified_user.or(Some(owner));
                }
            }
        }
        let new_equipment = self.build();
        create(db, &new_equipment.unwrap())
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
#[diesel(table_name = crate::schema::equipment)]
#[diesel(primary_key(id))]
pub struct EquipmentDbo {
    pub id: i64,                           // identifies all equipment uniquely
    pub computerid: Option<i64>,
    pub jumppadid: Option<i64>,
    pub interfaceboxid: Option<i64>,
    pub monitorid: Option<i64>,
    pub microphonerecorderid: Option<i64>,
    pub projectorid: Option<i64>,
    pub powerstripid: Option<i64>,
    pub extensioncordid: Option<i64>,
    pub misc_note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub equipmentsetid: i64,
    pub creator_id: Uuid,
    pub last_modified_user: Uuid,
}

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug
)]
#[diesel(table_name = crate::schema::equipment)]
pub struct NewEquipmentDbo {
    pub computerid: Option<i64>,
    pub jumppadid: Option<i64>,
    pub interfaceboxid: Option<i64>,
    pub monitorid: Option<i64>,
    pub microphonerecorderid: Option<i64>,
    pub projectorid: Option<i64>,
    pub powerstripid: Option<i64>,
    pub extensioncordid: Option<i64>,
    pub misc_note: Option<String>,
    pub equipmentsetid: i64,
    // Set server-side from the authenticated user; a client-sent value is ignored/overwritten.
    #[serde(default)]
    pub creator_id: Uuid,
    #[serde(default)]
    pub last_modified_user: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::equipment)]
#[diesel(primary_key(id))]
pub struct EquipmentDboChangeset {
    pub misc_note: Option<String>,
    pub equipmentsetid: Option<i64>,
}

pub fn create(db: &mut database::Connection, item: &NewEquipmentDbo) -> QueryResult<EquipmentDbo> {
    use crate::schema::equipment::dsl::*;
    insert_into(equipment)
        .values(item)
        .get_result::<EquipmentDbo>(db)
}

pub fn exists(db: &mut database::Connection, equipmentid: i64) -> bool {
    use crate::schema::equipment::dsl::equipment;
    equipment
        .find(equipmentid)
        .get_result::<EquipmentDbo>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, item_id: i64) -> QueryResult<EquipmentDbo> {
    use crate::schema::equipment::dsl::*;
    equipment.filter(id.eq(item_id)).first::<EquipmentDbo>(db)
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<EquipmentDbo>> {
    use crate::schema::equipment::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    equipment
        .order(created_at)
        .limit(page_size)
        .offset(offset_val)
        .load::<EquipmentDbo>(db)
}

/// One fully-formed row of the user's "My Gear" data table: a gear item plus its set name, a
/// derived type label, the type-specific detail (make/model/etc.), and the display names of its
/// creator and last-modifier. Populated in a single API call (no per-item detail fetch).
#[derive(Debug, Serialize, Deserialize)]
pub struct UserGearRow {
    pub id: i64,
    pub gear_type: String,
    pub equipmentsetid: i64,
    pub set_name: String,
    pub misc_note: Option<String>,
    /// Same tagged shape as the /api/equipment/{id} response, e.g. `{ "Computer": { ... } }`.
    pub detail: Option<crate::models::equipment::Equipment>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub creator_id: Uuid,
    pub creator_name: String,
    pub last_modified_user_id: Uuid,
    pub last_modified_user_name: String,
}

fn gear_type_of(item: &EquipmentDbo) -> String {
    if item.computerid.is_some() { "Computer" }
    else if item.jumppadid.is_some() { "Jump Pad" }
    else if item.interfaceboxid.is_some() { "Interface Box" }
    else if item.monitorid.is_some() { "Monitor" }
    else if item.microphonerecorderid.is_some() { "Microphone Recorder" }
    else if item.projectorid.is_some() { "Projector" }
    else if item.powerstripid.is_some() { "Power Strip" }
    else if item.extensioncordid.is_some() { "Extension Cord" }
    else { "Unknown" }.to_string()
}

/// Returns one page of the gear items across all equipment sets owned by `owner_id`, enriched with
/// set name, type, and creator/last-modifier display names, plus the total count. One scoped call.
pub fn read_gear_rows_of_owner(
    db: &mut database::Connection,
    owner_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<(Vec<UserGearRow>, i64)> {
    let set_name_by_id: std::collections::HashMap<i64, String> = {
        use crate::schema::equipmentsets::dsl::*;
        equipmentsets
            .filter(equipmentownerid.eq(owner_id))
            .select((id, name))
            .load::<(i64, String)>(db)?
            .into_iter()
            .collect()
    };
    let set_ids: Vec<i64> = set_name_by_id.keys().cloned().collect();

    let total: i64 = {
        use crate::schema::equipment::dsl::*;
        equipment.filter(equipmentsetid.eq_any(&set_ids)).count().get_result(db)?
    };
    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;
    let list: Vec<EquipmentDbo> = {
        use crate::schema::equipment::dsl::*;
        equipment
            .filter(equipmentsetid.eq_any(&set_ids))
            .order((created_at.asc(), id.asc()))
            .limit(page_size)
            .offset(offset_val)
            .load::<EquipmentDbo>(db)?
    };

    let mut user_ids: Vec<Uuid> = list.iter().map(|e| e.creator_id).collect();
    user_ids.extend(list.iter().map(|e| e.last_modified_user));
    let name_by_id = crate::models::user::read_display_names(db, &user_ids)?;
    let name_of = |uid: Uuid| name_by_id.get(&uid).cloned().unwrap_or_else(|| uid.to_string());

    // Resolve each item's type-specific detail up front (borrows db), so the single call carries
    // everything the table renders — no per-item detail fetch on the client.
    let details: Vec<Option<crate::models::equipment::Equipment>> = list
        .iter()
        .map(|e| crate::models::equipment::read(db, e.id).ok())
        .collect();

    let rows = list
        .into_iter()
        .zip(details)
        .map(|(e, detail)| UserGearRow {
            gear_type: gear_type_of(&e),
            set_name: set_name_by_id.get(&e.equipmentsetid).cloned().unwrap_or_default(),
            creator_name: name_of(e.creator_id),
            last_modified_user_name: name_of(e.last_modified_user),
            detail,
            id: e.id,
            equipmentsetid: e.equipmentsetid,
            misc_note: e.misc_note,
            created_at: e.created_at,
            updated_at: e.updated_at,
            creator_id: e.creator_id,
            last_modified_user_id: e.last_modified_user,
        })
        .collect();
    Ok((rows, total))
}

/// The owner (from the parent equipment set) that a gear item defaults its creator/modifier to.
pub fn owner_of_set(db: &mut database::Connection, set_id: i64) -> QueryResult<Uuid> {
    use crate::schema::equipmentsets::dsl::*;
    equipmentsets.filter(id.eq(set_id)).select(equipmentownerid).first::<Uuid>(db)
}

pub fn update(db: &mut database::Connection, item_id: i64, item: &EquipmentDboChangeset, modified_by: Uuid) -> QueryResult<EquipmentDbo> {
    use crate::schema::equipment::dsl::*;
    diesel::update(equipment.filter(id.eq(item_id)))
        .set((
            item,
            updated_at.eq(diesel::dsl::now),
            last_modified_user.eq(modified_by),
        ))
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, item_id: i64) -> QueryResult<usize> {
    use crate::schema::equipment::dsl::*;
    diesel::delete(equipment.filter(id.eq(item_id))).execute(db)
}

pub fn read_all_by_equipmentset(db: &mut database::Connection, set_id: i64) -> QueryResult<Vec<EquipmentDbo>> {
    use crate::schema::equipment::dsl::*;
    equipment
        .filter(equipmentsetid.eq(set_id))
        .order(created_at)
        .load::<EquipmentDbo>(db)
}
