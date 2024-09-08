use std::borrow::BorrowMut;
use std::u32;

use log::{error, info, trace, warn};
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
        Some(liked_playlist) => {
            let private_starred_tracks_data = client.saved_tracks().limit(10).get().await?;
            let public_starred_tracks_data = liked_playlist
                .tracks
                .expect("Unable to Access public liked playlist tracks");
            if private_starred_tracks_data.total == public_starred_tracks_data.total {
                info!("You have no new songs in your playlist");
                Ok(())
            } else if private_starred_tracks_data.total < public_starred_tracks_data.total {
                error!("You have more tracks in the public playlist than in your private starred. Unable to continue without an equal number of songs");
                Ok(())
            } else {
                let mut offset = 0;
                let mut limit = 50;

                let mut playlists_track_delta =
                    private_starred_tracks_data.total - public_starred_tracks_data.total;
                let is_not_synced = playlists_track_delta > std::u32::MIN;
                while is_not_synced {
                    if playlists_track_delta < 50 {
                        limit = playlists_track_delta
                    }
                    let track_uris =
                        get_liked_track_uris(client.borrow_mut(), offset, limit).await?;
                    let addded = add_tracks_to_playlist(
                        client.borrow_mut(),
                        liked_playlist.id,
                        track_uris.track_uris,
                        true,
                    )
                    .await?;

                    playlists_track_delta -= limit;
                    if has_more_likes {
                        offset += limit
                    }
                    trace!(
                        "Added {} new songs. A total {} new songs to {}:{} playlist",
                        limit,
                        offset,
                        SAVED_TRACKS_PLAYLIST_NAME,
                        liked_playlist.id.clone()
                    );
                }

                Ok(())
            }
        }
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
                    false,
                )
                .await?;
                if has_more_likes {
                    offset += 50;
                }
                trace!(
                    "Added {} total songs to {}:{} playlist",
                    offset,
                    SAVED_TRACKS_PLAYLIST_NAME,
                    playlist.id.clone()
                )
            }

            Ok(())
        }
    }
}

/// TODO: REDO:
/// compare the total tracks in both playlists and the first 50 tracks in each playlist. If they
/// are not identical (via checksum) then we believe there is a difference.
/// - this is the case of added songs and removed songs (with exceptions)
/// load all the starred tracks and the songs in the public playlist and append the results in the tmp
/// files
/// Update the tmp file by creating a set using the name and artist and album
/// - handles duplication
/// Identify the differences, learn about diff algorithms
/// Once through the starred tracks, add any missing ones to the public liked tracks

async fn add_tracks_to_playlist(
    client: &mut Client<Token, AuthCodeFlow, NoVerifier>,
    playlist_id: String,
    track_uris: &Vec<String>,
    insert_at_playlist_start: bool,
) -> Result<String, Error> {
    if insert_at_playlist_start {
        client
            .add_items_to_playlist(playlist_id, track_uris)
            .position(0)
            .send()
            .await
    } else {
        client
            .add_items_to_playlist(playlist_id, track_uris)
            .position(0)
            .send()
            .await
    }
}

struct LikedTrackUris {
    track_uris: Vec<String>,
    has_more_likes: bool,
    request_offset: u32,
    request_limit: u32,
}
async fn get_liked_track_uris(
    client: &mut Client<Token, AuthCodeFlow, NoVerifier>,
    offset: u32,
    limit: u32,
) -> Result<LikedTrackUris, Error> {
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
    Ok(LikedTrackUris {
        track_uris: liked_track_uris,
        has_more_likes: private_likes.next.is_some(),
        request_limit: limit,
        request_offset: offset,
    })
}

fn search_for_current_user_duplicate_liked_playlists(
    playlists: Page<SimplifiedPlaylist>,
) -> Option<SimplifiedPlaylist> {
    if playlists.total > 50 {
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
