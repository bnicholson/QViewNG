
// use crate::database;
// use crate::models::common::PaginationParams;
// use diesel::prelude::*;
// use diesel::*;
// use diesel::{QueryResult,AsChangeset,Insertable};
// use serde::{Deserialize, Serialize};
// use utoipa::ToSchema;
// use chrono::{Utc,DateTime};

// pub struct EquipmentDboBuilder {
//     pub computerid: Option<i64>,
//     pub jumppadid: Option<i64>,
//     pub interfaceboxid: Option<i64>,
//     pub monitorid: Option<i64>,
//     pub microphonerecorderid: Option<i64>,
//     pub projectorid: Option<i64>,
//     pub powerstripid: Option<i64>,
//     pub extensioncordid: Option<i64>,
//     pub misc_note: Option<String>,
//     pub equipmentsetid: Option<i64>,
// }

// impl EquipmentDboBuilder {
//     pub fn new() -> Self {
//         Self {
//             computerid: None,
//             jumppadid: None,
//             interfaceboxid: None,
//             monitorid: None,
//             microphonerecorderid: None,
//             projectorid: None,
//             powerstripid: None,
//             extensioncordid: None,
//             misc_note: None,
//             equipmentsetid: None,
//         }
//     }
//     pub fn new_default() -> Self {
//         Self {
//             computerid: None,
//             jumppadid: None,
//             interfaceboxid: None,
//             monitorid: None,
//             microphonerecorderid: None,
//             projectorid: None,
//             powerstripid: None,
//             extensioncordid: None,
//             misc_note: Some("".to_string()),
//             equipmentsetid: None,
//         }
//     }
//     pub fn set_computerid(mut self, computerid: Option<i64>) -> Self {
//         self.computerid = computerid;
//         self
//     }
//     pub fn set_jumppadid(mut self, jumppadid: Option<i64>) -> Self {
//         self.jumppadid = jumppadid;
//         self
//     }
//     pub fn set_interfaceboxid(mut self, interfaceboxid: Option<i64>) -> Self {
//         self.interfaceboxid = interfaceboxid;
//         self
//     }
//     pub fn set_monitorid(mut self, monitorid: Option<i64>) -> Self {
//         self.monitorid = monitorid;
//         self
//     }
//     pub fn set_microphonerecorderid(mut self, microphonerecorderid: Option<i64>) -> Self {
//         self.microphonerecorderid = microphonerecorderid;
//         self
//     }
//     pub fn set_projectorid(mut self, projectorid: Option<i64>) -> Self {
//         self.projectorid = projectorid;
//         self
//     }
//     pub fn set_powerstripid(mut self, powerstripid: Option<i64>) -> Self {
//         self.powerstripid = powerstripid;
//         self
//     }
//     pub fn set_extensioncordid(mut self, extensioncordid: Option<i64>) -> Self {
//         self.extensioncordid = extensioncordid;
//         self
//     }
//     pub fn set_misc_note(mut self, misc_note: Option<String>) -> Self {
//         self.misc_note = misc_note;
//         self
//     }
//     pub fn set_equipmentsetid(mut self, equipmentsetid: Option<i64>) -> Self {
//         self.equipmentsetid = equipmentsetid;
//         self
//     }
//     fn validate(&self) -> Result<(), Vec<String>> {
//         let mut errors = Vec::new();

//         let mut equipment_type_id_counter = 0;
//         for equipment_type_id in [
//             &self.computerid,
//             &self.jumppadid,
//             &self.interfaceboxid,
//             &self.monitorid,
//             &self.microphonerecorderid,
//             &self.projectorid,
//             &self.powerstripid,
//             &self.extensioncordid,
//         ] {
//             if equipment_type_id.is_some() {
//                 equipment_type_id_counter += 1;
//             }
//         }
//         if equipment_type_id_counter != 1 {
//             errors.push("exactly one equipment type ID is required".to_string());
//         }

//         if self.misc_note.is_none() {
//             errors.push("misc_note is required".to_string());
//         }
//         if self.equipmentsetid.is_none() {
//             errors.push("equipmentsetid is required".to_string());
//         }
//         if !errors.is_empty() {
//             return Err(errors);
//         }
//         Ok(())
//     }
//     pub fn build(self) -> Result<NewEquipmentDbo, Vec<String>> {
//         match self.validate() {
//             Err(e) => {
//                 Err(e)
//             },
//             Ok(_) => {
//                 Ok(
//                     NewEquipmentDbo {
//                         computerid: self.computerid,
//                         jumppadid: self.jumppadid,
//                         interfaceboxid: self.interfaceboxid,
//                         monitorid: self.monitorid,
//                         microphonerecorderid: self.microphonerecorderid,
//                         projectorid: self.projectorid,
//                         powerstripid: self.powerstripid,
//                         extensioncordid: self.extensioncordid,
//                         misc_note: self.misc_note,
//                         equipmentsetid: self.equipmentsetid.unwrap(),
//                     }
//                 )
//             }
//         }
//     }
//     pub fn build_and_insert(self, db: &mut database::Connection) -> QueryResult<EquipmentDbo> {
//         let new_equipment = self.build();
//         create(db, &new_equipment.unwrap())
//     }
// }

