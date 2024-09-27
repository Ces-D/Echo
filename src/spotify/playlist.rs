use spotify_rs::auth::{NoVerifier, Token};
use spotify_rs::client::Client;
use spotify_rs::model::playlist::{Playlist, SimplifiedPlaylist};
use spotify_rs::AuthCodeFlow;

use crate::error::EchoError;

use super::constants::SPOTIFY_PLAYLISTS_LIMIT;
use super::params::SpotifyAddItemsParams;

pub struct CreatePlaylistParams {
    pub name: String,
    pub description: String,
    pub user_id: String,
}
pub async fn create_playlist(
    client: &mut Client<Token, AuthCodeFlow, NoVerifier>,
    params: CreatePlaylistParams,
) -> Result<Playlist, EchoError> {
    client
        .create_playlist(params.user_id, params.name)
        .description(params.description)
        .public(true)
        .send()
        .await
        .map_err(|error| EchoError::ClientRequestError(error.to_string()))
}

pub struct AddItemsToPlaylistParams {
    pub playlist_id: String,
    pub spotify: SpotifyAddItemsParams,
}
pub async fn add_tracks_to_playlist(
    client: &mut Client<Token, AuthCodeFlow, NoVerifier>,
    params: &mut AddItemsToPlaylistParams,
) -> Result<(), EchoError> {
    while params.spotify.request_limit_exceeded() || params.spotify.request_required() {
        let next_item_uris = params.spotify.next_items();
        if params.spotify.position.is_some() {
            client
                .add_items_to_playlist(&params.playlist_id, &next_item_uris)
                .position(
                    params
                        .spotify
                        .position
                        .expect("Converted position to None value"),
                )
                .send()
                .await
                .map_err(|error| EchoError::ClientRequestError(error.to_string()))?;
        } else {
            client
                .add_items_to_playlist(&params.playlist_id, &next_item_uris)
                .send()
                .await
                .map_err(|error| EchoError::ClientRequestError(error.to_string()))?;
        }
    }
    Ok(())
}

pub struct FindPlaylistUsingIdentifiersParams {
    pub name: String,
    pub description: Option<String>,
}
pub async fn find_playlist_using_identifiers(
    client: &mut Client<Token, AuthCodeFlow, NoVerifier>,
    params: FindPlaylistUsingIdentifiersParams,
) -> Result<SimplifiedPlaylist, EchoError> {
    let playlists = client
        .current_user_playlists()
        .limit(SPOTIFY_PLAYLISTS_LIMIT)
        .offset(0)
        .get()
        .await
        .map_err(|error| EchoError::ClientRequestError(error.to_string()))?;
    let target = playlists
        .items
        .into_iter()
        .find(|p| p.name == params.name && params.description == p.description);
    if target.is_some() {
        Ok(target.unwrap())
    } else {
        Err(EchoError::ClientRequestError(format!(
            "Unable to find playlist: {}",
            params.name,
        )))
    }
}
