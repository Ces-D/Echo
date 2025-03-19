use crate::client::{pg::create_new_user, spotify_client};
use crate::shared;
use crate::shared::config::{get_env_var, Environment};
use crate::shared::crypto::create_session_cookie;
use crate::shared::{errors::http_spotify_client_error, types::DbPool};

use actix_web::{web::Json, HttpRequest, Responder, Result};
use rspotify::prelude::{BaseClient, OAuthClient};

#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, schemars::JsonSchema, apistos::ApiComponent,
)]
pub struct SpotifyAuthUrlResponse {
    url: String,
}

#[apistos::api_operation()]
pub async fn generate_spotify_request_url() -> Result<Json<SpotifyAuthUrlResponse>> {
    let client = spotify_client(None, None);
    match client.get_authorize_url(true) {
        Ok(url) => Ok(Json(SpotifyAuthUrlResponse { url })),
        Err(client_error) => Err(http_spotify_client_error(client_error)),
    }
}

#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, schemars::JsonSchema, apistos::ApiComponent,
)]
pub struct ParseSpotifyResponseUrlQueries {
    pub state: String,
    pub code: String,
}

#[apistos::api_operation()]
pub async fn parse_spotify_response_url(
    query: actix_web::web::Query<ParseSpotifyResponseUrlQueries>,
    pool: actix_web::web::Data<DbPool>,
    req: HttpRequest,
) -> impl Responder {
    let client = spotify_client(None, Some(query.state.clone()));
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

    actix_web::web::block(move || {
        let conn = pool.get().expect("couldn't get db connection from pool");
        create_new_user(conn, user)
    })
    .await?
    .await
    .map_err(shared::errors::http_diesel_error)?;

    let stored_token = client.get_token();
    let locked = stored_token.lock().await.unwrap();
    // create session id and somehow use that to get this cookie
    let cookie = match locked.clone() {
        Some(token) => create_session_cookie(token),
        None => {
            return Err(actix_web::error::ErrorBadRequest(
                "No token found in response",
            ))
        }
    };

    Ok(actix_web::HttpResponse::TemporaryRedirect()
        .insert_header(("location", get_env_var(Environment::ClientUrl)))
        .cookie(cookie)
        .finish())
}

#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, schemars::JsonSchema, apistos::ApiComponent,
)]
pub struct AuthorizationValidResponse {
    valid: bool,
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
