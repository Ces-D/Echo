use std::error::Error;

use clap::Parser;
use cli::{Cli, Commands};
use log::error;

mod cli;
mod config;
mod spotify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Cli::parse();

    let mut trace_builder = colog::default_builder();
    if app.trace {
        trace_builder.filter_level(log::LevelFilter::Trace);
    }
    trace_builder.init();

    let config = config::read_user_config();
    let spotify = spotify::client::initialize_client(config);
    let mut client = spotify.await?;
    let user = client.get_current_user_profile().await?;

    match app.command {
        Commands::LikedPlaylist => {
            match spotify::tracks::duplicate_users_saved_tracks(client, user.id).await {
                Ok(_) => {
                    log::info!("Success")
                }
                Err(error) => {
                    error!("{}", error)
                }
            }
        }
        Commands::Create {
            library,
            tempo,
            energy,
            instrumentalness,
            valence,
            popularity,
            name,
        } => match library {
            cli::LibraryType::Playlist => {
                // TODO: identify all the tracks necessary for this playlist or queue
                // Create the playlist and add the tracks to it
                let playlst = client
                    .create_playlist(
                        user.id,
                        name.unwrap_or(String::from("New Playlist Needs Better Name")),
                    )
                    .description(String::from(
                        "A new playlist created by the powers of technology",
                    ))
                    .send()
                    .await?;
            }

            cli::LibraryType::Queue => {}
        },
    }

    Ok(())
}
