
use actix_web::{App, HttpServer};
use actix_web::middleware::{Compress, Logger, NormalizePath};
use backend::database::Database;
use backend::cache::Cache;
use backend::live::{self, LiveHub};
use backend::database::clean_db::clean_database;
use backend::database::seed_data::seed_one::insert_seed_data_one;
use backend::database::seed_data::system_default_data::insert_system_default_data;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use backend::routes::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    #[cfg(debug_assertions)]
    dotenvy::from_filename(".env.dev").ok();
    #[cfg(not(debug_assertions))]
    dotenvy::from_filename(".env.release").ok();

    #[cfg(debug_assertions)] {
        tracing_log::LogTracer::init().ok();
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::DEBUG)
            .finish();
        tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    }
    
    #[cfg(not(debug_assertions))] {
        // Handle setting up log4rs (logging)
        log4rs::init_file("config/logging_prod.yaml", Default::default()).unwrap();
    }

    // tell everyone we have logging running
    log::info!("Initialized log4rs");

    // `cargo test` builds every target with the test-only feature, including this binary — so a
    // `target/debug/backend` left behind by a test run would store weak password hashes. `cargo run`
    // and release builds never enable it; say so loudly if it ever slips through.
    if cfg!(feature = "fast-test-hash") {
        log::warn!("Built with the test-only `fast-test-hash` feature: passwords are hashed at bcrypt's minimum cost. Do not use this build for real data.");
    }

    // DB maintenance runs on its own and exits before anything server-related starts. It needs
    // only one connection, so it uses a single-connection pool rather than the server's ten.
    if false {
        let db = Database::new_single_threaded("DATABASE_URL");
        let mut conn = db.get_connection().expect("Failed to get connection.");
        println!("");
        if true {
            clean_database(&mut conn);  // Removes all data from DB
            Cache::from_env("VALKEY_URL").await.clear_all().await;  // Drops cached entries keyed by now-reused ids

            if true {  // 'false' only when wanting a completely empty
                insert_system_default_data(&mut conn);  // (*required in prod and dev for permissions to work)
            }
        }
        if true {    
            // Repopulates DB with seed data (*for manual UI testing)
            // Settings:
            let include_scheduling: bool = true;   // seed the full schedule (rounds/pools/games); set false to stop at registered teams + gear
            let include_gameevents: bool = false;  // ***This applies only if include_scheduling = true;
            insert_seed_data_one(&mut conn, include_scheduling, include_gameevents);
        }
        let conn = conn;
        drop(conn);

        return Ok(());
    }

    // Grab the HOST:PORT the web server should run on.
    let host = match std::env::var("HOST") {
        Ok(h) => h,
        Err(_) => "127.0.0.1".to_string()
    };
    let port = match std::env::var("APP_PORT") {
        Ok(p) => p.to_string(),
        Err(_) => "3000".to_string()
    };
    let host_and_port = format!("{host}:{port}");

    let db = Database::new("DATABASE_URL");

    // Valkey cache. Built once and shared by every worker; `Cache` is cheap to clone and
    // multiplexes over a single auto-reconnecting connection. If VALKEY_URL is unset or the
    // server is unreachable, this returns a disabled cache and the app runs uncached rather
    // than failing to start -- see backend/src/cache.rs.
    let cache = Cache::from_env("VALKEY_URL").await;

    // Bring every division's own stats group in step with the division's games. Games used to join
    // it only when the division was created, so a division scheduled afterwards was missing most of
    // its games from its standings. Idempotent, and a no-op once everything is in step.
    match db.get_connection().map_err(|e| e.to_string())
        .and_then(|mut conn| backend::models::statsgroup::sync_all_division_statsgroups(&mut conn).map_err(|e| e.to_string()))
    {
        Ok(0) => {}
        Ok(changed) => log::info!("Brought {changed} division stats group(s) in step with their divisions' games."),
        Err(e) => log::error!("Could not sync division stats groups at startup: {e}"),
    }

    // Live updates for pages watching a tournament (Games in Progress, the Server Monitor). One
    // hub shared by every worker; its background worker recalculates only what someone is
    // watching -- see backend/src/live.rs.
    let live = LiveHub::start(live::snapshot_builder(db.clone(), cache.clone()));
    
    log::info!("Server listening on http://{host_and_port} ...");

    HttpServer::new(move || {
        App::new()
        .wrap(Compress::default())
        .wrap(NormalizePath::trim())
        .wrap(Logger::default())
        .app_data(actix_web::web::Data::new(db.clone()))
        .app_data(actix_web::web::Data::new(cache.clone()))
        .app_data(actix_web::web::Data::new(live.clone()))
        // .app_data(Data::new(app_data.mailer.clone()))
        // .app_data(Data::new(schema.clone()))
        // .app_data(Data::new(storage.clone()))
        .configure(configure_routes)
    })
    // Open live-update streams never finish on their own, so a graceful shutdown would otherwise
    // wait the full default 30 s for them. Their pages reconnect to the restarted server.
    .shutdown_timeout(5)
    .bind(host_and_port)?
    .run()
    .await
}