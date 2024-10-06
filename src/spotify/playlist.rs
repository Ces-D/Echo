use crate::spotify::params::SpotifyAddItemsParams;
use spotify_rs::auth::{NoVerifier, Token};
use spotify_rs::client::Client;
use spotify_rs::model::playlist::Playlist;
use spotify_rs::AuthCodeFlow;

use crate::error::EchoError;

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