// #[derive(
//     Debug,
//     Serialize,
//     Deserialize,
//     Clone,
//     Queryable,
//     QueryableByName,
//     Selectable,
//     Identifiable,
//     ToSchema
// )]
// #[diesel(check_for_backend(diesel::pg::Pg))]  // this shows which field has incorrect type compared to the schema.rs file
// #[diesel(table_name = crate::schema::equipment)]
// #[diesel(primary_key(id))]
// pub struct EquipmentDbo {
//     pub id: i64,                           // identifies all equipment uniquely
//     pub computerid: Option<i64>,
//     pub jumppadid: Option<i64>,
//     pub interfaceboxid: Option<i64>,
//     pub monitorid: Option<i64>,
//     pub microphonerecorderid: Option<i64>,
//     pub projectorid: Option<i64>,
//     pub powerstripid: Option<i64>,
//     pub extensioncordid: Option<i64>,
//     pub misc_note: Option<String>,
//     pub created_at: DateTime<Utc>,
//     pub updated_at: DateTime<Utc>,
//     pub equipmentsetid: i64,
// }

// #[derive(
//     Insertable,
//     Serialize,
//     Deserialize,
//     Debug
// )]
// #[diesel(table_name = crate::schema::equipment)]
// pub struct NewEquipmentDbo {
//     pub computerid: Option<i64>,
//     pub jumppadid: Option<i64>,
//     pub interfaceboxid: Option<i64>,
//     pub monitorid: Option<i64>,
//     pub microphonerecorderid: Option<i64>,
//     pub projectorid: Option<i64>,
//     pub powerstripid: Option<i64>,
//     pub extensioncordid: Option<i64>,
//     pub misc_note: Option<String>,
//     pub equipmentsetid: i64,
// }
// // impl NewEquipmentDbo {
// //     pub fn validate(&self) -> Result<(), Vec<String>> {
// //         let mut errors = Vec::new();

// //         let mut equipment_type_id_counter = 0;
// //         for equipment_type_id in [
// //             &self.computerid,
// //             &self.jumppadid,
// //             &self.interfaceboxid,
// //             &self.monitorid,
// //             &self.microphonerecorderid,
// //             &self.projectorid,
// //             &self.powerstripid,
// //             &self.extensioncordid,
// //         ] {
// //             if equipment_type_id.is_some() {
// //                 equipment_type_id_counter += 1;
// //             }
// //         }
// //         if equipment_type_id_counter != 1 {
// //             errors.push("exactly one equipment type ID is required".to_string());
// //         }

// //         if self.misc_note.is_none() {
// //             errors.push("misc_note is required".to_string());
// //         }
// //         if !errors.is_empty() {
// //             return Err(errors);
// //         }
// //         Ok(())
// //     }
// // }

// #[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
// #[diesel(table_name = crate::schema::equipment)]
// #[diesel(primary_key(id))]
// pub struct EquipmentChangesetDbo {
//     pub misc_note: Option<String>,
//     pub equipmentsetid: Option<i64>,
// }

// pub fn create(db: &mut database::Connection, item: &NewEquipmentDbo) -> QueryResult<EquipmentDbo> {
//     use crate::schema::equipment::dsl::*;
//     insert_into(equipment)
//         .values(item)
//         .get_result::<EquipmentDbo>(db)
// }

// pub fn exists(db: &mut database::Connection, equipmentid: i64) -> bool {
//     use crate::schema::equipment::dsl::equipment;
//     equipment
//         .find(equipmentid)
//         .get_result::<EquipmentDbo>(db)
//         .is_ok()
// }

// pub fn read(db: &mut database::Connection, item_id: i64) -> QueryResult<EquipmentDbo> {
//     use crate::schema::equipment::dsl::*;
//     equipment.filter(id.eq(item_id)).first::<EquipmentDbo>(db)
// }

// pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<EquipmentDbo>> {
//     use crate::schema::equipment::dsl::*;
//     equipment
//         .order(created_at)
//         .limit(pagination.page_size)
//         .offset(
//             pagination.page
//                 * std::cmp::max(pagination.page_size, PaginationParams::MAX_PAGE_SIZE as i64),
//         )
//         .load::<EquipmentDbo>(db)
// }

// pub fn update(db: &mut database::Connection, item_id: i64, item: &EquipmentChangesetDbo) -> QueryResult<EquipmentDbo> {
//     use crate::schema::equipment::dsl::*;
//     diesel::update(equipment.filter(id.eq(item_id)))
//         .set((
//             item,
//             updated_at.eq(diesel::dsl::now),
//         ))
//         .get_result(db)
// }

// pub fn delete(db: &mut database::Connection, item_id: i64) -> QueryResult<usize> {
//     use crate::schema::equipment::dsl::*;
//     diesel::delete(equipment.filter(id.eq(item_id))).execute(db)
// }
