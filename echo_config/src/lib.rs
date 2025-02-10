use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EchoConfig {
    pub port: Option<u32>,
    pub spotify: Option<Spotify>,
    pub apple: Option<Apple>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spotify {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Apple {
    pub kid: String,
    pub team_id: String,
}
