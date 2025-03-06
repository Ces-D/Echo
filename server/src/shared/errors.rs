use actix_web::error as e;

pub fn http_spotify_client_error(error: rspotify::ClientError) -> actix_web::Error {
    match error {
        rspotify::ClientError::ParseJson(error) => e::ErrorBadRequest(error),
        rspotify::ClientError::ParseUrl(parse_error) => e::ErrorBadRequest(parse_error),
        rspotify::ClientError::Http(reqwest_error) => e::ErrorInternalServerError(reqwest_error),
        _ => e::ErrorInternalServerError("Untracked Spotify client error"),
    }
}

pub fn http_diesel_error(error: diesel::result::Error) -> actix_web::Error {
    match error {
        diesel::result::Error::DatabaseError(_, database_error_information) => {
            e::ErrorInternalServerError(database_error_information.message().to_owned())
        }
        diesel::result::Error::NotFound => e::ErrorNotFound("Expected record not found"),
        _ => e::ErrorInternalServerError("Untracked diesel error"),
    }
}

pub fn http_aes_gcm_error(error: aes_gcm::Error) -> actix_web::Error {
    e::ErrorUnauthorized(error.to_string())
}

pub fn http_hex_error(error: hex::FromHexError) -> actix_web::Error {
    e::ErrorUnauthorized(error.to_string())
}
