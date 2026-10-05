
use std::any::type_name;
use actix_web::{HttpMessage, HttpRequest};
use diesel::QueryResult;

use crate::auth::policies::UserContext;
use crate::models::permission::{AppAction, AppResource};
use serde::{Deserialize, Serialize};
use diesel::result::Error as DBError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PagedResponse<T> {
    pub count: i64,
    pub items: Vec<T>,
}

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
                            response.message = "Duplicate object".to_string();
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
                    response.code = 400;
                    response.message = format!("{:?}",x);   
                    tracing::error!("{} {} create process_response-> {:?}", line!(), type_name::<T>(), x);     
                },
            }            
        }
    }    

    response
}
/// Read gate shared by the entity read endpoints.
///
/// Unauthenticated callers keep the public read access they have always had — the tournament
/// finder and public tournament pages work without a login. An authenticated caller must hold
/// `{resource}:read`, so a token can be issued that deliberately withholds a section.
pub fn authorized_to_read(req: &HttpRequest, resource: AppResource) -> bool {
    match req.extensions().get::<UserContext>() {
        Some(user_ctx) => user_ctx.can(&format!("{}:{}", resource.as_str(), AppAction::Read.as_str())),
        None => true,
    }
}

/// Write gate for endpoints with no ownership (ABAC) policy of their own: the caller must be
/// authenticated and hold `{resource}:{action}`. Endpoints that *do* have a policy go through
/// `is_rbac_and_abac_authorized` instead.
pub fn authorized_to_write(req: &HttpRequest, resource: AppResource, action: AppAction) -> bool {
    match req.extensions().get::<UserContext>() {
        Some(user_ctx) => user_ctx.can(&format!("{}:{}", resource.as_str(), action.as_str())),
        None => false,
    }
}
