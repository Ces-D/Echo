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
    Test {
        test: TestType,
    },
}

#[derive(ValueEnum, Clone, Copy)]
pub enum TestType {
    CreatePlaylist,
    FindPlaylist,
    AddTracksToPlaylist,
    LoadPlaylistTracks,
}
