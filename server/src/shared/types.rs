use apistos::ApiComponent;
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A type alias for a connection pool to the Postgres database
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

/// A type alias for a connection to the Postgres database called from a connectin pool
pub type PoolConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
pub struct SpotifyImage {
    pub height: Option<u32>,
    pub url: String,
    pub width: Option<u32>,
}
impl From<rspotify::model::Image> for SpotifyImage {
    fn from(image: rspotify::model::Image) -> Self {
        SpotifyImage {
            height: image.height,
            url: image.url,
            width: image.width,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
pub enum SpotifySubscriptionLevel {
    Premium,
    Free,
}
impl From<rspotify::model::SubscriptionLevel> for SpotifySubscriptionLevel {
    fn from(subscription_level: rspotify::model::SubscriptionLevel) -> Self {
        match subscription_level {
            rspotify::model::SubscriptionLevel::Premium => SpotifySubscriptionLevel::Premium,
            rspotify::model::SubscriptionLevel::Free => SpotifySubscriptionLevel::Free,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
/// Wrapper around  rspotify::PrivateUser
pub struct SpotifyPrivateUser {
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<SpotifyImage>,
    pub product: Option<SpotifySubscriptionLevel>,
}

impl From<rspotify::model::user::PrivateUser> for SpotifyPrivateUser {
    fn from(user: rspotify::model::user::PrivateUser) -> Self {
        SpotifyPrivateUser {
            display_name: user.display_name,
            email: user.email,
            href: user.href,
            id: user.id.to_string(),
            images: match user.images {
                Some(i) => i.iter().map(|image| image.clone().into()).collect(),
                None => vec![],
            },
            product: Some(
                user.product
                    .unwrap_or(rspotify::model::SubscriptionLevel::Free)
                    .into(),
            ),
        }
    }
}
