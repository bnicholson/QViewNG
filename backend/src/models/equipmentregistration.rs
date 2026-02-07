
use crate::database;
use crate::models::common::PaginationParams;
use diesel::prelude::*;
use diesel::*;
use diesel::{QueryResult,AsChangeset,Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{Utc,DateTime};

pub struct EquipmentRegistrationBuilder {
    equipmentid: i64,
    tournamentid: Uuid,
    roomid: Option<Uuid>,
    status: Option<String>,
}

impl EquipmentRegistrationBuilder {
    pub fn new(equipmentid: i64, tournamentid: Uuid) -> Self {
        Self {
            equipmentid,
            tournamentid,
            roomid: None,
            status: None,
        }
    }
    pub fn new_default(equipmentid: i64, tournamentid: Uuid) -> Self {
        Self {
            equipmentid,
            tournamentid,
            roomid: None,
            status: Some("Received from Owner".to_string()),
        }
    }
    pub fn set_name(mut self, equipmentid: i64) -> Self {
        self.equipmentid = equipmentid;
        self
    }
    pub fn set_tournamentid(mut self, tournamentid: Uuid) -> Self {
        self.tournamentid = tournamentid;
        self
    }
    pub fn set_roomid(mut self, roomid: Option<Uuid>) -> Self {
        self.roomid = roomid;
        self
    }
    pub fn set_status(mut self, status: Option<String>) -> Self {
        self.status = status;
        self
    }
    fn validate_all_are_some(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if self.status.is_none() {
            errors.push("status is required".to_string());
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
    pub fn build(self) -> Result<NewEquipmentRegistration, Vec<String>> {
        match self.validate_all_are_some() {
            Err(e) => {
                Err(e)
            },
            Ok(_) => {
                Ok(
                    NewEquipmentRegistration {
                        equipmentid: self.equipmentid,
                        tournamentid: self.tournamentid,
                        roomid: self.roomid,
                        status: self.status.unwrap(),
                    }
                )
            }
        }
    }
    pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<EquipmentRegistration> {
        let new_equipmentregistration = self.build();
        create(db, &new_equipmentregistration.unwrap())
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
#[diesel(table_name = crate::schema::equipmentregistrations)]
#[diesel(primary_key(id))]
pub struct EquipmentRegistration {
    pub id: i64,
    pub equipmentid: i64,
    pub tournamentid: Uuid,
    pub roomid: Option<Uuid>,
    pub status: String,  // 'Received from Owner', 'Prepared for Assignment', 'Assigned to Room', 'Deployed to Room', 'On Standby', 'Returned from Room', 'Needs Repair', 'Returned to Owner'
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(
    Insertable,
    Serialize,
    Deserialize,
    Debug
)]
#[diesel(table_name = crate::schema::equipmentregistrations)]
pub struct NewEquipmentRegistration {
    pub equipmentid: i64,
    pub tournamentid: Uuid,
    pub roomid: Option<Uuid>,
    pub status: String,
}

// #[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::equipmentregistrations)]
#[diesel(primary_key(id))]
pub struct EquipmentRegistrationChangeset {
    pub roomid: Option<Uuid>,
    pub status: Option<String>,
}

pub fn create(db: &mut database::Connection, item: &NewEquipmentRegistration) -> QueryResult<EquipmentRegistration> {
    use crate::schema::equipmentregistrations::dsl::*;
    insert_into(equipmentregistrations).values(item).get_result::<EquipmentRegistration>(db)
}

pub fn exists(db: &mut database::Connection, equipmentregistrationid: i64) -> bool {
    use crate::schema::equipmentregistrations::dsl::equipmentregistrations;
    equipmentregistrations
        .find(equipmentregistrationid)
        .get_result::<EquipmentRegistration>(db)
        .is_ok()
}

pub fn read(db: &mut database::Connection, item_id: i64) -> QueryResult<EquipmentRegistration> {
    use crate::schema::equipmentregistrations::dsl::*;
    equipmentregistrations.filter(id.eq(item_id)).first::<EquipmentRegistration>(db)
}

pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<EquipmentRegistration>> {
    use crate::schema::equipmentregistrations::dsl::*;
    equipmentregistrations
        .order(created_at)
        .limit(pagination.page_size)
        .offset(
            pagination.page
                * std::cmp::max(pagination.page_size, PaginationParams::MAX_PAGE_SIZE as i64),
        )
        .load::<EquipmentRegistration>(db)
}

pub fn read_all_equipmentregistrations_of_tournament(
    db: &mut database::Connection,
    tournament_id: Uuid,
    pagination: &PaginationParams,
) -> QueryResult<Vec<EquipmentRegistration>> {
    use crate::schema::equipmentregistrations::dsl::*;

    let page_size = pagination.page_size.min(PaginationParams::MAX_PAGE_SIZE as i64);
    let offset_val = pagination.page * page_size;

    equipmentregistrations
        .filter(tournamentid.eq(tournament_id))
        .order(id.asc())
        .limit(page_size)
        .offset(offset_val)
        .load::<EquipmentRegistration>(db)
}

pub fn update(db: &mut database::Connection, item_id: i64, item: &EquipmentRegistrationChangeset) -> QueryResult<EquipmentRegistration> {
    use crate::schema::equipmentregistrations::dsl::*;
    diesel::update(equipmentregistrations.filter(id.eq(item_id)))
        .set((
            item,
            updated_at.eq(diesel::dsl::now),
        ))
        .get_result(db)
}

pub fn delete(db: &mut database::Connection, item_id: i64) -> QueryResult<usize> {
    use crate::schema::equipmentregistrations::dsl::*;
    diesel::delete(equipmentregistrations.filter(id.eq(item_id))).execute(db)
}
