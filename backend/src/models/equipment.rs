use crate::{database, models::{self, computer::Computer, equipment_dbo::EquipmentDbo, extensioncord::ExtensionCord, interfacebox::InterfaceBox, jumppad::JumpPad, microphonerecorder::MicrophoneRecorder, monitor::Monitor, powerstrip::PowerStrip, projector::Projector}};
use diesel::*;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize,
    Deserialize,
    Debug
)]
pub enum Equipment {
    Computer(Computer),
    JumpPad(JumpPad),
    InterfaceBox(InterfaceBox),
    Monitor(Monitor),
    MicrophoneRecorder(MicrophoneRecorder),
    Projector(Projector),
    PowerStrip(PowerStrip),
    ExtensionCord(ExtensionCord)
}

// pub fn create(db: &mut database::Connection, item: &NewDivision) -> QueryResult<Division> {
//     use crate::schema::divisions::dsl::*;
//     insert_into(divisions).values(item).get_result::<Division>(db)
// }

// pub fn exists(db: &mut database::Connection, did: Uuid) -> bool {
//     use crate::schema::divisions::dsl::divisions;
//     divisions
//         .find(did)
//         .get_result::<Division>(db)
//         .is_ok()
// }

pub fn read(db: &mut database::Connection, equipment_id: i64) -> QueryResult<Equipment> {

    let equipment_dbo_result = 
        crate::schema::equipment::dsl::equipment
            .filter(crate::schema::equipment::dsl::id.eq(equipment_id))
            .first::<EquipmentDbo>(db);

    if equipment_dbo_result.is_err() {
        return Err(equipment_dbo_result.err().unwrap());
    }

    let equipment_dbo = equipment_dbo_result.unwrap();

    if let Some(computerid) = equipment_dbo.computerid {
        return match models::computer::read(db, computerid) {
            Ok(computer) => Ok(Equipment::Computer(computer)),
            Err(_) => {
                Err(diesel::result::Error::QueryBuilderError(
                format!("Error: Computer with ID {} does not exist", computerid).into()))
            }
        }
    } else if let Some(jumppadid) = equipment_dbo.jumppadid {
        return match models::jumppad::read(db, jumppadid) {
            Ok(jumppad) => Ok(Equipment::JumpPad(jumppad)),
            Err(_) => {
                Err(diesel::result::Error::QueryBuilderError(
                format!("Error: JumpPad with ID {} does not exist", jumppadid).into()))
            }
        }
    } else if let Some(interfaceboxid) = equipment_dbo.interfaceboxid {
        return match models::interfacebox::read(db, interfaceboxid) {
            Ok(interfacebox) => Ok(Equipment::InterfaceBox(interfacebox)),
            Err(_) => {
                Err(diesel::result::Error::QueryBuilderError(
                format!("Error: InterfaceBox with ID {} does not exist", interfaceboxid).into()))
            }
        }
    } else if let Some(monitorid) = equipment_dbo.monitorid {
        return match models::monitor::read(db, monitorid) {
            Ok(monitor) => Ok(Equipment::Monitor(monitor)),
            Err(_) => {
                Err(diesel::result::Error::QueryBuilderError(
                format!("Error: Monitor with ID {} does not exist", monitorid).into()))
            }
        }
    } else if let Some(microphonerecorderid) = equipment_dbo.microphonerecorderid {
        return match models::microphonerecorder::read(db, microphonerecorderid) {
            Ok(microphonerecorder) => Ok(Equipment::MicrophoneRecorder(microphonerecorder)),
            Err(_) => {
                Err(diesel::result::Error::QueryBuilderError(
                format!("Error: MicrophoneRecorder with ID {} does not exist", microphonerecorderid).into()))
            }
        }
    } else if let Some(projectorid) = equipment_dbo.projectorid {
        return match models::projector::read(db, projectorid) {
            Ok(projector) => Ok(Equipment::Projector(projector)),
            Err(_) => {
                Err(diesel::result::Error::QueryBuilderError(
                format!("Error: Projector with ID {} does not exist", projectorid).into()))
            }
        }
    } else if let Some(powerstripid) = equipment_dbo.powerstripid {
        return match models::powerstrip::read(db, powerstripid) {
            Ok(powerstrip) => Ok(Equipment::PowerStrip(powerstrip)),
            Err(_) => {
                Err(diesel::result::Error::QueryBuilderError(
                format!("Error: PowerStrip with ID {} does not exist", powerstripid).into()))
            }
        }
    } else if let Some(extensioncordid) = equipment_dbo.extensioncordid {
        return match models::extensioncord::read(db, extensioncordid) {
            Ok(extensioncord) => Ok(Equipment::ExtensionCord(extensioncord)),
            Err(_) => {
                Err(diesel::result::Error::QueryBuilderError(
                format!("Error: ExtensionCord with ID {} does not exist", extensioncordid).into()))
            }
        }
    };
    return Err(diesel::result::Error::QueryBuilderError(
        format!("Error: Could not find Equipment with ID {}", equipment_id).into()));
}

// pub fn read_all(db: &mut database::Connection, pagination: &PaginationParams) -> QueryResult<Vec<Division>> {
//     use crate::schema::divisions::dsl::*;
    
//     divisions
//         .order(created_at)
//         .limit(pagination.page_size)
//         .offset(
//             pagination.page
//                 * std::cmp::max(pagination.page_size, PaginationParams::MAX_PAGE_SIZE as i64),
//         )
//         .load::<Division>(db)
// }

// pub fn update(db: &mut database::Connection, item_id: Uuid, item: &DivisionChangeset) -> QueryResult<Division> {
//     use crate::schema::divisions::dsl::*;
//     diesel::update(divisions.filter(did.eq(item_id)))
//         .set((
//             item,
//             updated_at.eq(diesel::dsl::now),
//         ))
//         .get_result(db)
// }

// pub fn delete(db: &mut database::Connection, item_id: Uuid) -> QueryResult<usize> {
//     use crate::schema::divisions::dsl::*;
//     diesel::delete(divisions.filter(did.eq(item_id))).execute(db)
// }
