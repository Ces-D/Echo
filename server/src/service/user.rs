use actix_web::{web::Json, HttpRequest, Result};
use rspotify::model::user::PrivateUser;
use rspotify::prelude::OAuthClient;

use crate::shared::{
    config::{get_env_var, Environment},
    crypto::decrypt_session_cookie,
    errors::http_spotify_client_error,
    types::DbPool,
};
use crate::{client, shared};

#[derive(serde::Serialize, serde::Deserialize, utoipa::OpenApi)]
pub struct CompleteUser {
    pub id: i32,
    pub spotify_id: String,
    pub name: Option<String>,
    pub spotify: PrivateUser,
}

pub async fn get_complete_current_user(
    req: HttpRequest,
    pool: actix_web::web::Data<DbPool>,
) -> Result<Json<CompleteUser>> {
    let cookie = req
        .cookie(&get_env_var(Environment::SessionCookieKey))
        .unwrap();
    let token = decrypt_session_cookie(cookie);
    let client = client::spotify_client(Some(token));

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
    .map_err(shared::errors::http_diesel_error)?;

    Ok(Json(CompleteUser {
        id: db_user.id,
        spotify_id: db_user.spotify_id,
        name: db_user.name,
        spotify: spotify_user,
    }))
}
