use backend::database::Database;
use diesel::prelude::*;
use diesel::sql_query;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::sync::{Arc, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub const PAGE_NUM: i64 = 0;
pub const PAGE_SIZE: i64 = 10;

// ─── A fresh database for every test ──────────────────────────────────────────
//
// Every test gets its own database, copied from a template that already has the schema, so tests
// can't see each other's rows and can run in parallel. The template is built once (per migration
// set) on the server `TEST_DATABASE_URL` points at — the RAM-backed Postgres from
// compose.test.yaml — and copying it takes ~35 ms, against ~0.1–1.3 s for the old wipe-everything
// approach. Each test database is dropped when the test's last `Database` handle goes away.

/// Pool size for a test database. Connections open lazily (see `Database::from_url`), so this is
/// only a ceiling.
const TEST_POOL_SIZE: u32 = 10;

/// Postgres advisory-lock key that serialises template builds across test processes running at
/// the same time (cargo-nextest runs each test in its own process).
const TEMPLATE_LOCK_KEY: i64 = 0x7176_6965_775f_7470; // "qview_tp"

/// Test databases are named `qtest_<unix seconds>_<random>`. Any older than this belong to a run
/// that crashed before cleaning up, and are swept the next time a template is checked.
const STALE_TEST_DB_SECS: u64 = 60 * 60;

static TEMPLATE: OnceLock<String> = OnceLock::new();

/// A fresh, empty, fully-migrated database for one test. Hand clones of it (`db.clone()`) to
/// anything else in the test that needs the same data, such as the actix app.
pub fn test_db() -> Database {
    let template = TEMPLATE.get_or_init(ensure_template);
    let name = format!("qtest_{}_{}", unix_now(), &Uuid::new_v4().simple().to_string()[..12]);

    let mut admin = admin_connection();
    sql_query(format!(r#"CREATE DATABASE "{name}" TEMPLATE "{template}""#))
        .execute(&mut admin)
        .unwrap_or_else(|e| panic!("Failed to create test database {name}: {e}"));

    Database::from_url(&url_for_database(&name), TEST_POOL_SIZE)
        .with_keepalive(Arc::new(DropTestDatabase { name }))
}

/// Empties every table of the test's database mid-test, for tests that arrange a second scenario
/// from scratch partway through. (A new test needs nothing like this — `test_db` starts empty.)
pub fn reset_test_db(conn: &mut backend::database::Connection) {
    backend::database::clean_db::clean_database(conn);
}

/// Drops its test database once the last `Database` clone holding it is gone. `WITH (FORCE)`
/// closes any connection still open, e.g. one a test leaked out of the pool.
struct DropTestDatabase {
    name: String,
}

impl Drop for DropTestDatabase {
    fn drop(&mut self) {
        // Never panic here: this also runs while a failed test is unwinding. A database this fails
        // to drop is swept later (see `sweep_stale_databases`), so a warning is enough.
        let result = PgConnection::establish(&url_for_database("postgres"))
            .map_err(|e| e.to_string())
            .and_then(|mut admin| {
                sql_query(format!(r#"DROP DATABASE IF EXISTS "{}" WITH (FORCE)"#, self.name))
                    .execute(&mut admin)
                    .map_err(|e| e.to_string())
            });
        if let Err(e) = result {
            eprintln!("warning: couldn't drop test database {}: {e}", self.name);
        }
    }
}

/// Makes sure the template for the current migrations exists, building it if not, and returns its
/// name. Runs once per test process; the advisory lock makes concurrent processes take turns.
fn ensure_template() -> String {
    let template = format!("qview_tpl_{}", migrations_fingerprint());
    let mut admin = admin_connection();

    sql_query(format!("SELECT pg_advisory_lock({TEMPLATE_LOCK_KEY})"))
        .execute(&mut admin)
        .expect("Failed to take the template lock");

    if !database_exists(&mut admin, &template) {
        build_template(&mut admin, &template);
    }
    sweep_stale_databases(&mut admin, &template);

    sql_query(format!("SELECT pg_advisory_unlock({TEMPLATE_LOCK_KEY})"))
        .execute(&mut admin)
        .expect("Failed to release the template lock");

    template
}

/// Builds the template under a temporary name and renames it into place only once it's complete,
/// so a build that dies halfway can never leave a half-migrated template behind.
fn build_template(admin: &mut PgConnection, template: &str) {
    let building = format!("{template}_building");
    exec(admin, &format!(r#"DROP DATABASE IF EXISTS "{building}" WITH (FORCE)"#));
    exec(admin, &format!(r#"CREATE DATABASE "{building}""#));

    {
        let db = Database::from_url(&url_for_database(&building), 1);
        let mut conn = db.get_connection().expect("Failed to connect to the template being built");
        conn.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations on the test template");
        // Some migrations insert rows (e.g. the default permissions). Tests have always started
        // from completely empty tables, so empty them here once rather than in every test.
        backend::database::clean_db::clean_database(&mut conn);
    } // pool dropped: its connections close, which the rename below requires

    exec(admin, &format!(r#"ALTER DATABASE "{building}" RENAME TO "{template}""#));
    // A template nobody can connect to can't be modified by accident, and copying it fails if
    // anyone is connected — so make that impossible.
    exec(admin, &format!(r#"ALTER DATABASE "{template}" WITH IS_TEMPLATE true ALLOW_CONNECTIONS false"#));
}

/// Drops templates built from older migration sets, and test databases left by crashed runs.
fn sweep_stale_databases(admin: &mut PgConnection, current_template: &str) {
    #[derive(QueryableByName)]
    struct Name {
        #[diesel(sql_type = diesel::sql_types::Text)]
        datname: String,
    }
    let names: Vec<Name> = sql_query(
        r"SELECT datname::text AS datname FROM pg_database WHERE datname LIKE 'qview\_tpl\_%' OR datname LIKE 'qtest\_%'",
    )
    .load(admin)
    .expect("Failed to list test databases");

    let now = unix_now();
    for Name { datname } in names {
        if datname.starts_with("qview_tpl_") {
            if datname != current_template {
                // A template must be demoted before it can be dropped.
                exec(admin, &format!(r#"ALTER DATABASE "{datname}" WITH IS_TEMPLATE false"#));
                exec(admin, &format!(r#"DROP DATABASE IF EXISTS "{datname}" WITH (FORCE)"#));
            }
        } else if let Some(created) = datname.split('_').nth(1).and_then(|s| s.parse::<u64>().ok()) {
            if now.saturating_sub(created) > STALE_TEST_DB_SECS {
                exec(admin, &format!(r#"DROP DATABASE IF EXISTS "{datname}" WITH (FORCE)"#));
            }
        }
    }
}

/// A short hash of every migration file, so editing, adding or removing any migration builds a
/// new template instead of reusing a stale one.
fn migrations_fingerprint() -> String {
    use sha3::{Digest, Sha3_256};
    fn collect(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        for entry in std::fs::read_dir(dir).expect("Failed to read the migrations directory") {
            let path = entry.expect("Failed to read a migrations entry").path();
            if path.is_dir() { collect(&path, out) } else { out.push(path) }
        }
    }
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    let mut files = Vec::new();
    collect(&root, &mut files);
    files.sort();

    let mut hasher = Sha3_256::new();
    for path in files {
        hasher.update(path.strip_prefix(&root).unwrap().to_string_lossy().as_bytes());
        hasher.update(std::fs::read(&path).expect("Failed to read a migration file"));
    }
    hasher.finalize().iter().take(6).map(|b| format!("{b:02x}")).collect()
}

fn database_exists(admin: &mut PgConnection, name: &str) -> bool {
    #[derive(QueryableByName)]
    struct Count {
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        n: i64,
    }
    sql_query("SELECT count(*) AS n FROM pg_database WHERE datname = $1")
        .bind::<diesel::sql_types::Text, _>(name)
        .get_result::<Count>(admin)
        .expect("Failed to look up the test template")
        .n > 0
}

/// A connection to the server's maintenance database, for creating and dropping databases.
fn admin_connection() -> PgConnection {
    PgConnection::establish(&url_for_database("postgres"))
        .expect("Failed to connect to the test Postgres server — is it running? (docker compose -f compose.test.yaml up -d --wait)")
}

/// `TEST_DATABASE_URL` with its database name swapped for `name`.
fn url_for_database(name: &str) -> String {
    let raw = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let mut url = url::Url::parse(&raw).expect("TEST_DATABASE_URL is not a valid URL");
    url.set_path(&format!("/{name}"));
    url.to_string()
}

fn exec(conn: &mut PgConnection, sql: &str) {
    sql_query(sql).execute(conn).unwrap_or_else(|e| panic!("Failed: {sql}: {e}"));
}

fn unix_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("clock before 1970").as_secs()
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
