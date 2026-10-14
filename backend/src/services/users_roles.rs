use actix_web::{delete, get, post, put, Error, HttpMessage, HttpRequest, HttpResponse, Result,
    web::{Data, Json, Path}};
use crate::{
    auth::policies::UserContext,
    database::Database,
    models::{self, role::AppRole, users_roles::NewUsersRole},
};
use crate::services::common::{EntityResponse, process_response};
use diesel::QueryResult;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// A user's complete set of roles, by id — the body of PUT and the response of GET/PUT.
#[derive(Deserialize, Serialize, ToSchema)]
pub struct UserRoleIds {
    pub role_ids: Vec<Uuid>,
}

/// Role assignment is Superuser-only (`system:administer`): 401 without a login, 403 without it.
fn require_superuser(req: &HttpRequest) -> Result<UserContext, HttpResponse> {
    match req.extensions().get::<UserContext>() {
        None => Err(HttpResponse::Unauthorized().finish()),
        Some(ctx) if !ctx.is_system_administrator() => Err(HttpResponse::Forbidden().finish()),
        Some(ctx) => Ok(ctx.clone()),
    }
}

/// GET /usersroles/users/{user_id} — the ids of the roles assigned to a user
#[get("/users/{user_id}")]
async fn read_for_user(
    db: Data<Database>,
    user_id: Path<Uuid>,
    req: HttpRequest,
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();
    models::apicalllog::create(&mut db, &req);
    if let Err(resp) = require_superuser(&req) {
        return resp;
    }
    match models::users_roles::read_all_for_user(&mut db, user_id.into_inner()) {
        Ok(records) => HttpResponse::Ok().json(UserRoleIds { role_ids: records.iter().map(|r| r.role_id).collect() }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// PUT /usersroles/users/{user_id} — replace a user's roles with exactly `role_ids`
#[put("/users/{user_id}")]
async fn replace_for_user(
    db: Data<Database>,
    user_id: Path<Uuid>,
    Json(body): Json<UserRoleIds>,
    req: HttpRequest,
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();
    models::apicalllog::create(&mut db, &req);
    let caller = match require_superuser(&req) {
        Ok(ctx) => ctx,
        Err(resp) => return resp,
    };
    let user_id = user_id.into_inner();

    if !models::user::exists(&mut db, user_id) {
        return HttpResponse::NotFound().json(serde_json::json!({"error": "User not found"}));
    }
    let mut role_ids = body.role_ids;
    role_ids.sort();
    role_ids.dedup();
    if let Some(missing) = role_ids.iter().find(|&&rid| !models::role::exists(&mut db, rid)) {
        return HttpResponse::BadRequest().json(serde_json::json!({"error": format!("Role {missing} does not exist")}));
    }
    // A Superuser can't take the Super User role from themselves, so the console can't lock its
    // own user out.
    if user_id == caller.user_id {
        if let Ok(super_user) = models::role::read_by_name(&mut db, AppRole::SuperUser.as_str()) {
            if !role_ids.contains(&super_user.id) {
                return HttpResponse::BadRequest().json(serde_json::json!({"error": "You can't remove the Super User role from yourself"}));
            }
        }
    }

    match models::users_roles::replace_for_user(&mut db, user_id, &role_ids) {
        Ok(records) => HttpResponse::Ok().json(UserRoleIds { role_ids: records.iter().map(|r| r.role_id).collect() }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// /// GET /usersroles/roles/{role_id} — list all users assigned to a role
// #[get("/roles/{role_id}")]
// async fn read_for_role(
//     db: Data<Database>,
//     role_id: Path<Uuid>,
//     req: HttpRequest,
// ) -> HttpResponse {
//     let mut db = db.pool.get().unwrap();
//     models::apicalllog::create(&mut db, &req);
//     match models::users_roles::read_all_for_role(&mut db, role_id.into_inner()) {
//         Ok(records) => HttpResponse::Ok().json(records),
//         Err(_) => HttpResponse::NotFound().finish(),
//     }
// }

// /// POST /usersroles/users/{user_id}/roles/{role_id} — assign a role to a user
// #[post("/users/{user_id}/roles/{role_id}")]
// async fn assign(
//     db: Data<Database>,
//     path: Path<(Uuid, Uuid)>,
//     req: HttpRequest,
// ) -> Result<HttpResponse, Error> {
//     let mut db = db.get_connection().expect("Failed to get connection");
//     models::apicalllog::create(&mut db, &req);
//     let (user_id, role_id) = path.into_inner();
//     let result: QueryResult<_> = models::users_roles::create(
//         &mut db,
//         NewUsersRole { user_id, role_id },
//     );
//     let response: EntityResponse<_> = process_response(result, "post");
//     match response.code {
//         409 => Ok(HttpResponse::Conflict().json(response)),
//         201 => Ok(HttpResponse::Created().json(response)),
//         200 => Ok(HttpResponse::Ok().json(response)),
//         _ => Ok(HttpResponse::InternalServerError().json(response)),
//     }
// }

/// DELETE /usersroles/users/{user_id}/roles/{role_id} — revoke a role from a user
#[delete("/users/{user_id}/roles/{role_id}")]
async fn revoke(
    db: Data<Database>,
    path: Path<(Uuid, Uuid)>,
    req: HttpRequest,
) -> HttpResponse {
    let mut db = db.pool.get().unwrap();
    models::apicalllog::create(&mut db, &req);
    let (user_id, role_id) = path.into_inner();
    if models::users_roles::delete(&mut db, user_id, role_id).is_ok() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

/// DELETE /usersroles/users/{user_id} — revoke all roles from a user
// #[delete("/users/{user_id}")]
// async fn revoke_all(
//     db: Data<Database>,
//     user_id: Path<Uuid>,
//     req: HttpRequest,
// ) -> HttpResponse {
//     let mut db = db.pool.get().unwrap();
//     models::apicalllog::create(&mut db, &req);
//     if models::users_roles::delete_all_for_user(&mut db, user_id.into_inner()).is_ok() {
//         HttpResponse::Ok().finish()
//     } else {
//         HttpResponse::InternalServerError().finish()
//     }
// }

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    scope
        .service(read_for_user)
        .service(replace_for_user)
        // .service(read_for_role)
        // .service(assign)
        // .service(revoke_all)
        .service(revoke)
}
