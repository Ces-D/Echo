use crate::client::{pg::create_new_user, spotify_client};
use crate::shared::crypto::encrypt_token;
use crate::shared::{errors::http_spotify_client_error, types::DbPool};

use actix_web::{web::Json, HttpRequest, Result};
use rspotify::prelude::{BaseClient, OAuthClient};

#[derive(serde::Serialize, serde::Deserialize, utoipa::OpenApi)]
pub struct SpotifyAuthUrlResponse {
    url: String,
}
pub async fn generate_spotify_request_url() -> Result<Json<SpotifyAuthUrlResponse>> {
    let client = spotify_client(None);
    match client.get_authorize_url(true) {
        Ok(url) => Ok(Json(SpotifyAuthUrlResponse { url })),
        Err(client_error) => Err(http_spotify_client_error(client_error)),
    }
}

#[derive(serde::Serialize, serde::Deserialize, utoipa::OpenApi)]
pub struct SpotifyTokenResponse {
    token: rspotify::model::Token,
}
pub async fn parse_spotify_response_url(
    req: HttpRequest,
    pool: actix_web::web::Data<DbPool>,
) -> Result<Json<SpotifyTokenResponse>> {
    let client = spotify_client(None);
    let code = match client.parse_response_code(req.full_url().as_str()) {
        Some(code) => code,
        None => {
            return Err(actix_web::error::ErrorBadRequest(
                "No spotify code found in response",
            ))
        }
    };
    client
        .request_token(&code)
        .await
        .map_err(http_spotify_client_error)?;

    client
        .refresh_token()
        .await
        .map_err(http_spotify_client_error)?;

    let user = client
        .current_user()
        .await
        .map_err(http_spotify_client_error)?;

    let created = actix_web::web::block(move || {
        let conn = pool.get().expect("couldn't get db connection from pool");
        create_new_user(conn, user)
    })
    .await?
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if created {
        let stored_token = client.get_token();
        let locked = stored_token.lock().await.unwrap();
        let token = match locked.clone() {
            Some(token) => encrypt_token(token),
            None => {
                return Err(actix_web::error::ErrorBadRequest(
                    "No token found in response",
                ))
            }
        };

        Ok(Json(SpotifyTokenResponse { token }))
    } else {
        Err(actix_web::error::ErrorBadRequest("User may already exist"))
    }
}

#[cfg(test)]
mod auth_tests {
    use super::*;
    use actix_web::{test, web, App};

    #[actix_web::test]
    async fn generate_spotify_request_url_returns_url() {
        let app =
            test::init_service(App::new().route("/", web::get().to(generate_spotify_request_url)))
                .await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success())
    }
}
