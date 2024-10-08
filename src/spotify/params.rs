use super::constants::{SPOTIFY_TRACKS_LIMIT, SPOTIFY_URIS_LIMIT};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SpotifyReadTracksParams {
    pub offset: u32,
    pub limit: u32,
}
impl SpotifyReadTracksParams {
    /// Returns a vec of params where the limit and offset are structured for requests in parallel
    /// @see - https://docs.rs/tokio/latest/tokio/macro.join.html
    pub fn new_async(offset: u32, limit: u32) -> Vec<SpotifyReadTracksParams> {
        if limit > SPOTIFY_TRACKS_LIMIT {
            let mut instances: Vec<SpotifyReadTracksParams> = vec![];
            for loop_value in 0..limit / SPOTIFY_TRACKS_LIMIT {
                instances.push(SpotifyReadTracksParams {
                    offset: loop_value * SPOTIFY_TRACKS_LIMIT,
                    limit: SPOTIFY_TRACKS_LIMIT,
                });
            }
            let remaining = limit % SPOTIFY_TRACKS_LIMIT;
            if remaining != 0 {
                let last_offset = limit / SPOTIFY_TRACKS_LIMIT * SPOTIFY_TRACKS_LIMIT;
                instances.push(SpotifyReadTracksParams {
                    offset: last_offset,
                    limit: remaining,
                });
            }
            instances
        } else {
            vec![SpotifyReadTracksParams { offset, limit }]
        }
    }
}

impl Default for SpotifyReadTracksParams {
    fn default() -> Self {
        SpotifyReadTracksParams {
            offset: 0,
            limit: SPOTIFY_TRACKS_LIMIT,
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
    fn parallel_read_params_have_correct_offsets() {
        let params = SpotifyReadTracksParams::new_async(0, 183);
        assert_eq!(
            params[0],
            SpotifyReadTracksParams {
                offset: 0,
                limit: SPOTIFY_TRACKS_LIMIT
            },
            "The first item should be the params for the first tracks in the playlist"
        );
        assert_eq!(
            params[1],
            SpotifyReadTracksParams {
                offset: SPOTIFY_TRACKS_LIMIT,
                limit: SPOTIFY_TRACKS_LIMIT
            }
        );
        assert_eq!(
            params[2],
            SpotifyReadTracksParams {
                offset: SPOTIFY_TRACKS_LIMIT * 2,
                limit: SPOTIFY_TRACKS_LIMIT
            }
        );
        assert_eq!(
            params.last().unwrap().clone(),
            SpotifyReadTracksParams {
                offset: SPOTIFY_TRACKS_LIMIT * 3,
                limit: 33
            },
            "The last item should be the params for the last tracks in the playlist"
        )
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
