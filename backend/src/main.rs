
use actix_web::{App, HttpServer};
use actix_web::middleware::{Compress, Logger, NormalizePath};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use dotenvy::dotenv;
use backend::routes::configure_routes;
use backend::database::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    #[cfg(debug_assertions)] {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::DEBUG)
            .finish();
        tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    }
    
   #[cfg(not(debug_assertions))] {
       // Handle setting up log4rs (logging)
       // add syslog support
       let mut deserializers = log4rs::file::Deserializers::new();
       log4rs_syslog::register(&mut deserializers);

       log4rs::init_file("config/logging_prod.yaml",deserializers).unwrap();  
   }

    #[cfg(debug_assertions)]
    log4rs::init_file("config/logging_debug.yaml",Default::default()).unwrap();

    // tell everyone we have logging running
    log::info!("Initialized log4rs");

    // Grab the HOST:PORT the web server should run on.
    let host = match std::env::var("HOST") {
        Ok(h) => h,
        Err(_) => "127.0.0.1".to_string()
    };
    let port = match std::env::var("BACKEND_PORT") {
        Ok(p) => p.to_string(),
        Err(_) => "3000".to_string()
    };
    let host_and_port = format!("{host}:{port}");

    log::info!("Server listening on http://{host_and_port} ...");

    HttpServer::new(|| {
        App::new()
            .wrap(Compress::default())
            .wrap(NormalizePath::trim())
            .wrap(Logger::default())
            .app_data(actix_web::web::Data::new(Database::new("DATABASE_URL")))
            // .app_data(Data::new(app_data.mailer.clone()))
            // .app_data(Data::new(schema.clone()))
            // .app_data(Data::new(storage.clone()))
            .configure(configure_routes)
    })
    .bind(host_and_port)?
    .run()
    .await
}