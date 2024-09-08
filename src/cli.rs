use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about="A tool to manipulate your Spotify music", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(short, long, help = "Set log level to trace", global = true)]
    pub trace: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(
        about = "Either create a public replica of your Starred Music or update the current existing replica"
    )]
    LikedPlaylist,
    Create {
        library: LibraryType,
        tempo: Option<u8>,
        energy: Option<u8>,
        instrumentalness: Option<u8>,
        valence: Option<u8>,
        popularity: Option<u8>,
        name: Option<String>,
        // https://developer.spotify.com/documentation/web-api/reference/get-recommendations
    },
}

#[derive(ValueEnum, Clone, Copy)]
pub enum LibraryType {
    Playlist,
    Queue,
}
