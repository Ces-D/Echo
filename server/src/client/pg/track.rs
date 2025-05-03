use crate::schema::track;
use diesel::Insertable;

#[derive(Insertable)]
#[diesel(table_name = track)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertableTrack {
    spotify_track_id: Option<String>,
    spotify_album_id: Option<String>,
    album_name: Option<String>,
    name: String,
    popularity: Option<i32>,
    isrc: Option<String>,
    genres: Vec<Option<String>>,
    release_date: Option<chrono::NaiveDate>,
    acousticness: Option<i16>,
    danceability: Option<i16>,
    energy: Option<i16>,
    instrumentalness: Option<i16>,
    liveness: Option<i16>,
    tempo: Option<i16>,
    duration_ms: i64,
    updated_at: chrono::NaiveDateTime,
}

