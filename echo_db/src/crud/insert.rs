use crate::schema;
use diesel::prelude::*;
use diesel::{Connection, PgConnection, QueryResult};

/// Gathered information provided by the Spotify API.
pub mod spotify {
    use crate::schema;
    use diesel::prelude::Insertable;

    #[derive(Insertable)]
    #[diesel(table_name = schema::track)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct MinimumTrack {
        pub spotify_track_id: Option<String>,
        pub spotify_album_id: Option<String>,
        pub album_name: Option<String>,
        pub release_date: Option<chrono::NaiveDate>,
        pub name: String,
        pub popularity: i32,
        pub duration_ms: i64,
    }

    #[derive(Insertable)]
    #[diesel(table_name = schema::playlist_track)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct PlaylistTrack {
        pub playlist_id: i32,
        pub track_id: i32,
        pub spotify_added_at: Option<chrono::DateTime<chrono::Utc>>,
    }

    #[derive(Insertable)]
    #[diesel(table_name = schema::playlist)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct Playlist {
        /// Optional spotify_id for Starred Tracks which is a sort of playlist but without an id
        pub spotify_id: Option<String>,
        pub description: Option<String>,
        pub name: Option<String>,
        pub public: Option<bool>,
        pub total_tracks: Option<i32>,
    }

    #[derive(Insertable)]
    #[diesel(table_name = schema::artist)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct MinimumArtist {
        pub spotify_id: Option<String>,
        pub name: String,
    }

    #[derive(Insertable)]
    #[diesel(table_name = schema::artist_track)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct ArtistTrack {
        pub artist_id: i32,
        pub track_id: i32,
    }
}

pub fn insert_spotify_playlist_transaction(
    connection: &mut PgConnection,
    playlist: self::spotify::Playlist,
) -> QueryResult<super::read::echo::Playlist> {
    diesel::insert_into(schema::playlist::table)
        .values(playlist)
        .returning(super::read::echo::Playlist::as_select())
        .get_result(connection)
}

pub fn insert_spotify_track_transaction(
    connection: &mut PgConnection,
    track: self::spotify::MinimumTrack,
    artists: Vec<self::spotify::MinimumArtist>,
    added_at: Option<chrono::DateTime<chrono::Utc>>,
    // The db playlist id
    playlist_id: i32,
) -> Result<(), diesel::result::Error> {
    connection.transaction(|connection| {
        let track = diesel::insert_into(schema::track::table)
            .values(track)
            .returning(super::read::echo::Track::as_select())
            .get_result(connection)?;
        let artists = diesel::insert_into(schema::artist::table)
            .values(artists)
            .returning(super::read::echo::Artist::as_select())
            .get_results(connection)?;
        diesel::insert_into(schema::playlist_track::table)
            .values(self::spotify::PlaylistTrack {
                playlist_id,
                track_id: track.id,
                spotify_added_at: added_at,
            })
            .execute(connection)?;
        let artist_tracks = artists
            .into_iter()
            .map(|artist| self::spotify::ArtistTrack {
                artist_id: artist.id,
                track_id: track.id,
            })
            .collect::<Vec<self::spotify::ArtistTrack>>();
        diesel::insert_into(schema::artist_track::table)
            .values(artist_tracks)
            .execute(connection)?;
        Ok(())
    })
}
