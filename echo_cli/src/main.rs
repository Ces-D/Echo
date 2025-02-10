use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;
use log::{error, info};
use rspotify::prelude::OAuthClient;
use rspotify::AuthCodeSpotify;
use std::error::Error;
use std::io::Write;

mod cli;
mod error;
mod handlers;
mod spotify;

async fn prepare_spotify_client() -> AuthCodeSpotify {
    let config = spotify::client::read_config_from_env();
    let client = spotify::client::create_client(&config);

    // Obtaining the access token
    let url = client.get_authorize_url(false).unwrap();
    // This function requires the `cli` feature enabled.
    client
        .prompt_for_token(&url)
        .await
        .expect("Couln't authenticate successfully");
    client
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Cli::parse();

    let mut trace_builder = colog::default_builder();
    if app.trace {
        trace_builder.filter_level(log::LevelFilter::Trace);
    }
    trace_builder.init();

    match app.command {
        Commands::Liked => todo!(),

        Commands::Load { playlist_id } => {
            let client = prepare_spotify_client().await;
            match handlers::load_playlist::load_playlist_handler(client, playlist_id).await {
                Ok(_) => {
                    info!("The playlist has been completely loaded")
                }
                Err(error) => error!("{}", error),
            }
        }

        Commands::Loaded => {
            match handlers::loaded_playlists::loaded_playlists_handler().await {
                Ok(playlists) => {
                    if playlists.is_empty() {
                        info!("No playlists have been loaded")
                    } else {
                        info!("Loaded Playlists: {}", playlists.len());
                        let stdout = std::io::stdout(); // get the global stdout entity
                        let mut handle = stdout.lock(); // acquire a lock on it
                        for playlist in playlists {
                            writeln!(
                                handle,
                                "{:<10}{}",
                                "Name".green(),
                                playlist.name.unwrap_or(String::from("N/A"))
                            )?;
                            writeln!(handle, "{:<10}{}", "Public".green(), playlist.public)?;
                            writeln!(handle, "{:<10}{}", "Track #".green(), playlist.total_tracks)?;
                            writeln!(handle, "{:<10}{}", "Id".green(), playlist.id)?;
                            writeln!(handle, " ")?;
                        }
                    }
                }
                Err(error) => error!("{}", error),
            }
        }

        Commands::Find { name } => {
            let client = prepare_spotify_client().await;
            match handlers::find_playlist::find_playlist_handler(client, name).await {
                Ok(summarized_playlists) => {
                    if summarized_playlists.is_empty() {
                        info!("Did not find a single match")
                    } else {
                        info!("Found Several Matches: {}", summarized_playlists.len());
                        let stdout = std::io::stdout(); // get the global stdout entity
                        let mut handle = stdout.lock(); // acquire a lock on it
                        for playlist in summarized_playlists {
                            writeln!(handle, "{:<10}{}", "Name".green(), playlist.name)?;
                            writeln!(handle, "{:<10}{}", "Public".green(), playlist.public)?;
                            writeln!(handle, "{:<10}{}", "Track #".green(), playlist.total)?;
                            writeln!(handle, "{:<10}{}", "Id".green(), playlist.id)?;
                            writeln!(handle, " ")?;
                        }
                    }
                }
                Err(error) => error!("{}", error),
            }
        }

        Commands::Generate { prompt } => {
            let client = prepare_spotify_client().await;
            todo!();
        }
    }
    Ok(())
}
