use std::path::PathBuf;

use crate::cli::PlaylistCmp;
use crate::store;
use echo::error::EchoError;

use super::constants;

pub async fn compare_playlist_handler(
    playlist_id_a: Option<String>,
    playlist_id_b: Option<String>,
    cmp: PlaylistCmp,
) -> Result<PathBuf, EchoError> {
    // TODO(CES): playlist a should always be fetched and stored here on this command
    // playlist b should come from a file but thats located in echo_store
    // this will let us compare versions of playlists with their most recent versions. This takes
    // advantage of the diffing algoriths since we wont simply have inserts or equals. We may get
    // subtractions
    // But do I want this??
    // Actually yes I do. But it should compare by the date added in addition to the song itself.
    // Create a set of echo_ids and sort by the time added. Then compare the playlists
    let identifier_a = playlist_id_a
        .clone()
        .unwrap_or(constants::USERS_SAVED_TRACKS_STORE_FILE_PREFIX.to_string());
    let identifier_b = playlist_id_b
        .clone()
        .unwrap_or(constants::USERS_SAVED_TRACKS_STORE_FILE_PREFIX.to_string());
    let stored_a = store::stored_file_path(&identifier_a).unwrap();
    let stored_b = store::stored_file_path(&identifier_b).unwrap();

    if identifier_a == identifier_b {
        Err(EchoError::CliParamError(String::from(
            "You selected to compare the playlist with itself. This is not possible",
        )))
    } else if !stored_a.exists() || !stored_b.exists() {
        Err(EchoError::IoStoredFileError(format!(
            "One of the playlists could not be found in storage: {} | {}",
            stored_a.to_str().unwrap(),
            stored_b.to_str().unwrap(),
        )))
    } else {
        match cmp {
            PlaylistCmp::TrackItems => {
                todo!()
                //
                // song_id_a
                //            + song_id_b
                // song_id_c
                // song_id_d    song_id_d
                // song_id_e
                //
            }
        }
    }
}
