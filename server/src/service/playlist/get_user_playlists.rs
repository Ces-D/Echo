use actix_web::{
    web::{Json, Query},
    HttpRequest, Result,
};
use rspotify::prelude::{BaseClient, OAuthClient};

use crate::{
    client::spotify::{model::SimplePlaylist, spotify_client_from_req},
    shared::errors::http_spotify_client_error,
};

#[apistos::api_operation(summary = "Get a summary of all the current users playlists")]
pub async fn get_user_playlists(
    query: Query<SearchQuery>,
    req: HttpRequest,
) -> Result<Json<Vec<SimplePlaylist>>> {
    let client = spotify_client_from_req(req)?;
    let spotify_user = client
        .current_user()
        .await
        .map_err(http_spotify_client_error)?;
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);
    let page = client
        .user_playlists_manual(spotify_user.id, Some(limit), Some(offset))
        .await
        .map_err(http_spotify_client_error)?;

    let mut simple_playlists: Vec<SimplePlaylist> = vec![];

    for playlist in page.items {
        simple_playlists.push(playlist.into());
    }

    Ok(Json(simple_playlists))
}

#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, schemars::JsonSchema, apistos::ApiComponent,
)]
pub struct SearchQuery {
    limit: Option<u32>,
    offset: Option<u32>,
}
