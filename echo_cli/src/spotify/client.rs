use rspotify::{AuthCodeSpotify, Credentials, OAuth};

pub struct RedirectCredentials {
    pub auth_code: String,
    pub csrf_token: String,
}

pub struct SpotifyConfig {
    pub redirect_port: String,
    pub client_id: String,
    pub client_secret: String,
}

impl SpotifyConfig {
    pub fn redirect_as_uri(&self) -> String {
        format!("http://localhost:{}", self.redirect_port)
    }
    pub fn redirect_as_addr(&self) -> String {
        format!("127.0.0.1:{}", self.redirect_port)
    }
}

pub fn read_config_from_env() -> SpotifyConfig {
    SpotifyConfig {
        redirect_port: std::env::var("redirect_port").unwrap(),
        client_id: std::env::var("client_id").unwrap(),
        client_secret: std::env::var("client_secret").unwrap(),
    }
}

pub fn create_client(config: &SpotifyConfig) -> AuthCodeSpotify {
    let creds = Credentials {
        id: config.client_id.clone(),
        secret: Some(config.client_secret.clone()),
    };
    let scopes = rspotify::scopes!(
        "user-library-read",
        "playlist-read-private",
        "playlist-modify-private",
        "playlist-modify-public"
    );
    let oauth = OAuth {
        redirect_uri: config.redirect_as_uri(),
        scopes,
        ..OAuth::default()
    };

    AuthCodeSpotify::new(creds, oauth)
}
