use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use log::{error, info, trace};
use spotify_rs::auth::{NoVerifier, Token};
use spotify_rs::client::Client;
use spotify_rs::SpotifyResult;
use spotify_rs::{AuthCodeClient, AuthCodeFlow, RedirectUrl};

use crate::config::UserConfig;
use crate::spotify::constants::SCOPES;

struct RedirectCredentials {
    pub auth_code: String,
    pub csrf_token: String,
}

fn listen_to_redirect_server(redirect_uri: String) -> Option<RedirectCredentials> {
    let redirect_listener = TcpListener::bind(redirect_uri).expect("Unable to create listener");
    for stream in redirect_listener.incoming() {
        if stream.is_ok() {
            match handle_connection(stream.unwrap()) {
                Some(credentials) => {
                    info!("Retrieved credentials");
                    return Some(credentials);
                }

                None => trace!("Listening..."),
            }
        }
    }
    None
}

fn handle_connection(mut stream: TcpStream) -> Option<RedirectCredentials> {
    // The request will be quite large (> 512) so just assign plenty just in case
    let mut buffer = [0; 1000];
    let _ = stream.read(&mut buffer).unwrap();

    // convert buffer into string and 'parse' the URL
    match String::from_utf8(buffer.to_vec()) {
        Ok(request) => {
            let split: Vec<&str> = request.split_whitespace().collect();

            if split.len() > 1 {
                respond_with_success(stream);
                let values = split[1]
                    .split(|c| c == '=' || c == '&')
                    .collect::<Vec<&str>>();
                let auth_code = values[1].to_string();
                let csrf_token = values[3].to_string();

                return Some(RedirectCredentials {
                    auth_code,
                    csrf_token,
                });
            }

            respond_with_error("Malformed request".to_string(), stream);
        }
        Err(e) => {
            respond_with_error(format!("Invalid UTF-8 sequence: {}", e), stream);
        }
    };

    None
}

fn respond_with_success(mut stream: TcpStream) {
    let contents = "Client authorized. You can return to your terminal and close this window.";

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn respond_with_error(error_message: String, mut stream: TcpStream) {
    error!("Error: {}", error_message);
    let response = format!(
        "HTTP/1.1 400 Bad Request\r\n\r\n400 - Bad Request - {}",
        error_message
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub async fn initialize_client(
    config: UserConfig,
) -> SpotifyResult<Client<Token, AuthCodeFlow, NoVerifier>> {
    // Redirect the user to this URL to get the auth code and CSRF token
    let (client, url) = AuthCodeClient::new(
        AuthCodeFlow::new(
            config.client_id.clone(),
            config.client_secret.clone(),
            SCOPES,
        ),
        RedirectUrl::new(config.redirect_as_uri()).unwrap(),
        true,
    );
    info!("Click the authorization link:");
    info!("{}", url);

    // They will then have to be redirected to the `redirect_url` you specified,
    // with those two parameters present in the URL
    if let Some(credentials) = listen_to_redirect_server(config.redirect_as_addr()) {
        // Finally, exchange the auth code for an access token

        client
            .authenticate(credentials.auth_code, credentials.csrf_token)
            .await
    } else {
        Err(spotify_rs::Error::NotAuthenticated)
    }
}
