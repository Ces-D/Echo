// @generated automatically by Diesel CLI.

diesel::table! {
    artist (id) {
        id -> Int4,
        #[max_length = 255]
        spotify_id -> Nullable<Varchar>,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    artist_track (artist_id, track_id) {
        artist_id -> Int4,
        track_id -> Int4,
    }
}

diesel::table! {
    playlist (id) {
        id -> Int4,
        #[max_length = 255]
        spotify_id -> Nullable<Varchar>,
        #[max_length = 510]
        description -> Nullable<Varchar>,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        total_tracks -> Int4,
        public -> Bool,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    playlist_track (playlist_id, track_id) {
        playlist_id -> Int4,
        track_id -> Int4,
        spotify_added_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    track (id) {
        id -> Int4,
        #[max_length = 255]
        spotify_track_id -> Nullable<Varchar>,
        #[max_length = 255]
        spotify_album_id -> Nullable<Varchar>,
        #[max_length = 255]
        album_name -> Nullable<Varchar>,
        #[max_length = 255]
        name -> Varchar,
        popularity -> Nullable<Int4>,
        #[max_length = 12]
        isrc -> Nullable<Varchar>,
        genres -> Array<Nullable<Text>>,
        release_date -> Nullable<Date>,
        acousticness -> Nullable<Int2>,
        danceability -> Nullable<Int2>,
        energy -> Nullable<Int2>,
        instrumentalness -> Nullable<Int2>,
        liveness -> Nullable<Int2>,
        tempo -> Nullable<Int2>,
        duration_ms -> Int8,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(artist_track -> artist (artist_id));
diesel::joinable!(artist_track -> track (track_id));
diesel::joinable!(playlist_track -> playlist (playlist_id));
diesel::joinable!(playlist_track -> track (track_id));

diesel::allow_tables_to_appear_in_same_query!(
    artist,
    artist_track,
    playlist,
    playlist_track,
    track,
);
