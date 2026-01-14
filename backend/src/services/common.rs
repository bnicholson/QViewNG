
use std::any::type_name;

use diesel::QueryResult;
use serde::{Deserialize, Serialize};
use diesel::result::Error as DBError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityResponse<T> {
    pub code : i32,
    pub message: String,
    pub data : Option<T>,
}

pub fn process_response<T>(result : QueryResult<T>, http_method: &str) -> EntityResponse<T>
    where 
        T: std::fmt::Debug
{

    let mut response = EntityResponse::<T> {
        code : 200,
        message : "".to_string(),
        data : None,
    };

    match result {
        Ok(output) => {
            println!("Create {} (output)-> {:?}", type_name::<T>(), output);
            response.message = "".to_string();
            response.data = Some(output);
            
            match http_method.to_lowercase().as_str() {
                "post" => { response.code = 201; },
                _      => { response.code = 200; }
            }
        },
        Err(e) => {
            match e {
                DBError::DatabaseError(dbek,e) => {
                    match dbek {
                        diesel::result::DatabaseErrorKind::UniqueViolation => {
                            response.code = 409;
                            response.message = "Duplicate Tournament".to_string();
                            tracing::info!("{} {} create process_response-> {:?}", line!(), type_name::<T>(), e);
                        },
                        _ => {
                            response.code = 409;
                            response.message = format!("{:?}",e);
                            tracing::error!("{} {} create process_response-> {:?}", line!(), type_name::<T>(), e);
                        },
                    }
                },
                x => {
                    response.code = 409;
                    response.message = format!("{:?}",x);   
                    tracing::error!("{} {} create process_response-> {:?}", line!(), type_name::<T>(), x);     
                },
            }            
        }
    }    

    // return the result
    response
}