use super::{
    config::{get_env_var, Environment},
    errors::{http_aes_gcm_error, http_hex_error},
};
use actix_web::cookie::Cookie;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};

/// Encrypts the data using AES256-GCM
fn encrypt(data: String) -> String {
    let hash_key = get_env_var(Environment::HashKey);

    let key = Key::<Aes256Gcm>::from_slice(hash_key.as_bytes());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let cipher = Aes256Gcm::new(key);
    let ciphered_data = cipher
        .encrypt(&nonce, data.as_bytes())
        .expect("failed to encrypt");

    // combining nonce and encrypted data together
    // for storage purpose
    let mut encrypted_data: Vec<u8> = nonce.to_vec();
    encrypted_data.extend_from_slice(&ciphered_data);

    hex::encode(encrypted_data)
}

/// Decrypts the data using AES256-GCM
fn decrypt(data: String) -> actix_web::Result<String> {
    let hash_key = get_env_var(Environment::HashKey);

    let encrypted_data = hex::decode(data).map_err(http_hex_error)?;
    let key = Key::<Aes256Gcm>::from_slice(hash_key.as_bytes());
    let (nonce_arr, ciphered_data) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_arr);
    let cipher = Aes256Gcm::new(key);

    let plaintext = cipher
        .decrypt(nonce, ciphered_data)
        .map_err(http_aes_gcm_error)?;
    Ok(String::from_utf8(plaintext).expect("failed to convert vector of bytes to string"))
}

// Encrypt the spotify access_token and refresh_token
fn encrypt_token(token: rspotify::Token) -> rspotify::Token {
    match token.refresh_token {
        Some(refresh_token) => rspotify::Token {
            access_token: encrypt(token.access_token),
            refresh_token: Some(encrypt(refresh_token)),
            ..token
        },
        None => rspotify::Token {
            access_token: encrypt(token.access_token),
            ..token
        },
    }
}

// Decrypt the spotify access_token and refresh_token
fn decrypt_token(token: rspotify::Token) -> actix_web::Result<rspotify::Token> {
    let access_token = decrypt(token.access_token)?;
    let decrypted_token = match token.refresh_token {
        Some(refresh_token) => rspotify::Token {
            access_token,
            refresh_token: Some(decrypt(refresh_token)?),
            ..token
        },
        None => rspotify::Token {
            access_token,
            ..token
        },
    };
    Ok(decrypted_token)
}

// Create an encrypted session cookie
pub fn create_session_cookie<'a>(token: rspotify::Token) -> Cookie<'a> {
    let cookie_key = get_env_var(Environment::SessionCookieKey);
    let encrypted_token = encrypt_token(token);
    let token_str = serde_json::to_string(&encrypted_token).expect("failed to serialize token");
    Cookie::build(cookie_key, token_str)
        .domain(get_env_var(Environment::BaseUrl))
        .secure(true)
        .http_only(true)
        .expires(actix_web::cookie::Expiration::Session)
        .finish()
}

pub fn decrypt_session_cookie<'a>(cookie: Cookie<'a>) -> rspotify::Token {
    let token_str = cookie.value();
    let token: rspotify::Token =
        serde_json::from_str(token_str).expect("failed to deserialize token");
    decrypt_token(token).expect("failed to decrypt token")
}
