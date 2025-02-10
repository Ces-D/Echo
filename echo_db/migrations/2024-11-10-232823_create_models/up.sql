CREATE TABLE track (
  id SERIAL PRIMARY KEY,
  spotify_track_id VARCHAR(255) UNIQUE,
  spotify_album_id VARCHAR(255) UNIQUE,
  album_name VARCHAR(255),
  name VARCHAR(255) NOT NULL,
  popularity INT,
  isrc VARCHAR(12) UNIQUE,
  genres TEXT[] NOT NULL,
  release_date DATE,
  acousticness SMALLINT,
  danceability SMALLINT,
  energy SMALLINT,
  instrumentalness SMALLINT,
  liveness SMALLINT,
  tempo SMALLINT,
  duration_ms BIGINT NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
);

CREATE TABLE artist (
  id SERIAL PRIMARY KEY,
  spotify_id VARCHAR(255) UNIQUE,
  name VARCHAR(255) NOT NULL
);

CREATE TABLE artist_track (
  artist_id INT REFERENCES artist (id),
  track_id INT REFERENCES track (id) ON DELETE CASCADE,
  CONSTRAINT artist_track_pkey PRIMARY KEY (artist_id, track_id)
);

CREATE TABLE playlist (
  id SERIAL PRIMARY KEY,
  spotify_id VARCHAR(255) UNIQUE,
  description VARCHAR(510),
  name VARCHAR(255),
  total_tracks INT NOT NULL DEFAULT 0,
  public BOOLEAN NOT NULL DEFAULT FALSE,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
);

CREATE TABLE playlist_track (
  playlist_id INT REFERENCES playlist (id),
  track_id INT REFERENCES track (id) ON DELETE CASCADE,
  spotify_added_at TIMESTAMPTZ,
  CONSTRAINT playlist_track_pkey PRIMARY KEY (playlist_id, track_id)
)
