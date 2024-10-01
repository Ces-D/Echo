use std::borrow::BorrowMut;
use std::ops::Index;

use echo::error::EchoError;
use spotify_rs::auth::{NoVerifier, Token};
use spotify_rs::client::Client;
use spotify_rs::AuthCodeFlow;

pub struct SummarizedPlaylist {
    pub name: String,
    pub description: Option<String>,
    pub id: String,
}

pub async fn find_playlist_handler(
    client: &mut Client<Token, AuthCodeFlow, NoVerifier>,
    name: String,
    description: Option<String>,
    user_id: String,
) -> Result<Vec<SummarizedPlaylist>, EchoError> {
    let playlists = client
        .user_playlists(user_id)
        .get()
        .await
        .map_err(|error| EchoError::ClientRequestError(error.to_string()))?;

    let mut metadata: Vec<String> = vec![];
    for item in playlists.items {
        metadata.push(format!(
            "{}///{}///{}",
            item.name,
            item.description.unwrap_or("N/A".to_string()),
            item.id
        ));
    }

    // TODO: THIS FAILED maybe its the matched or the pattern
    let mut matcher = nucleo_matcher::Matcher::new(nucleo_matcher::Config::DEFAULT.match_paths());
    let metadata_matches = nucleo_matcher::pattern::Pattern::new(
        &format!("{}///{}", name, description.unwrap_or("N/A".to_string())),
        nucleo_matcher::pattern::CaseMatching::Ignore,
        nucleo_matcher::pattern::Normalization::Smart,
        nucleo_matcher::pattern::AtomKind::Fuzzy,
    )
    .match_list(metadata, matcher.borrow_mut());

    let mut summarized_playlists = vec![];
    for m in metadata_matches {
        let mdata: Vec<&str> = m.0.split("///").collect();
        assert!(
            mdata.len() == 3,
            "Expected metadata to include 3 items (name, description, id)"
        );
        let m_name = mdata.index(0).to_string();
        let m_description = mdata.index(1).to_string();
        let m_id = mdata.index(2).to_string();
        summarized_playlists.push(SummarizedPlaylist {
            name: m_name,
            description: Some(m_description),
            id: m_id,
        });
    }

    Ok(summarized_playlists)
}
