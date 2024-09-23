use super::constants::{SPOTIFY_TRACKS_LIMIT, SPOTIFY_URIS_LIMIT};

pub struct SpotifyTracksParams {
    pub offset: u32,
    limit: u32,
    remaining: bool,
}
impl SpotifyTracksParams {
    pub fn new(offset: u32, limit: u32) -> Self {
        SpotifyTracksParams {
            offset,
            limit,
            remaining: limit > 0,
        }
    }

    /// Indicates that a request can be started due to a limit > 0.
    pub fn request_required(&self) -> bool {
        self.remaining
    }

    /// Indicates when the request loop can be started due to exceeded remaining limit.
    /// Behaves as a `do; while;` when paired alongside `request_required`
    pub fn request_limit_exceeded(&self) -> bool {
        self.limit > SPOTIFY_TRACKS_LIMIT
    }

    /// The limit param for the next iteration of requests
    pub fn next_limit(&mut self) -> u32 {
        if self.request_limit_exceeded() {
            self.limit -= SPOTIFY_TRACKS_LIMIT;
            self.offset += SPOTIFY_TRACKS_LIMIT; // the next request will need to skip the first 50 requested
            if self.limit == 0 {
                self.remaining = false
            }
            SPOTIFY_TRACKS_LIMIT
        } else {
            self.remaining = false; // the next request should be the final
            self.limit
        }
    }
}
impl Default for SpotifyTracksParams {
    fn default() -> Self {
        SpotifyTracksParams {
            offset: 0,
            limit: SPOTIFY_TRACKS_LIMIT,
            remaining: true,
        }
    }
}

pub struct SpotifyAddItemsParams {
    uris: Vec<String>,
    pub position: Option<u32>,
    remaining: bool,
}
impl SpotifyAddItemsParams {
    pub fn new(uris: Vec<String>, position: Option<u32>) -> Self {
        SpotifyAddItemsParams {
            remaining: uris.len() > 0,
            position,
            uris,
        }
    }
    /// Indicates that a request can be started due to uris.len > 0.
    pub fn request_required(&self) -> bool {
        self.remaining
    }

    /// Indicates when the request loop can be started due to exceeded uri length.
    /// Behaves as a `do; while;` when paired alongside `request_required`
    pub fn request_limit_exceeded(&self) -> bool {
        self.uris.len() > SPOTIFY_URIS_LIMIT
    }

    pub fn next_items(&mut self) -> Vec<String> {
        if self.request_limit_exceeded() {
            let sub_items: Vec<String> = self.uris.drain(0..SPOTIFY_URIS_LIMIT).collect(); // Remove first 100
            if self.position.is_some() {
                // needed in order to add to the bottom of what was
                // previously added i
                self.position =
                    Some(self.position.unwrap() + u32::try_from(SPOTIFY_URIS_LIMIT).unwrap());
            }
            if self.uris.len() == 0 {
                self.remaining = false
            }
            sub_items
        } else {
            self.remaining = false;
            self.uris.clone()
        }
    }
}

// TODO: test these before running. The iterations should return expected limits and uris
