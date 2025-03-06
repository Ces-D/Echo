use actix_web::{web::Json, Result};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use rspotify::model::user::PrivateUser;
use rspotify::prelude::OAuthClient;

use crate::client;
use crate::shared::errors::http_spotify_client_error;
use crate::shared::{crypto::decrypt_token, types::DbPool};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CompleteUser {
    pub id: i32,
    pub spotify_id: String,
    pub name: Option<String>,
    pub spotify: PrivateUser,
}

pub async fn get_complete_current_user(
    auth: BearerAuth,
    pool: actix_web::web::Data<DbPool>,
) -> Result<Json<CompleteUser>> {
    let bearer_token = auth.token();
    let client_token = decrypt_token(bearer_token.to_string())?;
    let client = client::spotify_client(Some(client_token));

    let spotify_user = client
        .current_user()
        .await
        .map_err(http_spotify_client_error)?;
    let spotify_id = spotify_user.id.clone();
    let db_user = actix_web::web::block(move || {
        let conn = pool.get().expect("couldn't get db connection from pool");
        crate::client::pg::get_user_by_spotify_id(conn, spotify_id.to_string())
    })
    .await?
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(Json(CompleteUser {
        id: db_user.id,
        spotify_id: db_user.spotify_id,
        name: db_user.name,
        spotify: spotify_user,
    }))
}
