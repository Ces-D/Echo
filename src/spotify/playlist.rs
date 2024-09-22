use spotify_rs::auth::{NoVerifier, Token};
use spotify_rs::client::Client;
use spotify_rs::{AuthCodeFlow, Error};
use tempfile::NamedTempFile;

use super::cache::create_app_temp_file;

struct SpotifyTracksParams {
    offset: u32,
    limit: u32,
}
impl Default for SpotifyTracksParams {
    fn default() -> Self {
        SpotifyTracksParams {
            offset: 0,
            limit: 50,
        }
    }
}

struct LoadTracksParams {
    playlist_id: String,
    spotify: Option<SpotifyTracksParams>,
}
///  Load tracks from a playlist
async fn load_playlist_tracks(
    client: &mut Client<Token, AuthCodeFlow, NoVerifier>,
    params: LoadTracksParams,
) -> Option<NamedTempFile> {
    let playlist_tmp_file = create_app_temp_file(&params.playlist_id)?;

    match params.spotify {
        Some(_) => {
            // TODO: request the playlist tracks at the params then store the result
            todo!()

        }
        None => {
            //TODO: this should iterate through all tracks saving into tmp file as it goes along
            todo!()
        }
    }
}

/// Load tracks from the users starred playlist
async fn load_starred_playlist_tracks(
    client: &mut Client<Token, AuthCodeFlow, NoVerifier>,
    params: Option<SpotifyTracksParams>,
) -> Option<NamedTempFile> {
    match params {
        Some(_) => todo!(),
        None => todo!(),
    }
}
