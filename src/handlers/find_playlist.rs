use std::borrow::BorrowMut;
use std::collections::HashMap;

use echo::error::EchoError;
use log::{debug, trace};
use spotify_rs::auth::{NoVerifier, Token};
use spotify_rs::client::Client;
use spotify_rs::AuthCodeFlow;

#[derive(Clone)]
pub struct SummarizedPlaylist {
    pub name: String,
    pub description: Option<String>,
    pub id: String,
}

pub async fn find_playlist_handler(
    client: &mut Client<Token, AuthCodeFlow, NoVerifier>,
    name: String,
    user_id: String,
) -> Result<Vec<SummarizedPlaylist>, EchoError> {
    let playlists = client
        .user_playlists(user_id)
        .limit(50)
        .get()
        .await
        .map_err(|error| EchoError::ClientRequestError(error.to_string()))?;

    let mut metadata: HashMap<String, SummarizedPlaylist> = HashMap::default();
    for item in playlists.items {
        trace!("Adding {} to metadata", item.name);
        metadata.insert(
            item.name.clone(),
            SummarizedPlaylist {
                name: item.name,
                description: item.description,
                id: item.id,
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
