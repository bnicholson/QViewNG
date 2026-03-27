pub mod policies;

use actix_web::cookie;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};
use uuid::Uuid;

use crate::{auth::policies::{Policy, PolicyContext}, errors::AppError, models::{permission::Permission, role::AppRole}};

pub(crate) const REFRESH_COOKIE: &str = "refresh_token";
pub(crate) const ACCESS_EXPIRY_HOURS: i64 = 1;
pub(crate) const REFRESH_EXPIRY_DAYS: i64 = 30;
pub(crate) const RESET_EXPIRY_HOURS: i64 = 24;

#[derive(Serialize, Deserialize)]
pub(crate) struct Claims {
    pub sub: String,
    pub exp: usize,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

#[derive(Deserialize)]
pub(crate) struct LoginRequest {
    #[serde(alias = "email")]
    pub identifier: String,
    pub password: String,
    #[serde(default)]
    pub screen_width: Option<u32>,
    #[serde(default)]
    pub screen_height: Option<u32>,
}

#[derive(Serialize)]
pub(crate) struct TokenResponse {
    pub access_token: String,
}

pub(crate) fn jwt_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "qview_dev_secret_changeme".to_string())
}

pub(crate) fn make_access_token(user_id: Uuid, db: &mut crate::database::Connection) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = (Utc::now() + Duration::hours(ACCESS_EXPIRY_HOURS)).timestamp() as usize;

    // Load the user's assigned roles from the DB.
    let user_roles = crate::models::users_roles::read_all_for_user(db, user_id)
        .unwrap_or_default();

    let role_ids: Vec<i32> = user_roles.iter().map(|ur| ur.role_id).collect();

    // Resolve role names.
    let roles: Vec<String> = role_ids.iter()
        .filter_map(|&rid| crate::models::role::read(db, rid).ok())
        .map(|r| r.name)
        .collect();

    // Resolve permissions via roles_permissions join.
    let role_permissions: Vec<crate::models::role_permission::RolePermission> = role_ids.iter()
        .flat_map(|&rid| {
            crate::models::role_permission::read_all_for_role(db, rid)
                .unwrap_or_default()
        })
        .collect();
    let permissions: Vec<String> = {
        let mut seen = std::collections::HashSet::new();
        role_permissions.iter()
            .filter_map(|rp| crate::models::permission::read(db, rp.permission_id).ok())
            .map(|p| p.name)
            .filter(|name| seen.insert(name.clone()))
            .collect()
    };

    encode(
        &Header::default(),
        &Claims { sub: user_id.to_string(), exp, roles, permissions },
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )
}

pub(crate) fn verify_access_token(token: &str) -> Option<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret().as_bytes()),
        &Validation::default(),
    ).ok().map(|d| d.claims)
}

pub(crate) fn make_refresh_cookie(token: String, days: i64) -> cookie::Cookie<'static> {
    cookie::Cookie::build(REFRESH_COOKIE, token)
        .http_only(true)
        .secure(false)
        .path("/api/auth")
        .max_age(cookie::time::Duration::days(days))
        .finish()
}

pub(crate) fn clear_refresh_cookie() -> cookie::Cookie<'static> {
    cookie::Cookie::build(REFRESH_COOKIE, "")
        .http_only(true)
        .path("/api/auth")
        .max_age(cookie::time::Duration::seconds(0))
        .finish()
}

pub(crate) fn is_abac_authorized<R>(ctx: &PolicyContext<R>, permission: &str, resource_name: &str) -> Result<(), AppError>
where
    PolicyContext<R>: Policy<R>
{
    // Super users bypass all authorization checks.
    if ctx.user.roles.iter().any(|r| r == AppRole::SuperUser.as_str()) {
        return Ok(());
    }

    // RBAC gate
    if !ctx.user.permissions.contains(&permission.to_string()) {
        return Err(AppError::Forbidden);
    }

    // ABAC gate (only reached if RBAC passes)
    let allowed = match permission {
        name if name == format!("{}:edit", resource_name)   => ctx.can_edit(&ctx.resource),
        name if name == format!("{}:delete", resource_name) => ctx.can_delete(&ctx.resource),
        _ => true, // no policy defined, RBAC alone is sufficient
    };

    if !allowed {
        return Err(AppError::Forbidden);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::policies::{Policy, PolicyContext, UserContext};
    use uuid::Uuid;

    struct DummyResource;

    impl Policy<DummyResource> for PolicyContext<DummyResource> {
        fn can_edit(&self, _: &DummyResource) -> bool { false }
        fn can_delete(&self, _: &DummyResource) -> bool { false }
        fn can_view(&self, _: &DummyResource) -> bool { false }
    }

    fn make_ctx(roles: Vec<&str>, permissions: Vec<&str>) -> PolicyContext<DummyResource> {
        PolicyContext {
            user: UserContext::new(
                Uuid::new_v4(),
                roles.into_iter().map(str::to_string).collect(),
                permissions.into_iter().map(str::to_string).collect(),
            ),
            resource: DummyResource,
        }
    }

    #[test]
    fn super_user_bypasses_rbac_and_abac() {
        let ctx = make_ctx(vec![AppRole::SuperUser.as_str()], vec![]);
        // No permissions in the token, yet every check must pass.
        assert!(is_abac_authorized(&ctx, "tournament:create", "tournament").is_ok());
        assert!(is_abac_authorized(&ctx, "tournament:edit",   "tournament").is_ok());
        assert!(is_abac_authorized(&ctx, "tournament:delete", "tournament").is_ok());
        assert!(is_abac_authorized(&ctx, "user:delete",       "user").is_ok());
        assert!(is_abac_authorized(&ctx, "anything:unknown",  "anything").is_ok());
    }

    #[test]
    fn non_super_user_without_permission_is_forbidden() {
        let ctx = make_ctx(vec!["member"], vec![]);
        assert!(is_abac_authorized(&ctx, "tournament:create", "tournament").is_err());
    }

    #[test]
    fn non_super_user_with_permission_is_allowed() {
        let ctx = make_ctx(vec!["tournament_manager"], vec!["tournament:create"]);
        assert!(is_abac_authorized(&ctx, "tournament:create", "tournament").is_ok());
    }
}
