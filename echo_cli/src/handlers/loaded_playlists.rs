use std::borrow::BorrowMut;

use echo_db::crud::read::{echo::Playlist, read_playlists};

use crate::error::EchoError;

pub async fn loaded_playlists_handler() -> Result<Vec<Playlist>, EchoError> {
    let mut db_conn = echo_db::connection::establish_connection();
    read_playlists(db_conn.borrow_mut())
        .map_err(|error| EchoError::DatabaseError(error.to_string()))
}
