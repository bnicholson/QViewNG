//! Valkey-backed cache.
//!
//! [Valkey](https://valkey.io) is the BSD-licensed, Linux Foundation fork of Redis 7.2.4. It
//! is protocol-identical to Redis, which is why two Redis names unavoidably appear here and
//! nowhere else in this project:
//!
//!   * the `redis` crate -- the Rust client; it speaks RESP and works against Valkey unchanged
//!   * the `redis://` URL scheme -- the name of the wire protocol, which Valkey also uses
//!
//! Everything this project owns (the env var, the compose service, this module) is named for
//! Valkey. Do not read those two names as a dependency on Redis the server.
//!
//! # The one rule
//!
//! **The cache must never be able to take the site down.** Every operation degrades to a miss.
//! An unset `VALKEY_URL`, an unreachable server, a timeout, a serialization failure and a
//! stale entry that no longer deserializes are all the same thing to a caller:
//!
//!   * [`Cache::get_json`] returns `None`
//!   * [`Cache::set_json`] and [`Cache::invalidate`] do nothing and log a warning
//!
//! Nothing in this module panics, and nothing returns an error for a caller to handle. That is
//! deliberate: a cache is an optimization, and an optimization that can fail a live broadcast
//! is worse than no cache at all. Note what this rules out -- the cache can never be used as
//! the system of record for anything. Postgres stays authoritative.

use std::time::Duration;

use redis::aio::ConnectionManager;
use serde::Serialize;
use serde::de::DeserializeOwned;

/// How long to wait for the initial connection before giving up and running uncached.
const CONNECT_TIMEOUT: Duration = Duration::from_secs(2);

/// Per-operation ceiling. A wedged or overloaded Valkey must not add latency to a request, so
/// past this we abandon the operation and treat it as a miss. Recomputing stats is slow but
/// bounded; waiting on a dead socket is not.
const OP_TIMEOUT: Duration = Duration::from_millis(250);

/// A handle to the Valkey cache, or to nothing at all.
///
/// Cheap to clone -- [`ConnectionManager`] is internally reference-counted and multiplexes
/// commands over one connection, so cloning this per request (or per Actix worker) is fine.
/// It also reconnects on its own if the connection drops after startup.
#[derive(Clone)]
pub struct Cache {
    /// `None` means caching is disabled and every method below is a no-op.
    manager: Option<ConnectionManager>,
}

impl Cache {
    /// Builds a cache from the URL in `url_env_var_name` (conventionally `VALKEY_URL`).
    ///
    /// Returns a disabled cache -- never an error -- if the variable is unset or blank, if the
    /// URL is malformed, or if the server cannot be reached within [`CONNECT_TIMEOUT`]. That
    /// is what makes `cargo test` and a local `cargo run` work with no Valkey running.
    pub async fn from_env(url_env_var_name: &str) -> Cache {
        match std::env::var(url_env_var_name) {
            Ok(url) if !url.trim().is_empty() => Cache::connect(url.trim()).await,
            _ => {
                log::info!(
                    "{} is not set; running with caching disabled.",
                    url_env_var_name
                );
                Cache::disabled()
            }
        }
    }

    /// Connects to Valkey at `url`, falling back to a disabled cache on any failure.
    pub async fn connect(url: &str) -> Cache {
        let shown = redact_credentials(url);

        let client = match redis::Client::open(url) {
            Ok(client) => client,
            Err(e) => {
                log::warn!("Valkey URL {shown} is unusable ({e}); caching disabled.");
                return Cache::disabled();
            }
        };

        // `ConnectionManager::new` retries internally, so bound it: a down Valkey must not
        // hold up server startup.
        match tokio::time::timeout(CONNECT_TIMEOUT, ConnectionManager::new(client)).await {
            Ok(Ok(manager)) => {
                log::info!("Connected to Valkey at {shown}; caching enabled.");
                Cache { manager: Some(manager) }
            }
            Ok(Err(e)) => {
                log::warn!("Could not connect to Valkey at {shown} ({e}); caching disabled.");
                Cache::disabled()
            }
            Err(_) => {
                log::warn!(
                    "Connecting to Valkey at {shown} timed out after {:?}; caching disabled.",
                    CONNECT_TIMEOUT
                );
                Cache::disabled()
            }
        }
    }

