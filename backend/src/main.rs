
use actix_web::{App, HttpServer};
use actix_web::middleware::{Compress, Logger, NormalizePath};
use backend::database;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use dotenvy::dotenv;
use backend::routes::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let env_file = std::env::var("ENV_FILE").unwrap_or_else(|_| ".env".to_string());
    dotenv::from_filename(&env_file).ok();

    #[cfg(debug_assertions)] {
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
    
    // Change commented/uncommented if you want to populate your DB with data:
    // let mut conn = db.get_connection().expect("Failed to get connection.");
    // database::clean_db::clean_database(&mut conn);
    // database::seed_data::system_default_data::insert_system_default_data(&mut conn);
    // database::seed_data::seed_one::insert_seed_data_one(&mut conn);
    // let conn = conn;
    // drop(conn);
    
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