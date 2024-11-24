CREATE TABLE track (
  id SERIAL PRIMARY KEY,
  spotify_id VARCHAR(255) UNIQUE,
  name VARCHAR(255) NOT NULL,
  popularity SMALLINT,
  isrc VARCHAR(12) UNIQUE,
  genres TEXT[] NOT NULL,
  acousticness SMALLINT,
  danceability SMALLINT,
  energy SMALLINT,
  instrumentalness SMALLINT,
  liveness SMALLINT,
  tempo SMALLINT,
  duration_ms INT,
  chart_position INT,
  streams BIGINT,
  sales BIGINT,
  user_rating SMALLINT,
  updated_at TIMESTAMP NOT NULL DEFAULT NOW ()
);

CREATE TABLE album (
  id SERIAL PRIMARY KEY,
  spotify_id VARCHAR(255) UNIQUE,
  name VARCHAR(255) NOT NULL,
  release_date DATE,
  record_label VARCHAR(255)
);

CREATE TABLE album_track (
  track_id INT REFERENCES track (id) ON DELETE CASCADE,
  album_id INT REFERENCES album (id),
  CONSTRAINT album_track_pkey PRIMARY KEY (track_id, album_id)
);

CREATE TABLE artist (
  id SERIAL PRIMARY KEY,
  spotify_id VARCHAR(255) UNIQUE,
  name VARCHAR(255) NOT NULL
);

CREATE TABLE artist_album (
  artist_id INT REFERENCES artist (id) ON DELETE CASCADE,
  album_id INT REFERENCES album (id),
  CONSTRAINT artist_album_pkey PRIMARY KEY (artist_id, album_id)
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
  public BOOLEAN
);

CREATE TABLE playlist_track (
  playlist_id INT REFERENCES playlist (id),
  track_id INT REFERENCES track (id) ON DELETE CASCADE,
  CONSTRAINT playlist_track_pkey PRIMARY KEY (playlist_id, track_id)
)
