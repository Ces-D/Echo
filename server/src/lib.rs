use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::{from_fn, Next},
    web, App, Error, HttpServer, Result,
};

mod client;
mod schema;
mod service;
mod shared;

async fn authorizaton_required(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let cookie_key = shared::config::get_env_var(shared::config::Environment::SessionCookieKey);
    match req.cookie(&cookie_key) {
        Some(cookie) => {
            let token = shared::crypto::decrypt_session_cookie(cookie);
            if token.is_expired() {
                return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
            } else {
                next.call(req).await
            }
        }
        None => Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
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
                    )
                    .route(
                        "/authorization_valid",
                        web::get().to(service::auth::authorization_valid),
                    ),
            )
            .service(
                web::scope("/v1")
                    .wrap(from_fn(authorizaton_required))
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
