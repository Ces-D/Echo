use std::borrow::BorrowMut;
use std::error::Error;

use clap::Parser;
use cli::{Cli, Commands};
use echo;
use echo::spotify::constants::TEST_PLAYLIST_NAME;
use echo::spotify::params::SpotifyAddItemsParams;
use echo::spotify::playlist::{
    AddItemsToPlaylistParams, CreatePlaylistParams, FindPlaylistUsingIdentifiersParams,
};
use log::{error, info};

mod cli;
mod handlers;
mod store;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Cli::parse();

    let mut trace_builder = colog::default_builder();
    if app.trace {
        trace_builder.filter_level(log::LevelFilter::Trace);
    }
    trace_builder.init();

    let config = echo::config::read_user_config();
    let spotify = echo::spotify::client::initialize_client(config);
    let mut client = spotify.await?;
    let user = client.get_current_user_profile().await?;

    match app.command {
        Commands::LikedPlaylist => todo!(),

        Commands::LoadPlaylist {
            playlist_id,
            offset,
            limit,
        } => {
            match handlers::load_playlist::load_playlist_handler(
                client.borrow_mut(),
                playlist_id,
                offset,
                limit,
            )
            .await
            {
                Ok(success) => {
                    info!(
                        "The playlist has been completely loaded. You can view the data here: {}",
                        success.to_str().unwrap()
                    )
                }
                Err(error) => error!("{}", error),
            }
        }

        Commands::ComparePlaylist {
            playlist_id_a,
            playlist_id_b,
            offset,
            limit,
            cmp,
        } => {
            match handlers::compare_playlist::compare_playlist_handler(
                client.borrow_mut(),
                playlist_id_a,
                playlist_id_b,
                offset,
                limit,
                cmp,
            )
            .await
            {
                Ok(success) => {
                    info!(
                        "The playlist comparison has completed. You can view the data here: {}",
                        success.to_str().unwrap()
                    )
                }
                Err(error) => error!("{}", error),
            }
        }

        Commands::Test { test, playlist_id } => match test {
            cli::TestType::CreatePlaylist => {
                match echo::spotify::playlist::create_playlist(
                    client.borrow_mut(),
                    CreatePlaylistParams {
                        name: echo::spotify::constants::TEST_PLAYLIST_NAME.to_string(),
                        description: echo::spotify::constants::TEST_PLAYLIST_DESCRIPTION
                            .to_string(),
                        user_id: user.id,
                    },
                )
                .await
                {
                    Ok(playlist) => {
                        info!(
                            "{} created test playlist {}: {}",
                            user.display_name.unwrap_or(String::from("You")),
                            TEST_PLAYLIST_NAME,
                            playlist.id
                        );
                    }
                    Err(error) => error!("{}", error),
                }
            }
            cli::TestType::FindPlaylist => {
                match echo::spotify::playlist::find_playlist_using_identifiers(
                    client.borrow_mut(),
                    FindPlaylistUsingIdentifiersParams {
                        name: echo::spotify::constants::TEST_PLAYLIST_NAME.to_string(),
                        description: Some(
                            echo::spotify::constants::TEST_PLAYLIST_DESCRIPTION.to_string(),
                        ),
                    },
                )
                .await
                {
                    Ok(playlist) => {
                        info!("Located test playlist {}: {}", playlist.name, playlist.id)
                    }
                    Err(error) => error!("{}", error),
                }
            }
            cli::TestType::AddTracksToPlaylist => {
                if playlist_id.is_none() {
                    error!("Playlist id required");
                }
                let uris: Vec<String> = vec![
                    String::from("spotify:track:7EdPWyTm6EtO5httz2Dcoa"), // Hollow - Morten, Artbat
                ];
                // TODO: LoadPlaylist needs to work and store the result in a file so that we
                // can use the tracks for this test
                let mut params = AddItemsToPlaylistParams {
                    playlist_id: playlist_id.unwrap(),
                    spotify: SpotifyAddItemsParams::new(uris, None),
                };
                match echo::spotify::playlist::add_tracks_to_playlist(
                    client.borrow_mut(),
                    params.borrow_mut(),
                )
                .await
                {
                    Ok(_) => todo!(),
                    Err(_) => todo!(),
                }
            }
        },
    };

    Ok(())
}
