CREATE TABLE persontrack (
  person_id INT NOT NULL,
  track_id INT NOT NULL,
  added_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (person_id, track_id),
  FOREIGN KEY (person_id) REFERENCES person(id) ON DELETE CASCADE,
  FOREIGN KEY (track_id) REFERENCES track(id) ON DELETE CASCADE
)
