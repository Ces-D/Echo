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
