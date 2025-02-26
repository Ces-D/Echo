use actix_web::{web, App, HttpServer};
use diesel::{r2d2, PgConnection};

mod config;
mod service;

pub async fn run(address: &str) -> std::io::Result<()> {
    let config = config::EchoConfig::new();

    // Create a connection pool to the Postgres database
    let manager = r2d2::ConnectionManager::<PgConnection>::new(config.database_url.as_str());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to Postgres DB");

    HttpServer::new(move || {
        App::new()
            .service(service::health_check::service)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(address)?
    .run()
    .await
}
