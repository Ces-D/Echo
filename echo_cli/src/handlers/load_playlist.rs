use std::borrow::BorrowMut;

use crate::error::EchoError;
use echo_db::crud::insert::{
    insert_spotify_playlist_transaction, insert_spotify_track_transaction, spotify,
};
use futures::{StreamExt, TryStreamExt};
use rspotify::model::{FullTrack, PlayableItem, PlaylistId, SavedTrack};
use rspotify::prelude::{BaseClient, OAuthClient};
use rspotify::{AuthCodeSpotify, ClientError};

/// Load all data regarding a specific playlist and store it.
///
/// `playlist_id` - If None then the users saved tracks is the target playlist
pub async fn load_playlist_handler(
    client: AuthCodeSpotify,
    playlist_id: Option<String>,
) -> Result<(), EchoError> {
    let mut db_conn = echo_db::connection::establish_connection();

    match playlist_id {
        Some(pid) => {
            let playlist_id = PlaylistId::from_uri(&pid)?;
            let playlist_information = client.playlist(playlist_id.clone(), None, None).await?;
            let playlist = insert_spotify_playlist_transaction(
                db_conn.borrow_mut(),
                spotify::Playlist {
                    spotify_id: Some(playlist_information.id.to_string()),
                    description: playlist_information.description,
                    name: Some(playlist_information.name),
                    public: playlist_information.public,
                    total_tracks: Some(playlist_information.tracks.total as i32),
                },
            )
            .map_err(|error| EchoError::DatabaseError(error.to_string()))?;

            let streamed_information = client
                .playlist_items(playlist_id, None, None)
                .try_filter_map(|playlist_item| async move {
                    if playlist_item.track.is_some() {
                        let item = match playlist_item.track.unwrap() {
                            PlayableItem::Track(full_track) => {
                                Some(gather_information_from_full_track(
                                    full_track,
                                    playlist_item.added_at,
                                ))
                            }
                            PlayableItem::Episode(_) => None,
                        };
                        Ok(item)
                    } else {
                        Ok(None)
                    }
                })
                .collect::<Vec<Result<GatheredInformation, ClientError>>>()
                .await;

            for gathered_information in streamed_information {
                match gathered_information {
                    Ok(i) => insert_spotify_track_transaction(
                        db_conn.borrow_mut(),
                        i.0,
                        i.1,
                        i.2,
                        playlist.id,
                    )
                    .map_err(|error| EchoError::DatabaseError(error.to_string()))?,
                    Err(error) => {
                        eprintln!("Error: {}", error)
                    }
                }
            }

            Ok(())
        }

        None => {
            let saved_information = client
                .current_user_saved_tracks(None)
                .map(|saved_track| gather_information_from_saved_track(saved_track.unwrap()))
                .collect::<Vec<GatheredInformation>>()
                .await;
            let saved_tracks_playlist = spotify::Playlist {
                spotify_id: None,
                description: Some(String::from(
                    "A list of the songs saved in the current Spotify user's 'Your Music' library.",
                )),
                name: Some(String::from("Saved Tracks")),
                public: None,
                total_tracks: Some(saved_information.len() as i32),
            };
            match insert_spotify_playlist_transaction(db_conn.borrow_mut(), saved_tracks_playlist) {
                Ok(saved_tracks_playlist) => {
                    for gathered_information in saved_information {
                        if let Err(error) = insert_spotify_track_transaction(
                            db_conn.borrow_mut(),
                            gathered_information.0,
                            gathered_information.1,
                            gathered_information.2,
                            saved_tracks_playlist.id,
                        ) {
                            return Err(EchoError::DatabaseError(error.to_string()));
                        }
                    }
                }
                Err(error) => return Err(EchoError::DatabaseError(error.to_string())),
            }

            Ok(())
        }
    }
}

type GatheredInformation = (
    spotify::MinimumTrack,
    Vec<spotify::MinimumArtist>,
    Option<chrono::DateTime<chrono::Utc>>,
);

fn gather_information_from_full_track(
    track: FullTrack,
    added_at: Option<chrono::DateTime<chrono::Utc>>,
) -> GatheredInformation {
    let release_date = match track.album.release_date {
        Some(date) => chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d").ok(),
        None => None,
    };
    (
        spotify::MinimumTrack {
            spotify_track_id: track.id.map(|x| x.to_string()),
            spotify_album_id: track.album.id.map(|x| x.to_string()),
            album_name: Some(track.album.name),
            name: track.name,
            release_date,
            popularity: track.popularity as i32,
            duration_ms: track.duration.num_milliseconds(),
        },
        track
            .artists
            .into_iter()
            .map(|artist| spotify::MinimumArtist {
                spotify_id: artist.id.map(|x| x.to_string()),
                name: artist.name,
            })
            .collect(),
        added_at,
    )
}
fn gather_information_from_saved_track(track: SavedTrack) -> GatheredInformation {
    let release_date = match track.track.album.release_date {
        Some(date) => chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d").ok(),
        None => None,
    };
    (
        spotify::MinimumTrack {
            spotify_track_id: track.track.id.map(|x| x.to_string()),
            spotify_album_id: track.track.album.id.map(|x| x.to_string()),
            album_name: Some(track.track.album.name),
            release_date,
            name: track.track.name,
            popularity: track.track.popularity as i32,
            duration_ms: track.track.duration.num_milliseconds(),
        },
        track
            .track
            .artists
            .into_iter()
            .map(|artist| spotify::MinimumArtist {
                spotify_id: artist.id.map(|x| x.to_string()),
                name: artist.name,
            })
            .collect(),
        Some(track.added_at),
    )
}
