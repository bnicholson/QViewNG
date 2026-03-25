use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web::Data,
    HttpMessage,
};
use uuid::Uuid;

use crate::{
    auth::{policies::UserContext, verify_access_token},
    database::Database,
    models,
};

pub async fn add_user_context_to_extensions_from_access_token_middleware<B>(
    req: ServiceRequest,
    next: Next<B>,
) -> Result<ServiceResponse<B>, actix_web::Error>
{
    let token_option = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    if let Some(token) = token_option {
        if let Some(claims) = verify_access_token(token) {
            if let Some(db) = req.app_data::<Data<Database>>() {
                let mut conn = db.pool.get().unwrap();

                if let Ok(user_id) = Uuid::parse_str(&claims.sub) {
                    req.extensions_mut().insert(UserContext::new(user_id, claims.roles, claims.permissions));
                }
            }
        }
    }

    next.call(req).await
}
