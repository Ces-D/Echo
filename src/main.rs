use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;
use echo;
use log::{error, info};
use rspotify::prelude::OAuthClient;
use std::borrow::BorrowMut;
use std::error::Error;
use std::io::Write;

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

    let config = echo::spotify::client::read_config_from_env();
    let client = echo::spotify::client::create_client(&config);

    // Obtaining the access token
    let url = client.get_authorize_url(false).unwrap();
    // This function requires the `cli` feature enabled.
    client
        .prompt_for_token(&url)
        .await
        .expect("Couln't authenticate successfully");

    match app.command {
        Commands::LikedPlaylist => todo!(),

        Commands::LoadPlaylist { playlist_id } => {
            match handlers::load_playlist::load_playlist_handler(client, playlist_id).await {
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
            cmp,
        } => {
            match handlers::compare_playlist::compare_playlist_handler(
                client,
                playlist_id_a,
                playlist_id_b,
                cmp,
            )
            .await
            {
                Ok(success) => {
                    info!("The playlist comparison has completed. You can view the data here:",);
                    let stdout = std::io::stdout(); // get the global stdout entity
                    let mut handle = stdout.lock(); // acquire a lock on it

                    writeln!(handle, "{}", success.to_str().unwrap().on_bright_red())?;
                }
                Err(error) => error!("{}", error),
            }
        }

        Commands::FindPlaylist { name } => {
            match handlers::find_playlist::find_playlist_handler(client, name).await {
                Ok(summarized_playlists) => {
                    if summarized_playlists.len() == 0 {
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
                            writeln!(handle, "")?;
                        }
                    }
                }
                Err(error) => error!("{}", error),
            }
        }
    }
    Ok(())
}
