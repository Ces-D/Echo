use apistos::ApiComponent;
use rspotify::model;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
pub struct Image {
    pub height: Option<u32>,
    pub url: String,
    pub width: Option<u32>,
}
impl From<model::Image> for Image {
    fn from(image: rspotify::model::Image) -> Self {
        Image {
            height: image.height,
            url: image.url,
            width: image.width,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
pub enum SubscriptionLevel {
    Premium,
    Free,
}
impl From<model::SubscriptionLevel> for SubscriptionLevel {
    fn from(subscription_level: rspotify::model::SubscriptionLevel) -> Self {
        match subscription_level {
            rspotify::model::SubscriptionLevel::Premium => SubscriptionLevel::Premium,
            rspotify::model::SubscriptionLevel::Free => SubscriptionLevel::Free,
        }
    }
}

/// Wrapper around  rspotify::PrivateUser
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
pub struct PrivateUser {
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub product: Option<SubscriptionLevel>,
}

impl From<model::user::PrivateUser> for PrivateUser {
    fn from(user: rspotify::model::user::PrivateUser) -> Self {
        PrivateUser {
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

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
pub struct SimplePlaylist {
    pub collaborative: bool,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub owner_id: String,
    pub public: Option<bool>,
    pub total_tracks: u32,
}

impl From<model::SimplifiedPlaylist> for SimplePlaylist {
    fn from(playlist: rspotify::model::SimplifiedPlaylist) -> Self {
        SimplePlaylist {
            collaborative: playlist.collaborative,
            id: playlist.id.to_string(),
            images: playlist
                .images
                .iter()
                .map(|image| image.clone().into())
                .collect(),
            name: playlist.name,
            owner_id: playlist.owner.id.to_string(),
            public: playlist.public,
            total_tracks: playlist.tracks.total,
        }
    }
}
