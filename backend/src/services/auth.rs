use actix_web::{cookie, delete, get, post, HttpRequest, HttpResponse, web::{Data, Json, Path, Query}};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};
use uuid::Uuid;
use crate::database::Database;
use crate::models;

const REFRESH_COOKIE: &str = "refresh_token";
const ACCESS_EXPIRY_HOURS: i64 = 1;
const REFRESH_EXPIRY_DAYS: i64 = 30;
const RESET_EXPIRY_HOURS: i64 = 24;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    roles: Vec<String>,
    permissions: Vec<PermissionClaim>,
}

#[derive(Serialize, Deserialize, Clone)]
struct PermissionClaim {
    permission: String,
}

fn jwt_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "qview_dev_secret_changeme".to_string())
}

fn make_access_token(user_id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = (Utc::now() + Duration::hours(ACCESS_EXPIRY_HOURS)).timestamp() as usize;
    encode(
        &Header::default(),
        &Claims { sub: user_id.to_string(), exp, roles: vec!["user".to_string()], permissions: vec![] },
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )
}

fn verify_access_token(token: &str) -> Option<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret().as_bytes()),
        &Validation::default(),
    ).ok().map(|d| d.claims)
}

fn make_refresh_cookie(token: String, days: i64) -> cookie::Cookie<'static> {
    cookie::Cookie::build(REFRESH_COOKIE, token)
        .http_only(true)
        .secure(false)
        .path("/api/auth")
        .max_age(cookie::time::Duration::days(days))
        .finish()
}

fn clear_refresh_cookie() -> cookie::Cookie<'static> {
    cookie::Cookie::build(REFRESH_COOKIE, "")
        .http_only(true)
        .path("/api/auth")
        .max_age(cookie::time::Duration::seconds(0))
        .finish()
}

#[derive(Deserialize)]
struct LoginRequest {
    #[serde(alias = "email")]
    identifier: String,
    password: String,
}

#[derive(Serialize)]
struct TokenResponse {
    access_token: String,
}

#[post("/login")]
async fn login(db: Data<Database>, Json(body): Json<LoginRequest>, req: HttpRequest) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();
    models::apicalllog::create(&mut conn, &req);

    let user = match models::user::find_by_email_or_username(&mut conn, &body.identifier) {
        Ok(u) => u,
        Err(_) => return HttpResponse::Unauthorized().json(serde_json::json!({"error": "Invalid credentials"})),
    };

    let hash = match &user.hash_password {
        Some(h) => h.clone(),
        None => return HttpResponse::Unauthorized().json(serde_json::json!({"error": "Incorrect password"})),
    };

    if !bcrypt::verify(&body.password, &hash).unwrap_or(false) {
        return HttpResponse::Unauthorized().json(serde_json::json!({"error": "Incorrect password"}));
    }

    if !user.activated {
        return HttpResponse::Unauthorized().json(serde_json::json!({"error": "Account not activated. Please check your email for the activation link."}));
    }

    let access_token = match make_access_token(user.id) {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let refresh_token = Uuid::new_v4().to_string();
    let device = req.headers().get("user-agent").and_then(|v| v.to_str().ok()).map(|s| s.to_string());
    if models::user_session::create(&mut conn, user.id, &refresh_token, device.as_deref()).is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok()
        .cookie(make_refresh_cookie(refresh_token, REFRESH_EXPIRY_DAYS))
        .json(TokenResponse { access_token })
}

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
    #[serde(default)]
    fname: String,
    #[serde(default)]
    mname: String,
    #[serde(default)]
    lname: String,
    #[serde(default)]
    username: Option<String>,
}

#[post("/register")]
async fn register(db: Data<Database>, Json(body): Json<RegisterRequest>, req: HttpRequest) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();
    models::apicalllog::create(&mut conn, &req);

    // Check if email already exists
    if models::user::find_by_email_or_username(&mut conn, &body.email).is_ok() {
        return HttpResponse::Conflict().json(serde_json::json!({"error": "Email already in use"}));
    }

    let fname = if body.fname.is_empty() { "New".to_string() } else { body.fname.clone() };
    let lname = if body.lname.is_empty() { "User".to_string() } else { body.lname.clone() };

    let mut builder = crate::models::user::UserBuilder::new(&fname)
        .set_email(&body.email)
        .set_lname(&lname)
        .set_mname(&body.mname)
        .set_hash_password(&body.password)
        .set_activated(false);
    if let Some(ref uname) = body.username {
        if !uname.is_empty() {
            builder = builder.set_username(uname);
        }
    }
    let new_user = match builder.build_and_insert(&mut conn) {
        Ok(u) => u,
        Err(e) => {
            tracing::error!("Failed to create user: {:?}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({"error": "Failed to create user"}));
        }
    };

    // Generate activation token (24 hour expiry)
    let token = Uuid::new_v4().to_string();
    let expires_at = Utc::now() + Duration::hours(24);
    if models::activation_token::create(&mut conn, new_user.id, &token, expires_at).is_err() {
        return HttpResponse::InternalServerError().json(serde_json::json!({"error": "Failed to create activation token"}));
    }

    let frontend_url = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
    let activation_link = format!("{}/activate?token={}", frontend_url, token);
    let _ = send_activation_email(&new_user.email, &activation_link).await;
    tracing::info!("Activation link for {}: {}", new_user.email, activation_link);

    HttpResponse::Created().json(serde_json::json!({"id": new_user.id, "email": new_user.email}))
}

