use actix_web::{dev::ServiceRequest, Result};
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use actix_web_httpauth::middleware::HttpAuthentication;

mod client;
mod schema;
mod service;
mod shared;

pub async fn authorization_middleware(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    if req.path() == "/health_check" {
        return Ok(req);
    }
    if req.path().starts_with("/auth") {
        return Ok(req);
    }
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

pub async fn run(address: &str) -> std::io::Result<()> {
    let database_pool = crate::client::database_pool();

    HttpServer::new(move || {
        App::new()
            // ~~~ App data
            .app_data(web::Data::new(database_pool.clone()))
            // ~~~ Middleware
            .wrap(actix_web::middleware::NormalizePath::default())
            .wrap(actix_web::middleware::Compress::default())
            .wrap(HttpAuthentication::bearer(authorization_middleware))
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
            .route(
                "/current_user",
                web::get().to(service::user::get_complete_current_user),
            )
    })
    .bind(address)?
    .run()
    .await
}