    /// A cache that stores nothing. Every method is a no-op and every read is a miss.
    ///
    /// Note the one asymmetry worth knowing: `ConnectionManager` reconnects on its own if the
    /// connection drops *after* startup, but a cache that started disabled because Valkey was
    /// down stays disabled until the process restarts. In deployment `compose.yaml` gates the
    /// app on `valkey` being healthy, so this only bites a cold start against a down server.
    pub fn disabled() -> Cache {
        Cache { manager: None }
    }

    /// Whether this cache actually talks to a server. For health endpoints and logging; the
    /// read and write paths handle the disabled case themselves.
    pub fn is_enabled(&self) -> bool {
        self.manager.is_some()
    }

    /// Reads and deserializes `key`, or `None` on a miss **or any failure**.
    ///
    /// A stale entry whose shape no longer matches `T` is reported as a miss, not an error --
    /// which is the safety net behind the version token in a cache key (see
    /// `gameevent::CALC_VERSION`): even if a version bump is forgotten, the worst case is a
    /// recompute rather than wrong data being served.
    pub async fn get_json<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        let manager = self.manager.as_ref()?;
        let mut conn = manager.clone();

        // `Cmd` must outlive the future that borrows it, so bind it before awaiting.
        let mut cmd = redis::cmd("GET");
        cmd.arg(key);
        let query = cmd.query_async::<Option<String>>(&mut conn);
        let raw = match tokio::time::timeout(OP_TIMEOUT, query).await {
            Ok(Ok(raw)) => raw?,
            Ok(Err(e)) => {
                log::warn!("Valkey GET {key} failed ({e}); treating as a cache miss.");
                return None;
            }
            Err(_) => {
                log::warn!("Valkey GET {key} timed out; treating as a cache miss.");
                return None;
            }
        };

