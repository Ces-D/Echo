use crate::schema;
use diesel::{PgConnection, RunQueryDsl};

pub mod echo {
    use crate::schema;
    use diesel::{prelude::Queryable, Selectable};

    #[derive(Queryable, Selectable)]
    #[diesel(table_name = schema::track)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct Track {
        pub id: i32,
        pub spotify_track_id: Option<String>,
        pub spotify_album_id: Option<String>,
        pub album_name: Option<String>,
        pub name: String,
        pub popularity: Option<i32>,
        pub isrc: Option<String>,
        pub genres: Vec<Option<String>>,
        pub acousticness: Option<i16>,
        pub danceability: Option<i16>,
        pub energy: Option<i16>,
        pub instrumentalness: Option<i16>,
        pub liveness: Option<i16>,
        pub tempo: Option<i16>,
        pub duration_ms: i64,
        pub updated_at: chrono::DateTime<chrono::Utc>,
    }

    #[derive(Queryable, Selectable)]
    #[diesel(table_name = schema::playlist)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct Playlist {
        pub id: i32,
        pub spotify_id: Option<String>,
        pub description: Option<String>,
        pub name: Option<String>,
        pub total_tracks: i32,
        pub public: bool,
        pub updated_at: chrono::DateTime<chrono::Utc>,
    }

    #[derive(Queryable, Selectable)]
    #[diesel(table_name = schema::artist)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct Artist {
        pub id: i32,
        pub spotify_id: Option<String>,
        pub name: String,
    }
}

pub fn read_playlists(
    connection: &mut PgConnection,
) -> Result<Vec<echo::Playlist>, diesel::result::Error> {
    use schema::playlist::dsl::*;
    playlist.load::<self::echo::Playlist>(connection)
}
