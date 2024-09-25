use super::constants::{SPOTIFY_TRACKS_LIMIT, SPOTIFY_URIS_LIMIT};

pub struct SpotifyReadTracksParams {
    pub offset: u32,
    limit: u32,
    remaining: bool,
}
impl SpotifyReadTracksParams {
    pub fn new(offset: u32, limit: u32) -> Self {
        SpotifyReadTracksParams {
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
impl Default for SpotifyReadTracksParams {
    fn default() -> Self {
        SpotifyReadTracksParams {
            offset: 0,
            limit: SPOTIFY_TRACKS_LIMIT,
            remaining: true,
        }
    }
}

pub struct SpotifyAddItemsParams {
    pub position: Option<u32>,
    uris: Vec<String>,
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

#[cfg(test)]
mod spotify_params_tests {
    use super::*;

    #[test]
    fn read_track_params_while_loop() {
        let mut params = SpotifyReadTracksParams::new(0, 103);
        assert!(
            params.request_limit_exceeded() && params.request_required(),
            "Request should be exceeded and required"
        );
        assert!(
            params.next_limit() == SPOTIFY_TRACKS_LIMIT,
            "We can only request the spotify limit at a time"
        );
        params.next_limit();
        assert!(
            params.offset == 100,
            "We should have added 50 twice, once per `next_limit` call"
        );
        let n = params.next_limit();
        assert_eq!(
            params.request_limit_exceeded(),
            false,
            "The remaining limit is 3"
        );
        assert!(n == 3, "The remaining limit should be 3");
        assert_eq!(params.request_required(), false);
    }

    #[test]
    fn add_items_params_while_loop() {
        let uris: Vec<String> = (0..150).map(|val| val.to_string()).collect();
        let mut params = SpotifyAddItemsParams::new(uris, None);
        assert!(
            params.request_limit_exceeded() && params.request_required(),
            "Request should be exceeded and required"
        );
        assert!(
            params.next_items().len() == SPOTIFY_URIS_LIMIT,
            "We can only send the uri limit"
        );
        assert!(
            params.position == None,
            "Never provided the original position so we should continue to not have a position"
        );
        let n = params.next_items();
        assert_eq!(
            params.request_limit_exceeded(),
            false,
            "The remaining limit is 50"
        );
        assert!(n.len() == 50, "The remaining limit should be 50");
        assert_eq!(params.request_required(), false);
    }
}