        match serde_json::from_str(&raw) {
            Ok(value) => Some(value),
            Err(e) => {
                log::warn!(
                    "Cached value at {key} did not deserialize ({e}); treating as a cache \
                     miss. If this repeats, a cache-key version token needs bumping."
                );
                None
            }
        }
    }

    /// Serializes `value` and stores it at `key` with a `ttl` expiry. Best-effort: logs and
    /// moves on if anything fails.
    ///
    /// The TTL is mandatory rather than optional on purpose. Explicit invalidation is the
    /// primary correctness mechanism, but any invalidation scheme eventually misses a path;
    /// a TTL caps how long a mistake can be served.
    pub async fn set_json<T: Serialize + ?Sized>(&self, key: &str, value: &T, ttl: Duration) {
        let Some(manager) = self.manager.as_ref() else {
            return;
        };

        let payload = match serde_json::to_string(value) {
            Ok(payload) => payload,
            Err(e) => {
                log::warn!("Could not serialize the value for {key} ({e}); not caching it.");
                return;
            }
        };

        // Valkey rejects an expiry of 0, and a sub-second TTL is never what a caller meant.
        let ttl_secs = ttl.as_secs().max(1);

        let mut conn = manager.clone();
        // `Cmd` must outlive the future that borrows it, so bind it before awaiting.
        let mut cmd = redis::cmd("SET");
        cmd.arg(key).arg(payload).arg("EX").arg(ttl_secs);
        let query = cmd.query_async::<()>(&mut conn);

        match tokio::time::timeout(OP_TIMEOUT, query).await {
            Ok(Ok(())) => {}
            Ok(Err(e)) => log::warn!("Valkey SET {key} failed ({e}); value not cached."),
            Err(_) => log::warn!("Valkey SET {key} timed out; value not cached."),
        }
    }

    /// Deletes `keys`. Best-effort, and a no-op when `keys` is empty.
    ///
    /// Takes a slice so a write path can drop a game's entry and every stats group that game
    /// belongs to in one round trip. Missing keys are not an error.
    ///
    /// Note the failure mode this *cannot* paper over: if an invalidation is lost (server
    /// down, timeout), stale stats are served until the TTL expires. Write paths that must
    /// not serve stale data should be ordered to write Postgres first, then invalidate, so a
    /// lost invalidation degrades to "briefly stale" rather than "permanently wrong".
    pub async fn invalidate(&self, keys: &[String]) {
        let Some(manager) = self.manager.as_ref() else {
            return;
        };
        if keys.is_empty() {
            return;
        }

        let mut conn = manager.clone();
        let mut cmd = redis::cmd("DEL");
        for key in keys {
            cmd.arg(key);
        }
        let query = cmd.query_async::<()>(&mut conn);

        match tokio::time::timeout(OP_TIMEOUT, query).await {
            Ok(Ok(())) => {}
            Ok(Err(e)) => log::warn!(
                "Valkey DEL of {} key(s) failed ({e}); stale entries will persist until their \
                 TTL expires.",
                keys.len()
            ),
            Err(_) => log::warn!(
                "Valkey DEL of {} key(s) timed out; stale entries will persist until their \
                 TTL expires.",
                keys.len()
            ),
        }
    }

    /// Deletes every key this project owns (everything under `qv:`). Best-effort.
    ///
    /// For dev reseeding: after the database is wiped and repopulated, ids are reused, so any
    /// cached entry keyed by an old id would describe data that no longer exists. Scoped to
    /// `qv:*` with SCAN rather than FLUSHDB so a Valkey shared with anything else is left alone.
    pub async fn clear_all(&self) {
        let Some(manager) = self.manager.as_ref() else {
            return;
        };
        let mut conn = manager.clone();

        let mut cursor: u64 = 0;
        let mut deleted: usize = 0;
        loop {
            let mut cmd = redis::cmd("SCAN");
            cmd.arg(cursor).arg("MATCH").arg("qv:*").arg("COUNT").arg(1000);
            let query = cmd.query_async::<(u64, Vec<String>)>(&mut conn);
            let (next, keys) = match tokio::time::timeout(OP_TIMEOUT, query).await {
                Ok(Ok(page)) => page,
                Ok(Err(e)) => {
                    log::warn!("Valkey SCAN failed ({e}); cache only partially cleared.");
                    return;
                }
                Err(_) => {
                    log::warn!("Valkey SCAN timed out; cache only partially cleared.");
                    return;
                }
            };

            deleted += keys.len();
            self.invalidate(&keys).await;

            if next == 0 {
                break;
            }
            cursor = next;
        }
        log::info!("Cleared {deleted} cached key(s) from Valkey.");
    }
}