#[post("/logout")]
async fn logout(req: HttpRequest, db: Data<Database>) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();
    models::apicalllog::create(&mut conn, &req);

    if let Some(cookie) = req.cookie(REFRESH_COOKIE) {
        let _ = models::user_session::delete_by_token(&mut conn, cookie.value());
    }

    HttpResponse::Ok().cookie(clear_refresh_cookie()).finish()
}

#[post("/refresh")]
async fn refresh(req: HttpRequest, db: Data<Database>) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();
    models::apicalllog::create(&mut conn, &req);

    let refresh_token = match req.cookie(REFRESH_COOKIE) {
        Some(c) => c.value().to_string(),
        None => return HttpResponse::Unauthorized().json(serde_json::json!({"error": "No refresh token"})),
    };

    let session = match models::user_session::find_by_token(&mut conn, &refresh_token) {
        Ok(s) => s,
        Err(_) => return HttpResponse::Unauthorized().json(serde_json::json!({"error": "Invalid session"})),
    };

    let access_token = match make_access_token(session.user_id) {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    HttpResponse::Ok().json(TokenResponse { access_token })
}

#[derive(Deserialize)]
struct ForgotRequest {
    email: String,
}

#[post("/forgot")]
async fn forgot_password(db: Data<Database>, Json(body): Json<ForgotRequest>, req: HttpRequest) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();
    models::apicalllog::create(&mut conn, &req);

    // Always return 200 to avoid user enumeration
    let user = match models::user::find_by_email_or_username(&mut conn, &body.email) {
        Ok(u) => u,
        Err(_) => return HttpResponse::Ok().json(serde_json::json!({"message": "If that email exists, a recovery link has been sent."})),
    };

    let token = Uuid::new_v4().to_string();
    let expires_at = Utc::now() + Duration::hours(RESET_EXPIRY_HOURS);

    if models::password_reset::create(&mut conn, user.id, &token, expires_at).is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let frontend_url = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
    let reset_link = format!("{}/reset?token={}", frontend_url, token);

    // Try to send email; if SMTP is not configured, log the link
    let _ = send_reset_email(&user.email, &reset_link).await;
    tracing::info!("Password reset link for {}: {}", user.email, reset_link);

    HttpResponse::Ok().json(serde_json::json!({"message": "If that email exists, a recovery link has been sent."}))
}

async fn send_reset_email(to_email: &str, reset_link: &str) -> Result<(), Box<dyn std::error::Error>> {
    use lettre::{Message, SmtpTransport, Transport};
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::message::header::ContentType;

    let smtp_host = std::env::var("SMTP_HOST")?;
    let smtp_user = std::env::var("SMTP_USER")?;
    let smtp_pass = std::env::var("SMTP_PASS")?;
    let smtp_from = std::env::var("SMTP_FROM").unwrap_or_else(|_| smtp_user.clone());

    let email = Message::builder()
        .from(smtp_from.parse()?)
        .to(to_email.parse()?)
        .subject("QView Password Reset")
        .header(ContentType::TEXT_PLAIN)
        .body(format!("Click this link to reset your password (expires in 24 hours):\n\n{}", reset_link))?;

    let creds = Credentials::new(smtp_user, smtp_pass);
    let mailer = SmtpTransport::relay(&smtp_host)?.credentials(creds).build();
    mailer.send(&email)?;
    Ok(())
}

async fn send_activation_email(to_email: &str, activation_link: &str) -> Result<(), Box<dyn std::error::Error>> {
    use lettre::{Message, SmtpTransport, Transport};
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::message::header::ContentType;

    let smtp_host = std::env::var("SMTP_HOST")?;
    let smtp_user = std::env::var("SMTP_USER")?;
    let smtp_pass = std::env::var("SMTP_PASS")?;
    let smtp_from = std::env::var("SMTP_FROM").unwrap_or_else(|_| smtp_user.clone());

    let email = Message::builder()
        .from(smtp_from.parse()?)
        .to(to_email.parse()?)
        .subject("Activate your QView account")
        .header(ContentType::TEXT_PLAIN)
        .body(format!("Click this link to activate your account (expires in 24 hours):\n\n{}", activation_link))?;

    let creds = Credentials::new(smtp_user, smtp_pass);
    let mailer = SmtpTransport::relay(&smtp_host)?.credentials(creds).build();
    mailer.send(&email)?;
    Ok(())
}

#[derive(Deserialize)]
struct ResetRequest {
    reset_token: String,
    new_password: String,
}

