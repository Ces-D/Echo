// @generated automatically by Diesel CLI.

diesel::table! {
    album (id) {
        id -> Int4,
        #[max_length = 255]
        spotify_id -> Nullable<Varchar>,
        #[max_length = 255]
        name -> Varchar,
        release_date -> Nullable<Date>,
        #[max_length = 255]
        record_label -> Nullable<Varchar>,
    }
}

diesel::table! {
    album_track (track_id, album_id) {
        track_id -> Int4,
        album_id -> Int4,
    }
}

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
    artist_album (artist_id, album_id) {
        artist_id -> Int4,
        album_id -> Int4,
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
        public -> Nullable<Bool>,
    }
}

diesel::table! {
    playlist_track (playlist_id, track_id) {
        playlist_id -> Int4,
        track_id -> Int4,
    }
}

diesel::table! {
    track (id) {
        id -> Int4,
        #[max_length = 255]
        spotify_id -> Nullable<Varchar>,
        spotify_added_at -> Nullable<Timestamp>,
        #[max_length = 255]
        name -> Varchar,
        popularity -> Nullable<Int2>,
        #[max_length = 12]
        isrc -> Nullable<Varchar>,
        genres -> Array<Nullable<Text>>,
        acousticness -> Nullable<Int2>,
        danceability -> Nullable<Int2>,
        energy -> Nullable<Int2>,
        instrumentalness -> Nullable<Int2>,
        liveness -> Nullable<Int2>,
        tempo -> Nullable<Int2>,
        duration_ms -> Nullable<Int4>,
        chart_position -> Nullable<Int4>,
        streams -> Nullable<Int8>,
        sales -> Nullable<Int8>,
        user_rating -> Nullable<Int2>,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(album_track -> album (album_id));
diesel::joinable!(album_track -> track (track_id));
diesel::joinable!(artist_album -> album (album_id));
diesel::joinable!(artist_album -> artist (artist_id));
diesel::joinable!(artist_track -> artist (artist_id));
diesel::joinable!(artist_track -> track (track_id));
diesel::joinable!(playlist_track -> playlist (playlist_id));
diesel::joinable!(playlist_track -> track (track_id));

diesel::allow_tables_to_appear_in_same_query!(
    album,
    album_track,
    artist,
    artist_album,
    artist_track,
    playlist,
    playlist_track,
    track,
);
