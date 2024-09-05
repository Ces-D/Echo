use std::borrow::BorrowMut;

use log::{trace, warn};
use spotify_rs::auth::{NoVerifier, Token};
use spotify_rs::client::Client;
use spotify_rs::model::playlist::SimplifiedPlaylist;
use spotify_rs::model::Page;
use spotify_rs::{AuthCodeFlow, Error};

const SAVED_TRACKS_PLAYLIST_NAME: &str = "SaVeD TrAcKs";
const SAVED_TRACKS_PLAYLIST_DESCRIPTION: &str =
    "A duplicate of my liked tracks made public. Sharing is caring";
pub async fn duplicate_users_saved_tracks(
    mut client: Client<Token, AuthCodeFlow, NoVerifier>,
    user_id: String,
) -> Result<(), Error> {
    // check if the duplicated_likes playlist exists
    //
    // if it exists then sync it
    // -- makes the request to get the saved tracks
    //
    // if it does not exist then create it
    // -- create an empty playlist
    // -- makes the request to get the saved tracks
    // -- add each track id into this new playlist

    let playlists = client.current_user_playlists().limit(50).get().await?;
    match search_for_current_user_duplicate_liked_playlists(playlists) {
        Some(_liked_playlist) => Ok(()),
        None => {
            let playlist = client
                .create_playlist(user_id, SAVED_TRACKS_PLAYLIST_NAME)
                .description(SAVED_TRACKS_PLAYLIST_DESCRIPTION)
                .public(true)
                .send()
                .await?;
            let mut offset = 0;
            let mut has_more_likes = true;
            while has_more_likes {
                has_more_likes = add_liked_tracks_to_playlist(
                    client.borrow_mut(),
                    playlist.id.clone(),
                    offset,
                    50,
                )
                .await?;
                if has_more_likes {
                    offset += 50;
                }
                trace!(
                    "Added {} songs to {}{} playlist",
                    offset,
                    SAVED_TRACKS_PLAYLIST_NAME,
                    playlist.id.clone()
                )
            }

            Ok(())
        }
    }
}

async fn add_liked_tracks_to_playlist(
    client: &mut Client<Token, AuthCodeFlow, NoVerifier>,
    playlist_id: String,
    offset: u32,
    limit: u32,
) -> Result<bool, Error> {
    let private_likes = client
        .saved_tracks()
        .limit(limit)
        .offset(offset)
        .get()
        .await?;

    let liked_track_uris: Vec<String> = private_likes
        .items
        .into_iter()
        .map(|item| item.track.uri)
        .collect();
    client
        .add_items_to_playlist(playlist_id, &liked_track_uris)
        .send()
        .await?;
    Ok(private_likes.next.is_some())
}

fn search_for_current_user_duplicate_liked_playlists(
    playlists: Page<SimplifiedPlaylist>,
) -> Option<SimplifiedPlaylist> {
    if playlists.total < 50 {
        warn!(
            "{} playlist might be on a separate page due to exceeded playlist limit",
            SAVED_TRACKS_PLAYLIST_NAME
        );
    }
    playlists.items.into_iter().find(|playlist| {
        playlist.name == SAVED_TRACKS_PLAYLIST_NAME
            && playlist.description == Some(SAVED_TRACKS_PLAYLIST_DESCRIPTION.to_string())
    })
}
