use std::borrow::BorrowMut;
use std::collections::HashMap;

use echo::error::EchoError;
use echo::spotify::SPOTIFY_PLAYLISTS_LIMIT;
use log::{debug, trace};
use rspotify::prelude::OAuthClient;
use rspotify::AuthCodeSpotify;

#[derive(Clone)]
pub struct SummarizedPlaylist {
    pub name: String,
    pub total: u32,
    pub public: bool,
    pub id: String,
}

pub async fn find_playlist_handler(
    client: AuthCodeSpotify,
    name: String,
) -> Result<Vec<SummarizedPlaylist>, EchoError> {
    let playlists = client
        .current_user_playlists_manual(Some(SPOTIFY_PLAYLISTS_LIMIT), Some(0))
        .await
        .map_err(|error| EchoError::ClientRequestError(error.to_string()))?;

    let mut metadata: HashMap<String, SummarizedPlaylist> = HashMap::default();
    for item in playlists.items {
        trace!("Adding {} to metadata", item.name);
        metadata.insert(
            item.name.clone(),
            SummarizedPlaylist {
                name: item.name,
                total: item.tracks.total,
                public: item.public.is_some_and(|x| x == true),
                id: item.id.to_string(),
            },
        );
    }

    let mut matcher = nucleo_matcher::Matcher::new(nucleo_matcher::Config::DEFAULT.match_paths());
    let metadata_matches = nucleo_matcher::pattern::Pattern::new(
        &name,
        nucleo_matcher::pattern::CaseMatching::Ignore,
        nucleo_matcher::pattern::Normalization::Smart,
        nucleo_matcher::pattern::AtomKind::Fuzzy,
    )
    .match_list(metadata.keys(), matcher.borrow_mut());

    let mut summarized_playlists = vec![];
    for m in metadata_matches {
        match metadata.get(m.0) {
            Some(summary) => summarized_playlists.push(summary.clone()),
            None => debug!("{} Is a match that should not exist", m.0),
        }
    }

    Ok(summarized_playlists)
}
