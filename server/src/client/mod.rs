use crate::shared::config;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub mod pg;

/// Creates a new Spotify client with the provided token, or without one if none is given.
pub fn spotify_client(
    token: Option<rspotify::Token>,
    oauth_state: Option<String>,
) -> rspotify::AuthCodeSpotify {
    let client_id = config::get_env_var(config::Environment::SpotifyClientId);
    let client_secret = config::get_env_var(config::Environment::SpotifyClientSecret);
    let redirect_uri = config::get_env_var(config::Environment::SpotifyRedirectUri);

    let creds = rspotify::Credentials {
        id: client_id,
        secret: Some(client_secret),
    };

    let scopes = rspotify::scopes!(
        "user-library-read",
        "playlist-read-private",
        "playlist-modify-private",
        "playlist-modify-public"
    );

    let oauth = match oauth_state {
        Some(s) => rspotify::OAuth {
            redirect_uri,
            scopes,
            state: s,
            ..rspotify::OAuth::default()
        },
        None => rspotify::OAuth {
            redirect_uri,
            scopes,
            ..rspotify::OAuth::default()
        },
    };

    let config = rspotify::Config {
        token_refreshing: true,
        token_cached: false,
        ..rspotify::Config::default()
    };

    match token {
        Some(t) => rspotify::AuthCodeSpotify::from_token_with_config(t, creds, oauth, config),
        None => rspotify::AuthCodeSpotify::with_config(creds, oauth, config),
    }
}

/// Creates a connection pool to the Postgres database
pub fn database_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_uri = config::get_env_var(config::Environment::DatabaseUrl);

    // Create a connection pool to the Postgres database
    let manager = ConnectionManager::<PgConnection>::new(database_uri);
    Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to Postgres DB")
}
