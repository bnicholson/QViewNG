
use actix_web::{App, HttpServer};
use actix_web::middleware::{Compress, Logger, NormalizePath};
use backend::database;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use backend::routes::configure_routes;
use chrono::Utc;

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

    let db = database::Database::new("DATABASE_URL");
    
    if false {
        let mut conn = db.get_connection().expect("Failed to get connection.");
        if false {
            // Removes all data from DB
            let start_time_for_db_purge = Utc::now();
            println!("Starting DB Purge");
            database::clean_db::clean_database(&mut conn);
            let end_time_for_db_purge = Utc::now();
            let duration_for_db_purge = end_time_for_db_purge.naive_utc() - start_time_for_db_purge.naive_utc();
            println!("Done. DB Purging Time Duration: {}\n", duration_for_db_purge);
        }
        if true {
            // Repopulates DB with default system data (*required in prod and dev for proper functioning)
            let start_time_for_db_pop_system_default_data = Utc::now();
            println!("Starting DB Data Population for System Default Data");
            database::seed_data::system_default_data::insert_system_default_data(&mut conn);
            let end_time_for_db_pop_system_default_data = Utc::now();
            let duration_for_db_pop_system_default_data = end_time_for_db_pop_system_default_data.naive_utc() - start_time_for_db_pop_system_default_data.naive_utc();
            println!("Done. DB System Default Data Population Time Duration: {}\n", duration_for_db_pop_system_default_data);
        }
        if true {    
            // Repopulates DB with seed data (*for manual UI testing)
            let include_gameevents_in_reseed: bool = false;
            let start_time_for_db_pop_seed_data = Utc::now();
            println!("Starting DB Data Population for Seed Data");
            database::seed_data::seed_one::insert_seed_data_one(&mut conn, include_gameevents_in_reseed);
            let end_time_for_db_pop_seed_data = Utc::now();
            let duration_for_db_pop_seed_data = end_time_for_db_pop_seed_data.naive_utc() - start_time_for_db_pop_seed_data.naive_utc();
            println!("Done. DB Seed Data Population Time Duration: {}\n", duration_for_db_pop_seed_data);
        }
        let conn = conn;
        drop(conn);
    }
    
    log::info!("Server listening on http://{host_and_port} ...");

    HttpServer::new(move || {
        App::new()
        .wrap(Compress::default())
        .wrap(NormalizePath::trim())
        .wrap(Logger::default())
        .app_data(actix_web::web::Data::new(db.clone()))
        // .app_data(Data::new(app_data.mailer.clone()))
        // .app_data(Data::new(schema.clone()))
        // .app_data(Data::new(storage.clone()))
        .configure(configure_routes)
    })
    .bind(host_and_port)?
    .run()
    .await
}