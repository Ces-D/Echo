use actix_web::{http::header::Header, Result};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};

/// Wrappers around rspotify models
pub mod model;
pub mod tracks;

use crate::shared::{
    config::{get_env_var, Environment},
    crypto::authorization::decrypt_session_token,
};

pub fn spotify_client_from_req(req: actix_web::HttpRequest) -> Result<rspotify::AuthCodeSpotify> {
    let credentials = Authorization::<Bearer>::parse(&req)?;
    let bearer = credentials.as_ref();
    let token = decrypt_session_token(bearer.token())?;
    Ok(spotify_client(Some(token), None))
}

/// Creates a new Spotify client with the provided token, or without one if none is given.
pub fn spotify_client(
    token: Option<rspotify::Token>,
    oauth_state: Option<String>,
) -> rspotify::AuthCodeSpotify {
    let client_id = get_env_var(Environment::SpotifyClientId);
    let client_secret = get_env_var(Environment::SpotifyClientSecret);
    let redirect_uri = get_env_var(Environment::SpotifyRedirectUri);

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
