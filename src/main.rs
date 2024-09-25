use std::borrow::BorrowMut;
use std::error::Error;

use clap::Parser;
use cli::{Cli, Commands};
use echo;
use echo::spotify::constants::TEST_PLAYLIST_NAME;
use echo::spotify::playlist::CreatePlaylistParams;
use log::{error, info};

mod cli;

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

        Commands::Test { test } => match test {
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
                    Err(error) => {
                        error!("{}", error)
                    }
                }
            }
            cli::TestType::FindPlaylist => todo!(),
            cli::TestType::AddTracksToPlaylist => todo!(),
            cli::TestType::LoadPlaylistTracks => todo!(),
        },
    };

    Ok(())
}
