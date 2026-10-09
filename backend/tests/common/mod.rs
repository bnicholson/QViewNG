use backend::database::Database;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;
// use pgtemp::PgTempDB;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub const TEST_DB_URL: &str = "TEST_DATABASE_URL";

pub const PAGE_NUM: i64 = 0;
pub const PAGE_SIZE: i64 = 10;

pub fn establish_test_connection() -> PgConnection {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must be set");
    
    let mut conn = PgConnection::establish(&database_url)
        .expect("Failed to connect to test database");
    
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
    
    conn
}

/// Mint a signed JWT using the default dev secret, with the given roles and
/// permissions embedded in the claims. The token is valid for 1 hour.
/// Use this to supply an `Authorization: Bearer <token>` header in tests that
/// hit auth-gated endpoints.
pub fn make_token(user_id: Uuid, roles: Vec<String>, permissions: Vec<String>) -> String {
    let exp = (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize;
    let claims = serde_json::json!({
        "sub": user_id.to_string(),
        "exp": exp,
        "roles": roles,
        "permissions": permissions,
    });
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(b"qview_dev_secret_changeme"),
    ).expect("Failed to encode test JWT")
}

pub fn clean_database() {
    establish_test_connection();  // mostly for running pending migrations
    let db = Database::new(TEST_DB_URL);
    let mut conn = db.get_connection().expect("Failed to get connection.");

    backend::database::clean_db::clean_database(&mut conn);
}

// pub fn prepare_database() -> {

//     let db = PgTempDB::async_new().await;

//     let url = db.connection_uri();
    
//     let mut conn = PgConnection::establish(&url)
//         .expect("Failed to connect to test database");
    
//     conn.run_pending_migrations(MIGRATIONS)
//         .expect("Failed to run migrations");

//     db
// }

// ─── Signing game events the way a QuizMachine client does ────────────────────

/// The pre-shared key the tests sign game events with.
pub const TEST_GAMEEVENT_PSK: &str = "test-gameevent-psk";

/// Points the server at `TEST_GAMEEVENT_PSK`. The server reads `GAMEEVENT_PSK` per request, so this
/// must run before any signed request is sent.
pub fn use_test_gameevent_psk() {
    // SAFETY: edition 2024 marks set_var unsafe because another thread could read the environment
    // concurrently. Integration tests run with --test-threads=1, and this only ever writes the
    // same value.
    unsafe { std::env::set_var("GAMEEVENT_PSK", TEST_GAMEEVENT_PSK) };
}

/// One game event's fields, as a QuizMachine client would send them.
#[derive(Clone)]
pub struct ClientGameEvent {
    pub gid: String,
    pub tournament: String,
    pub division: String,
    pub room: String,
    pub round: String,
    pub question: i32,
    pub eventnum: i32,
    pub name: String,
    pub team: i32,
    pub quizzer: i32,
    pub event: String,
    pub parm1: String,
    pub parm2: String,
    pub nonce: String,
}

/// Percent-encodes a query value (RFC 3986 unreserved characters kept). The server's query
/// parser decodes it back, which keeps the `+`, `/` and `=` of a base64 checksum intact.
pub fn query_encode(s: &str) -> String {
    let mut out = String::new();
    for &b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => out.push(b as char),
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

const TEST_BLDGROOM: &str = "bldg";
const TEST_CLIENTKEY: &str = "test-client-key";
const TEST_TK: &str = "tk1";

/// The `/api/gameevents/create` URI for `ev`, signed with `TEST_GAMEEVENT_PSK` exactly as the
/// server verifies it: sha3-512 over the nonce, the key, then the fields in a fixed order.
pub fn signed_gameevent_uri(ev: &ClientGameEvent) -> String {
    gameevent_uri_with_checksum(ev, &gameevent_checksum(ev))
}

/// The same request with a checksum the caller chooses — e.g. a wrong one.
pub fn gameevent_uri_with_checksum(ev: &ClientGameEvent, s3s: &str) -> String {
    let pairs: Vec<(&str, String)> = vec![
        ("bldgroom", TEST_BLDGROOM.to_string()),
        ("key", TEST_CLIENTKEY.to_string()),
        ("gid", ev.gid.clone()),
        ("tk", TEST_TK.to_string()),
        ("tn", ev.tournament.clone()),
        ("dn", ev.division.clone()),
        ("rm", ev.room.clone()),
        ("rd", ev.round.clone()),
        ("qn", ev.question.to_string()),
        ("e", ev.eventnum.to_string()),
        ("n", ev.name.clone()),
        ("t", ev.team.to_string()),
        ("q", ev.quizzer.to_string()),
        ("ec", ev.event.clone()),
        ("p1", ev.parm1.clone()),
        ("p2", ev.parm2.clone()),
        ("ts", "1786991939".to_string()),
        ("md5", "0123456789abcdef0123456789abcdef".to_string()),
        ("nonce", ev.nonce.clone()),
        ("s3s", s3s.to_string()),
    ];
    let query: Vec<String> = pairs.iter().map(|(k, v)| format!("{}={}", k, query_encode(v))).collect();
    format!("/api/gameevents/create?{}", query.join("&"))
}

pub fn gameevent_checksum(ev: &ClientGameEvent) -> String {
    use base64::Engine;
    use sha3::{Digest, Sha3_512};
    let mut h = Sha3_512::new();
    for part in [
        ev.nonce.as_str(), TEST_GAMEEVENT_PSK, TEST_BLDGROOM, TEST_CLIENTKEY, TEST_TK,
        &ev.tournament, &ev.division, &ev.room, &ev.round,
        &ev.question.to_string(), &ev.eventnum.to_string(), &ev.name,
        &ev.team.to_string(), &ev.quizzer.to_string(), &ev.event, &ev.parm1, &ev.parm2,
    ] {
        h.update(part);
    }
    base64::engine::general_purpose::STANDARD.encode(h.finalize())
}

/// The `sha3512sum` header the server returns when it acknowledges an event with this nonce.
pub fn expected_ack_header(nonce: &str) -> String {
    use base64::Engine;
    use sha3::{Digest, Sha3_512};
    let mut h = Sha3_512::new();
    h.update(nonce);
    h.update(TEST_GAMEEVENT_PSK);
    base64::engine::general_purpose::STANDARD.encode(h.finalize())
}
