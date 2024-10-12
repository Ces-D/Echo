use echo::error::EchoError;
use futures::TryStreamExt;
use rspotify::model::PlaylistId;
use rspotify::prelude::{BaseClient, OAuthClient};
use rspotify::AuthCodeSpotify;
use std::path::PathBuf;

use crate::store::{open_stored_file, stored_file_path};

use super::constants;

/// Load all data regarding a specific playlist and write into a storage file. Returns the path of the storage file
///
/// `playlist_id` - If None then the users saved tracks is the target playlist
pub async fn load_playlist_handler(
    client: AuthCodeSpotify,
    playlist_id: Option<String>,
) -> Result<PathBuf, EchoError> {
    let identifier = playlist_id
        .clone()
        .unwrap_or(constants::USERS_SAVED_TRACKS_STORE_FILE_PREFIX.to_string());
    //let stored_file = open_stored_file(&identifier, false, true)?;
    match playlist_id {
        Some(pid) => {
            let playlist_id = PlaylistId::from_uri(&pid)
                .map_err(|error| EchoError::CliParamError(error.to_string()))?;
            let stream = client.playlist_items(playlist_id, None, None);
            println!("\nItems (concurrent):");
            stream
                .try_for_each_concurrent(10, |item| async move {
                    if let Some(i) = item.track {
                        match i {
                            rspotify::model::PlayableItem::Track(full_track) => {
                                println!("* {}", full_track.name)
                            }
                            rspotify::model::PlayableItem::Episode(full_episode) => {
                                println!("* {}", full_episode.name)
                            }
                        }
                    }
                    Ok(())
                })
                .await
                .unwrap();
            return Ok(stored_file_path(&identifier).unwrap());
        }
        None => {
            // Executing the futures concurrently
            let stream = client.current_user_saved_tracks(None);
            println!("\nItems (concurrent):");
            stream
                .try_for_each_concurrent(10, |item| async move {
                    println!("* {}", item.track.name);
                    Ok(())
                })
                .await
                .unwrap();
            return Ok(stored_file_path(&identifier).unwrap());
        }
    }
}