#[post("/reset")]
async fn reset_password(db: Data<Database>, Json(body): Json<ResetRequest>, req: HttpRequest) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();
    models::apicalllog::create(&mut conn, &req);

    let reset_token = match models::password_reset::find_valid(&mut conn, &body.reset_token) {
        Ok(t) => t,
        Err(_) => return HttpResponse::BadRequest().json(serde_json::json!({"error": "Invalid or expired token"})),
    };

    let hashed = match bcrypt::hash(&body.new_password, bcrypt::DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    if models::user::change_password(&mut conn, reset_token.user_id, &hashed).is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let _ = models::password_reset::mark_used(&mut conn, &body.reset_token);

    HttpResponse::Ok().json(serde_json::json!({"message": "Password reset successfully"}))
}

#[derive(Deserialize)]
struct ActivateQuery {
    activation_token: String,
}

#[get("/activate")]
async fn activate(db: Data<Database>, query: Query<ActivateQuery>, req: HttpRequest) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();
    models::apicalllog::create(&mut conn, &req);

    let record = match models::activation_token::find_valid(&mut conn, &query.activation_token) {
        Ok(r) => r,
        Err(_) => return HttpResponse::BadRequest().json(serde_json::json!({"error": "Invalid or expired activation token"})),
    };

    if models::user::activate_user(&mut conn, record.user_id).is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let _ = models::activation_token::mark_used(&mut conn, &query.activation_token);

    HttpResponse::Ok().json(serde_json::json!({"message": "Account activated successfully"}))
}

#[derive(Deserialize)]
struct ChangePasswordRequest {
    old_password: String,
    new_password: String,
}

fn extract_bearer_token(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.trim_start_matches("Bearer ").trim().to_string())
}

#[post("/change")]
async fn change_password(db: Data<Database>, Json(body): Json<ChangePasswordRequest>, req: HttpRequest) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();
    models::apicalllog::create(&mut conn, &req);

    let token = match extract_bearer_token(&req) {
        Some(t) => t,
        None => return HttpResponse::Unauthorized().json(serde_json::json!({"error": "Missing authorization"})),
    };

    let claims = match verify_access_token(&token) {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().json(serde_json::json!({"error": "Invalid token"})),
    };

    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    let user = match models::user::read(&mut conn, user_id) {
        Ok(u) => u,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    let hash = match &user.hash_password {
        Some(h) => h.clone(),
        None => return HttpResponse::BadRequest().json(serde_json::json!({"error": "No password set"})),
    };

    if !bcrypt::verify(&body.old_password, &hash).unwrap_or(false) {
        return HttpResponse::Unauthorized().json(serde_json::json!({"error": "Current password is incorrect"}));
    }

    let new_hash = match bcrypt::hash(&body.new_password, bcrypt::DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match models::user::change_password(&mut conn, user_id, &new_hash) {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"message": "Password changed successfully"})),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Deserialize)]
struct SessionsQuery {
    #[serde(default)]
    page: i64,
    #[serde(default = "default_page_size")]
    page_size: i64,
}

fn default_page_size() -> i64 { 10 }

#[derive(Serialize)]
struct SessionsResponse {
    sessions: Vec<models::user_session::UserSession>,
    num_pages: i64,
}

#[get("/sessions")]
async fn list_sessions(db: Data<Database>, query: Query<SessionsQuery>, req: HttpRequest) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();
    models::apicalllog::create(&mut conn, &req);

    let token = match extract_bearer_token(&req) {
        Some(t) => t,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let claims = match verify_access_token(&token) {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    let sessions = match models::user_session::read_all_for_user(&mut conn, user_id, query.page, query.page_size) {
        Ok(s) => s,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let total = models::user_session::count_for_user(&mut conn, user_id).unwrap_or(0);
    let num_pages = ((total as f64) / (query.page_size as f64)).ceil() as i64;

    HttpResponse::Ok().json(SessionsResponse { sessions, num_pages: num_pages.max(1) })
}

#[delete("/sessions/{session_id}")]
async fn delete_session(db: Data<Database>, path: Path<i64>, req: HttpRequest) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();
    models::apicalllog::create(&mut conn, &req);

    let token = match extract_bearer_token(&req) {
        Some(t) => t,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let claims = match verify_access_token(&token) {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    match models::user_session::delete_by_id(&mut conn, path.into_inner(), user_id) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[delete("/sessions")]
async fn delete_all_sessions(db: Data<Database>, req: HttpRequest) -> HttpResponse {
    let mut conn = db.pool.get().unwrap();
    models::apicalllog::create(&mut conn, &req);

    let token = match extract_bearer_token(&req) {
        Some(t) => t,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let claims = match verify_access_token(&token) {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    match models::user_session::delete_all_for_user(&mut conn, user_id) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    scope
        .service(login)
        .service(register)
        .service(logout)
        .service(refresh)
        .service(forgot_password)
        .service(reset_password)
        .service(activate)
        .service(change_password)
        .service(list_sessions)
        .service(delete_session)
        .service(delete_all_sessions)
}
