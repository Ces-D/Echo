use actix_web::{web::Query, HttpRequest, Result};
use futures::stream::StreamExt;
use rspotify::{
    model::{PlaylistId, PlaylistItem},
    prelude::{BaseClient, OAuthClient},
    ClientResult,
};

use crate::{
    client::{pg::DbPool, spotify::spotify_client_from_req},
    shared::errors::http_spotify_client_error,
};

#[apistos::api_operation(summary = "Load the all tracks from a playlist")]
pub async fn load_user_playlist(
    query: Query<SearchQuery>,
    req: HttpRequest,
    pool: actix_web::web::Data<DbPool>,
) -> Result<()> {
    let client = spotify_client_from_req(req)?;
    let playlist_id = match query.id.clone() {
        Some(id) => Some(PlaylistId::from_id(id).unwrap()),
        None => None,
    };

    let track_stream = match playlist_id {
        Some(id) => {
            client
                .playlist_items(id, None, None)
                .collect::<Vec<ClientResult<PlaylistItem>>>()
                .await
        }
        None => {
            todo!()
        }
    };

    Ok(())
}

#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, schemars::JsonSchema, apistos::ApiComponent,
)]
pub struct SearchQuery {
    id: Option<String>,
}
