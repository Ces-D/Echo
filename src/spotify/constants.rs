pub const SCOPES: [&str; 4] = [
    "user-library-read",
    "playlist-read-private",
    "playlist-modify-private",
    "playlist-modify-public",
];

pub const SAVED_TRACKS_PLAYLIST_NAME: &str = "SaVeD TrAcKs";
pub const SAVED_TRACKS_PLAYLIST_DESCRIPTION: &str =
    "A duplicate of my liked tracks made public. Sharing is caring";

pub const SPOTIFY_TRACKS_LIMIT: u32 = 50;
pub const SPOTIFY_URIS_LIMIT: usize = 100;
pub const SPOTIFY_PLAYLISTS_LIMIT: u32 = 50;

pub const TEST_PLAYLIST_NAME: &str = "Test Playlist Ignore";
pub const TEST_PLAYLIST_DESCRIPTION: &str =
    "A test playlist used for program. Will contain random music and episodes";
