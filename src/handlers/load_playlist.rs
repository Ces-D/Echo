use echo::error::EchoError;
use echo::spotify::params::SpotifyReadTracksParams;
use log::trace;
use spotify_rs::auth::{NoVerifier, Token};
use spotify_rs::client::Client;
use spotify_rs::AuthCodeFlow;
use std::borrow::BorrowMut;
use std::io::Write;
use std::path::PathBuf;

// TODO: switch to https://github.com/ramsayleung/rspotify/blob/master/examples/tasks.rs
use crate::store::{self, stored_file_path};

use super::constants;

// FIXME: The loading writes the data as a buffer. Not Useful since we want json

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
    let identifier = playlist_id
        .clone()
        .unwrap_or(constants::USERS_SAVED_TRACKS_STORE_FILE_PREFIX.to_string());
    let mut stored_file = store::open_stored_file(&identifier, false, true)?;
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
                    playlist_data.tracks.total - offset
                }
            };
            let spotify_params = SpotifyReadTracksParams::new_async(offset, l);
            let mut handles = Vec::with_capacity(spotify_params.len());
            let (wr, mut rd) = tokio::sync::mpsc::unbounded_channel();
            let spotify = std::sync::Arc::new(client);
            for param in spotify_params {
                let spotify = std::sync::Arc::clone(&spotify);
                let wr = wr.clone();
                let handle = tokio::task::spawn(async move {
                    let items = spotify
                        .playlist_items(pid.clone())
                        .limit(param.limit)
                        .offset(param.offset)
                        .get()
                        .await;
                    wr.send(items).unwrap();
                });
                handles.push(handle);
            }
            drop(wr); // Automatically closed channel

            //while spotify_params.request_limit_exceeded() || spotify_params.request_required() {
            //    let next_limit = spotify_params.next_limit();
            //    let next_offset = spotify_params.offset;
            //    let playlist_data = client
            //        .playlist_items(&pid)
            //        .limit(next_limit)
            //        .offset(next_offset)
            //        .get()
            //        .await
            //        .map_err(|error| EchoError::ClientRequestError(error.to_string()))?;
            //    trace!(
            //        "Requested playlist items | offset: {} | limit: {}",
            //        next_offset,
            //        next_limit
            //    );
            //    let _ = stored_file
            //        .write(unsafe { any_as_u8_slice(&playlist_data) })
            //        .map_err(|error| EchoError::IoStoredFileError(error.to_string()));
            //}

            return Ok(stored_file_path(&identifier).unwrap());
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
                    playlist_data.total - offset
                }
            };

            let mut spotify_params = SpotifyReadTracksParams::new_async(offset, l);

            //while spotify_params.request_limit_exceeded() || spotify_params.request_required() {
            //    let next_limit = spotify_params.next_limit();
            //    let next_offset = spotify_params.offset;
            //    let playlist_data = client
            //        .saved_tracks()
            //        .limit(next_limit)
            //        .offset(next_offset)
            //        .get()
            //        .await
            //        .map_err(|error| EchoError::ClientRequestError(error.to_string()))?;
            //    trace!(
            //        "Requested playlist items | offset: {} | limit: {}",
            //        next_offset,
            //        next_limit
            //    );
            //    //let _ = stored_file
            //    //    .write_all(unsafe { any_as_u8_slice(&playlist_data) })
            //    //    .map_err(|error| EchoError::IoStoredFileError(error.to_string()));
            //}

            return Ok(stored_file_path(&identifier).unwrap());
        }
    }
}
