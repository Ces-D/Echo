use crate::error::EchoError;
use futures::TryStreamExt;
use rspotify::model::PlaylistId;
use rspotify::prelude::{BaseClient, OAuthClient};
use rspotify::AuthCodeSpotify;

/// Load all data regarding a specific playlist and write into a storage file. Returns the path of the storage file
///
/// `playlist_id` - If None then the users saved tracks is the target playlist
pub async fn load_playlist_handler(
    client: AuthCodeSpotify,
    playlist_id: Option<String>,
) -> Result<(), EchoError> {
    let db_conn = echo_db::connection::establish_connection();

    match playlist_id {
        Some(pid) => {
            let playlist_id = PlaylistId::from_uri(&pid)
                .map_err(|error| EchoError::CliParamError(error.to_string()))?;
            let stream = client.playlist_items(playlist_id, None, None);
            stream
                .try_for_each_concurrent(10, |item| {
                    async move {
                        if let Some(i) = item.track {
                            match i {
                                rspotify::model::PlayableItem::Track(full_track) => {
                                    let record: EchoFullTrack = full_track.into();
                                }
                                rspotify::model::PlayableItem::Episode(full_episode) => {
                                    println!("* {}", full_episode.name)
                                    // TODO(CES): create a separate file for tracks data
                                    // and another for episodes
                                }
                            }
                        }
                        Ok(())
                    }
                })
                .await
                .unwrap();
            return Ok(());
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
            return Ok(());
        }
    }
}
