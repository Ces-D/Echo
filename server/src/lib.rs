use actix_web::{dev::ServiceRequest, web::Data, App, HttpServer, Result};
use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};
use apistos::{app::OpenApiWrapper, web};

mod client;
mod schema;
mod service;
mod shared;

async fn authorizaton_required(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    match shared::crypto::authorization::decrypt_session_token(credentials.token()) {
        Ok(_) => Ok(req),
        Err(_) => Err((
            actix_web::error::ErrorUnauthorized("Incorrect authorization"),
            req,
        )),
    }
}

pub async fn run() -> std::io::Result<()> {
    let bind_address = shared::config::get_env_var(shared::config::Environment::BindAddress);

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_pool = client::pg::database_pool();

    HttpServer::new(move || {
        let api_spec = shared::config::create_api_spec();
        let cors = shared::config::create_cors();

        App::new()
            // ~~~ API Spec
            .document(api_spec)
            // ~~~ App data
            .app_data(Data::new(database_pool.clone()))
            // ~~~ Global Middleware
            .wrap(actix_web::middleware::Compress::default())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(cors)
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
                    .wrap(HttpAuthentication::bearer(authorizaton_required))
                    .route(
                        "/current_user",
                        web::get().to(service::user::get_complete_current_user),
                    )
                    .route(
                        "/current_user/playlists",
                        web::get().to(service::playlist::get_user_playlists::get_user_playlists),
                    ),
            )
            .build("/openapi.json")
    })
    .bind(bind_address)?
    .run()
    .await
}
