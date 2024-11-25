use crate::error::EchoError;
use echo_db::track::insert::MinimumSpotifyTrack;
use futures::TryStreamExt;
use rspotify::model::{PlayableItem, PlaylistId, PlaylistItem};
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
            let playlist_tracks = client
                .playlist_items(playlist_id, None, None)
                .try_filter_map(|x| async move {
                    if x.track.is_none() {
                        return Ok(None);
                    }
                    match x.track.unwrap() {
                        PlayableItem::Track(full_track) => Ok(Some(MinimumSpotifyTrack {
                            spotify_id: full_track.id,
                            name: full_track.name,
                            popularity: full_track.popularity,
                            duration_ms: full_track.duration_ms,
                        })),
                        PlayableItem::Episode(_) => Ok(None),
                    }
                });

            return Ok(());
        }
        None => {
            // Executing the futures concurrently
            let stream = client.current_user_saved_tracks(None);
            println!("\nItems (concurrent):");

            return Ok(());
        }
    }
}
