use std::path::PathBuf;

use crate::cli::PlaylistCmp;
use crate::store;
use echo::error::EchoError;
use rspotify::AuthCodeSpotify;

use super::constants;

pub async fn compare_playlist_handler(
    client: AuthCodeSpotify,
    playlist_id_a: Option<String>,
    playlist_id_b: Option<String>,
    cmp: PlaylistCmp,
) -> Result<PathBuf, EchoError> {
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
        todo!()
    }
}
