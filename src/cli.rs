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

    #[command(about = "Load all the data about a specific playlist")]
    LoadPlaylist {
        #[arg(
            short,
            long,
            help = "Id of playlist. Users liked tracks are selected by default"
        )]
        playlist_id: Option<String>,
    },

    #[command(about = "Compare the data between two specific playlists")]
    ComparePlaylist {
        #[arg(
            short = 'a',
            long,
            help = "Id of playlist a. Users liked tracks are selected by default"
        )]
        playlist_id_a: Option<String>,
        #[arg(
            short = 'b',
            long,
            help = "Id of playlist b. Users liked tracks are selected by default"
        )]
        playlist_id_b: Option<String>,
        #[arg(short, long, help = "Method for comparing the playlists", default_value_t=PlaylistCmp::TrackItems)]
        cmp: PlaylistCmp,
    },

    #[command(about = "Search your playlists for something specific")]
    FindPlaylist { name: String },
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaylistCmp {
    TrackItems,
}
impl std::fmt::Display for PlaylistCmp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}
