use actix_web::{http::header::Header, web::Json, HttpRequest, Result};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use rspotify::prelude::OAuthClient;

use crate::{
    client::{
        pg::{user::get_user_by_spotify_id, DbPool},
        spotify::{model::PrivateUser, spotify_client},
    },
    shared::{crypto::authorization::decrypt_session_token, errors::http_spotify_client_error},
};

#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, schemars::JsonSchema, apistos::ApiComponent,
)]
pub struct CompleteUser {
    pub id: i32,
    pub spotify_id: String,
    pub name: Option<String>,
    pub spotify: PrivateUser,
}

#[apistos::api_operation(summary = "Get the complete user")]
pub async fn get_complete_current_user(
    req: HttpRequest,
    pool: actix_web::web::Data<DbPool>,
) -> Result<Json<CompleteUser>> {
    let credentials = Authorization::<Bearer>::parse(&req)?;
    let bearer = credentials.as_ref();
    let token = decrypt_session_token(bearer.token())?;
    let client = spotify_client(Some(token), None);

    let spotify_user = client
        .current_user()
        .await
        .map_err(http_spotify_client_error)?;
    let spotify_id = spotify_user.id.clone();
    let db_user = actix_web::web::block(move || {
        let conn = pool.get().expect("couldn't get db connection from pool");
        get_user_by_spotify_id(conn, spotify_id.to_string())
    })
    .await?
    .await
    .map_err(crate::shared::errors::http_diesel_error)?;

    Ok(Json(CompleteUser {
        id: db_user.id,
        spotify_id: db_user.spotify_id,
        name: db_user.name,
        spotify: spotify_user.into(),
    }))
}