/// Replaces any `user:password@` in a connection URL with `***@` so it is safe to log.
fn redact_credentials(url: &str) -> String {
    let Some((scheme, rest)) = url.split_once("//") else {
        return url.to_string();
    };
    match rest.rsplit_once('@') {
        Some((_credentials, host)) => format!("{scheme}//***@{host}"),
        None => url.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A disabled cache has to be usable, not just constructible: these are the exact calls
    /// every request path makes when `VALKEY_URL` is unset, as it is under `cargo test`.
    #[actix_web::test]
    async fn a_disabled_cache_is_a_silent_no_op() {
        let cache = Cache::disabled();

        assert![!cache.is_enabled()];
        assert![cache.get_json::<String>("qv:v1:absent").await.is_none()];

        // Neither of these may panic or block.
        cache.set_json("qv:v1:absent", &"value", Duration::from_secs(60)).await;
        cache.invalidate(&["qv:v1:absent".to_string()]).await;

        // Still a miss afterwards -- a disabled cache stores nothing.
        assert![cache.get_json::<String>("qv:v1:absent").await.is_none()];
    }

    /// A bad URL must degrade to a disabled cache rather than failing server startup.
    #[actix_web::test]
    async fn an_unusable_url_degrades_to_disabled() {
        assert![!Cache::connect("not-a-url").await.is_enabled()];
    }

    /// An unset or blank env var means "no caching", not an error.
    #[actix_web::test]
    async fn an_unset_env_var_degrades_to_disabled() {
        let name = "VALKEY_URL_CACHE_UNIT_TEST_ABSENT";
        unsafe { std::env::remove_var(name) };
        assert![!Cache::from_env(name).await.is_enabled()];

        unsafe { std::env::set_var(name, "   ") };
        assert![!Cache::from_env(name).await.is_enabled()];
        unsafe { std::env::remove_var(name) };
    }

    #[test]
    fn credentials_are_redacted_before_logging() {
        assert_eq![
            redact_credentials("redis://default:s3cret@valkey:6379"),
            "redis://***@valkey:6379"
        ];
        // The common case has no credentials and must pass through unchanged.
        assert_eq![redact_credentials("redis://valkey:6379"), "redis://valkey:6379"];
        assert_eq![redact_credentials("garbage"), "garbage"];
    }

    /// Round-trip against a real Valkey. Opt-in, because CI and `cargo test` must not need a
    /// server: ignored by default, and skipped unless `VALKEY_TEST_URL` is set.
    ///
    ///   docker run -d --rm --name qview-valkey-test -p 6399:6379 valkey/valkey:8-alpine \
    ///     valkey-server --save "" --maxmemory 256mb --maxmemory-policy allkeys-lru
    ///   VALKEY_TEST_URL=redis://127.0.0.1:6399 cargo test --lib cache -- --ignored
    #[actix_web::test]
    #[ignore = "needs a running Valkey; set VALKEY_TEST_URL"]
    async fn a_connected_cache_round_trips_set_get_and_invalidate() {
        let Ok(url) = std::env::var("VALKEY_TEST_URL") else {
            eprintln!("VALKEY_TEST_URL not set; skipping.");
            return;
        };

        let cache = Cache::connect(&url).await;
        assert![cache.is_enabled(), "expected to connect to Valkey at {url}"];

        // Unique per run so repeat runs and parallel tests cannot collide.
        let key = format!("qv:test:{}", uuid::Uuid::new_v4());

        // A key that was never written is a miss, not an error.
        assert_eq![cache.get_json::<Vec<i32>>(&key).await, None];

        // Round-trip a value with the same shape as the real payload: a struct holding an
        // integer-keyed map, which is what `CalculatedGame` is and where serde_json's
        // string-key rendering could bite.
        let mut written = std::collections::HashMap::new();
        written.insert(0, "Red Team".to_string());
        written.insert(1, "Blue Team".to_string());

        cache.set_json(&key, &written, Duration::from_secs(60)).await;

        let read: std::collections::HashMap<i32, String> = cache
            .get_json(&key)
            .await
            .expect("value written a moment ago should read back");
        assert_eq![read, written, "integer map keys must survive Valkey + serde_json"];

        // Invalidation really removes it.
        cache.invalidate(&[key.clone()]).await;
        assert_eq![
            cache.get_json::<std::collections::HashMap<i32, String>>(&key).await,
            None,
            "invalidate should have removed the key"
        ];

        // Deleting an absent key is not an error, and an empty slice is a no-op.
        cache.invalidate(&[key.clone()]).await;
        cache.invalidate(&[]).await;

        // A short TTL really expires.
        let ttl_key = format!("qv:test:ttl:{}", uuid::Uuid::new_v4());
        cache.set_json(&ttl_key, &"transient", Duration::from_secs(1)).await;
        assert![cache.get_json::<String>(&ttl_key).await.is_some()];
        tokio::time::sleep(Duration::from_millis(1500)).await;
        assert_eq![
            cache.get_json::<String>(&ttl_key).await,
            None,
            "the 1s TTL should have expired the key"
        ];
    }
}
