use rspotify::model::{AlbumId, ArtistId, TrackId};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct EchoId<'a>(Cow<'a, str>);

impl EchoId<'_> {
    /// Uses targeted (presumably immutable) fields of a track to create a uniqueness hash
    pub fn from_full_track(value: &rspotify::model::FullTrack) -> Self {
        let mut hasher = std::hash::DefaultHasher::new();
        value.name.hash(&mut hasher);
        value.id.hash(&mut hasher);
        for artist_id in value.artists.iter().filter_map(|i| i.id.clone()) {
            artist_id.hash(&mut hasher);
        }
        value.album.id.hash(&mut hasher);
        let hash_result = hasher.finish();
        EchoId(hash_result.to_string().into())
    }

    /// Uses targeted (presumably immutable) fields of a simple artist to create a uniqueness hash
    fn from_simplified_artist(value: &rspotify::model::SimplifiedArtist) -> Self {
        let mut hasher = std::hash::DefaultHasher::new();
        value.name.hash(&mut hasher);
        value.id.hash(&mut hasher);
        let hash_result = hasher.finish();
        EchoId(hash_result.to_string().into())
    }

    /// Uses targeted (presumably immutable) fields of a simple album to create a uniqueness hash
    fn from_simplified_album(value: rspotify::model::SimplifiedAlbum) -> Self {
        let mut hasher = std::hash::DefaultHasher::new();
        value.name.hash(&mut hasher);
        value.id.hash(&mut hasher);
        let hash_result = hasher.finish();
        EchoId(hash_result.to_string().into())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct EchoFullArtist {
    pub echo_id: EchoId<'static>,
    pub id: Option<ArtistId<'static>>,
    pub name: String,
    pub genres: Vec<String>,
    pub popularity: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct EchoFullAlbum {
    pub echo_id: EchoId<'static>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date_precision: Option<String>,
    pub genres: Vec<String>,
    pub id: AlbumId<'static>,
    pub popularity: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct EchoFullTrack {
    pub echo_id: EchoId<'static>,
    pub explicit: bool,
    pub id: Option<TrackId<'static>>,
    pub is_local: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_playable: Option<bool>,
    pub name: String,
    pub popularity: u32,
    pub track_number: u32,
    pub album: EchoId<'static>,
    pub artists: Vec<EchoId<'static>>, // TODO(CES): csv serialization throws errors here
}

impl From<rspotify::model::FullTrack> for EchoFullTrack {
    fn from(value: rspotify::model::FullTrack) -> Self {
        EchoFullTrack {
            echo_id: EchoId::from_full_track(&value),
            explicit: value.explicit,
            id: value.id,
            is_local: value.is_local,
            is_playable: value.is_playable,
            name: value.name,
            popularity: value.popularity,
            track_number: value.track_number,
            album: EchoId::from_simplified_album(value.album),
            artists: value
                .artists
                .iter()
                .map(|i| EchoId::from_simplified_artist(i))
                .collect(),
        }
    }
}
