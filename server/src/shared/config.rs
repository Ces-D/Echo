use actix_cors::Cors;
use apistos::{info::Info, spec::Spec};
use std::fmt::Display;

pub enum Environment {
    DatabaseUrl,
    SpotifyClientId,
    SpotifyClientSecret,
    SpotifyRedirectUri,
    HashKey,
    BindAddress,
    SessionCookieKey,
    ClientUrl,
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::DatabaseUrl => write!(f, "DATABASE_URL"),
            Environment::SpotifyClientId => write!(f, "SPOTIFY_CLIENT_ID"),
            Environment::SpotifyClientSecret => write!(f, "SPOTIFY_CLIENT_SECRET"),
            Environment::SpotifyRedirectUri => write!(f, "SPOTIFY_REDIRECT_URI"),
            Environment::HashKey => write!(f, "HASH_KEY"),
            Environment::BindAddress => write!(f, "BIND_ADDRESS"),
            Environment::SessionCookieKey => write!(f, "SESSION_COOKIE_KEY"),
            Environment::ClientUrl => write!(f, "CLIENT_URL"),
        }
    }
}

pub fn get_env_var(env: Environment) -> String {
    std::env::var(env.to_string()).expect(format!("{} must be set", env).as_str())
}

pub fn create_api_spec() -> Spec {
    Spec {
        info: Info {
            title: "Echo Server".to_string(),
            version: "1.0".to_string(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn create_cors() -> Cors {
    Cors::default()
        .allowed_origin(&get_env_var(Environment::ClientUrl))
        .supports_credentials()
        .allow_any_method()
        .allow_any_header()
        .max_age(3_600)
}
