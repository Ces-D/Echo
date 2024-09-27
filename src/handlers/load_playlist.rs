use echo::error::EchoError;
use echo::spotify::params::SpotifyReadTracksParams;
use echo::utils::any_as_u8_slice;
use log::trace;
use spotify_rs::auth::{NoVerifier, Token};
use spotify_rs::client::Client;
use spotify_rs::AuthCodeFlow;
use std::io::Write;
use std::path::PathBuf;

use crate::store::{self, create_stored_file_path};

/// Load all data regarding a specific playlist and write into a storage file. Returns the path of
/// the storage file
///
/// `playlist_id` - If None then the users saved tracks is the target playlist
pub async fn load_playlist_handler(
    client: &mut Client<Token, AuthCodeFlow, NoVerifier>,
    playlist_id: Option<String>,
    offset: u32,
    limit: Option<u32>,
) -> Result<PathBuf, EchoError> {
    let file_name = format!(
        "{ }.json",
        playlist_id
            .clone()
            .unwrap_or(String::from("user_liked_tracks"))
    );
    let mut stored_file = store::open_stored_file(&file_name, false, true)?;
    match playlist_id {
        Some(pid) => {
            let l = match limit {
                Some(l) => l,
                None => {
                    let playlist_data = client
                        .playlist(&pid)
                        .get()
                        .await
                        .map_err(|error| EchoError::ClientRequestError(error.to_string()))?;
                    playlist_data.tracks.total
                }
            };
            let mut spotify_params = SpotifyReadTracksParams::new(offset, l);

            while spotify_params.request_limit_exceeded() || spotify_params.request_required() {
                let next_limit = spotify_params.next_limit();
                let next_offset = spotify_params.offset;
                let playlist_data = client
                    .playlist_items(&pid)
                    .limit(next_limit)
                    .offset(next_offset)
                    .get()
                    .await
                    .map_err(|error| EchoError::ClientRequestError(error.to_string()))?;
                trace!(
                    "Requested playlist items | offset: {} | limit: {}",
                    next_offset,
                    next_limit
                );
                let _ = stored_file
                    .write_all(unsafe { any_as_u8_slice(&playlist_data) })
                    .map_err(|error| EchoError::IoStoredFileError(error.to_string()));
            }

            return Ok(create_stored_file_path(&file_name).unwrap());
        }
        None => {
            let l = match limit {
                Some(l) => l,
                None => {
                    let playlist_data = client
                        .saved_tracks()
                        .get()
                        .await
                        .map_err(|error| EchoError::ClientRequestError(error.to_string()))?;
                    playlist_data.total
                }
            };

            let mut spotify_params = SpotifyReadTracksParams::new(offset, l);

            while spotify_params.request_limit_exceeded() || spotify_params.request_required() {
                let next_limit = spotify_params.next_limit();
                let next_offset = spotify_params.offset;
                let playlist_data = client
                    .saved_tracks()
                    .limit(next_limit)
                    .offset(next_offset)
                    .get()
                    .await
                    .map_err(|error| EchoError::ClientRequestError(error.to_string()))?;
                trace!(
                    "Requested playlist items | offset: {} | limit: {}",
                    next_offset,
                    next_limit
                );
                let _ = stored_file
                    .write_all(unsafe { any_as_u8_slice(&playlist_data) })
                    .map_err(|error| EchoError::IoStoredFileError(error.to_string()));
            }

            return Ok(create_stored_file_path(&file_name).unwrap());
        }
    }
}
