// @generated automatically by Diesel CLI.

diesel::table! {
    person (id) {
        id -> Int4,
        #[max_length = 255]
        spotify_id -> Nullable<Varchar>,
        #[max_length = 255]
        name -> Varchar,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    persontrack (person_id, track_id) {
        person_id -> Int4,
        track_id -> Int4,
        added_at -> Nullable<Timestamptz>,
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

diesel::joinable!(persontrack -> person (person_id));
diesel::joinable!(persontrack -> track (track_id));

diesel::allow_tables_to_appear_in_same_query!(
    person,
    persontrack,
    track,
);
