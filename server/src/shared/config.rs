use std::fmt::Display;

pub enum Environment {
    DatabaseUrl,
    SpotifyClientId,
    SpotifyClientSecret,
    SpotifyRedirectUri,
    HashKey,
    BaseUrl,
    BaseServerPort,
    SessionCookieKey,
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::DatabaseUrl => write!(f, "DATABASE_URL"),
            Environment::SpotifyClientId => write!(f, "SPOTIFY_CLIENT_ID"),
            Environment::SpotifyClientSecret => write!(f, "SPOTIFY_CLIENT_SECRET"),
            Environment::SpotifyRedirectUri => write!(f, "SPOTIFY_REDIRECT_URI"),
            Environment::HashKey => write!(f, "HASH_KEY"),
            Environment::BaseUrl => write!(f, "BASE_URL"),
            Environment::BaseServerPort => write!(f, "BASE_SERVER_PORT"),
            Environment::SessionCookieKey => write!(f, "SESSION_COOKIE_KEY"),
        }
    }
}

pub fn get_env_var(env: Environment) -> String {
    std::env::var(env.to_string()).expect(format!("{} must be set", env).as_str())
}
