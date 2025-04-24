use super::{
    config::{get_env_var, Environment},
    errors::{http_aes_gcm_error, http_hex_error},
};
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

pub mod authorization {
    use super::{decrypt, encrypt};
    use crate::shared::config::{get_env_var, Environment};
    use actix_web::cookie::Cookie;

    /// Create an encrypted session cookie
    pub fn create_session_cookie<'a>(token: rspotify::Token) -> Cookie<'a> {
        let cookie_key = get_env_var(Environment::SessionCookieKey);
        let token_str = serde_json::to_string(&token).expect("failed to serialize token");
        let encrypted_token = encrypt(token_str);
        Cookie::build(cookie_key, encrypted_token)
            .path("/")
            .secure(true)
            .same_site(actix_web::cookie::SameSite::None)
            .http_only(true)
            .expires(None) // session cookie
            .finish()
    }

    /// Decrypt a session cookie token
    pub fn decrypt_session_token(token: &str) -> actix_web::Result<rspotify::Token> {
        let decrypted_str = decrypt(token.to_string())?;
        let token: rspotify::Token =
            serde_json::from_str(decrypted_str.as_str()).expect("failed to deserialize token");
        Ok(token)
    }
}
