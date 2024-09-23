use std::io::Write;

use spotify_rs::auth::{NoVerifier, Token};
use spotify_rs::client::Client;
use spotify_rs::model::playlist::{Playlist, SimplifiedPlaylist};
use spotify_rs::AuthCodeFlow;
use tempfile::NamedTempFile;

use crate::error::EchoError;
use crate::utils::any_as_u8_slice;

use super::cache::create_app_temp_file;
use super::constants::SPOTIFY_PLAYLISTS_LIMIT;
use super::params::{SpotifyAddItemsParams, SpotifyTracksParams};

struct LoadTracksParams {
    playlist_id: String,
    spotify: Option<SpotifyTracksParams>,
}
///  Load tracks from a playlist
async fn load_playlist_tracks(
    client: &mut Client<Token, AuthCodeFlow, NoVerifier>,
    params: LoadTracksParams,
) -> Result<NamedTempFile, EchoError> {
    let mut playlist_tmp_file =
        create_app_temp_file(&params.playlist_id).map_err(|_| EchoError::IoNamedTempFileError)?;

    let mut spotify_params = match params.spotify {
        Some(spotify_params) => spotify_params,
        None => {
            let playlist_data = client
                .playlist(&params.playlist_id)
                .get()
                .await
                .map_err(|error| EchoError::ClientRequestError(error.to_string()))?;
            SpotifyTracksParams::new(
                SpotifyTracksParams::default().offset,
                playlist_data.tracks.total,
            )
        }
    };

    while spotify_params.request_limit_exceeded() || spotify_params.request_required() {
        let playlist_data = client
            .playlist_items(&params.playlist_id)
            .limit(spotify_params.next_limit())
            .offset(spotify_params.offset)
            .get()
            .await
            .map_err(|error| EchoError::ClientRequestError(error.to_string()))?;

        let byte_data = unsafe { any_as_u8_slice(&playlist_data.items) };
        playlist_tmp_file
            .write_all(byte_data)
            .map_err(|_| EchoError::IoNamedTempFileError)?;
    }

    Ok(playlist_tmp_file)
}

/// Load tracks from the users starred playlist
async fn load_starred_playlist_tracks(
    client: &mut Client<Token, AuthCodeFlow, NoVerifier>,
    params: Option<SpotifyTracksParams>,
) -> Result<NamedTempFile, EchoError> {
    let mut playlist_tmp_file =
        create_app_temp_file("users_saved_tracks").map_err(|_| EchoError::IoNamedTempFileError)?;

    let mut spotify_params = match params {
        Some(spotify_params) => spotify_params,
        None => {
            let playlist_data = client
                .saved_tracks()
                .get()
                .await
                .map_err(|error| EchoError::ClientRequestError(error.to_string()))?;
            SpotifyTracksParams::new(SpotifyTracksParams::default().offset, playlist_data.total)
        }
    };

    while spotify_params.request_limit_exceeded() || spotify_params.request_required() {
        let playlist_data = client
            .saved_tracks()
            .limit(spotify_params.next_limit())
            .offset(spotify_params.offset)
            .get()
            .await
            .map_err(|error| EchoError::ClientRequestError(error.to_string()))?;

        let byte_data = unsafe { any_as_u8_slice(&playlist_data.items) };
        playlist_tmp_file
            .write_all(byte_data)
            .map_err(|_| EchoError::IoNamedTempFileError)?;
    }

    Ok(playlist_tmp_file)
}

struct CreatePlaylistParams {
    name: String,
    description: String,
    user_id: String,
}
async fn create_playlist(
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

struct AddItemsToPlaylistParams {
    playlist_id: String,
    spotify: SpotifyAddItemsParams,
}
async fn add_tracks_to_playlist(
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

struct FindPlaylistUsingIdentifiersParams {
    name: String,
    description: Option<String>,
}
async fn find_playlist_using_identifiers(
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
