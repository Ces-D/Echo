use clap::{Parser, Subcommand};

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
    Liked,

    #[command(about = "Load all the data about a specific playlist")]
    Load {
        #[arg(
            short,
            long,
            help = "Id of playlist. Users liked tracks are selected by default",
            default_missing_value = None,
            required = false
        )]
        playlist_id: Option<String>,
    },

    #[command(about = "Retrieve details on all the loaded playlists")]
    Loaded,

    Generate {
        #[arg(
            short,
            long,
            help = "Describe what kind of filtering this playlist will consider when generating the playlist",
            long_help = "Ex. 'only tracks that are not in the original playlist and are from the 80s' or 'copy the original playlist' or 'only songs that will make me dance'. The tracks will be read from the playlists that have been loaded.",
            required = true
        )]
        prompt: String,
    },

    #[command(about = "Search your playlists for something specific")]
    Find { name: String },
}
