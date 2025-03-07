use actix_web::{dev::ServiceRequest, Result};
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use actix_web_httpauth::middleware::HttpAuthentication;

mod client;
mod schema;
mod service;
mod shared;

async fn authorization_middleware(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let bearer_token = credentials.token();
    if bearer_token.is_empty() {
        Err((
            actix_web::error::ErrorUnauthorized("Authorization not provided"),
            req,
        ))
    } else {
        Ok(req)
    }
}

pub async fn run() -> std::io::Result<()> {
    let base_url = shared::config::get_env_var(shared::config::Environment::BaseUrl);
    let server_port = shared::config::get_env_var(shared::config::Environment::BaseServerPort)
        .parse::<u16>()
        .expect("Server port should by a number");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_pool = client::database_pool();

    HttpServer::new(move || {
        App::new()
            // ~~~ App data
            .app_data(web::Data::new(database_pool.clone()))
            // ~~~ Middleware
            .wrap(actix_web::middleware::Compress::default())
            .wrap(actix_web::middleware::Logger::default())
            // ~~~ Routes
            .route(
                "/health_check",
                web::get().to(service::health_check::health_check),
            )
            // ~~~ Services
            .service(
                web::scope("/auth")
                    .route(
                        "/spotify",
                        web::get().to(service::auth::generate_spotify_request_url),
                    )
                    .route(
                        "/spotify/callback",
                        web::get().to(service::auth::parse_spotify_response_url),
                    ),
            )
            .service(
                web::scope("/v1")
                    .wrap(HttpAuthentication::bearer(authorization_middleware))
                    .route(
                        "/current_user",
                        web::get().to(service::user::get_complete_current_user),
                    ),
            )
    })
    .bind((base_url.as_str(), server_port))?
    .run()
    .await
}
