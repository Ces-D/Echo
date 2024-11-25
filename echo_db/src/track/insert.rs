use crate::schema::track;
use diesel::prelude::Insertable;
use diesel::PgConnection;

#[derive(Insertable)]
#[diesel(table_name = track)]
pub struct MinimumSpotifyTrack {
    pub spotify_id: Option<String>,
    pub spotify_added_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub popularity: Option<i16>,
    pub duration_ms: Option<i32>,
}

/// Insert a minimum Spotify track into the database.
pub fn insert_minimum_spotify_track(
    connection: &mut PgConnection,
    data: &MinimumSpotifyTrack,
) -> Result<usize, diesel::result::Error> {
    diesel::insert_into(track::table)
        .values(data)
        .execute(connection)
}
