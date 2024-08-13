use redirect_uri::listen_to_redirect_server;
use spotify_rs::{AuthCodeClient, AuthCodeFlow, RedirectUrl};
use std::error::Error;
mod redirect_uri;

struct UserConfig {
    redirect_port: String,
    client_id: String,
    client_secret: String,
}

impl UserConfig {
    pub fn redirect_as_uri(&self) -> String {
        format!("http://localhost:{}", self.redirect_port)
    }
    pub fn redirect_as_addr(&self) -> String {
        format!("127.0.0.1:{}", self.redirect_port)
    }
}

fn read_user_config() -> UserConfig {
    UserConfig {
        redirect_port: std::env::var("redirect_port").unwrap(),
        client_id: std::env::var("client_id").unwrap(),
        client_secret: std::env::var("client_secret").unwrap(),
    }
}

fn create_playlist() {}
fn generate_playlist_recommendations() {
    // see - https://developer.spotify.com/documentation/web-api/reference/get-users-saved-tracks
    // see - https://developer.spotify.com/documentation/web-api/reference/get-recommendations
    // see - https://developer.spotify.com/documentation/web-api/reference/get-audio-analysis
    // see - https://developer.spotify.com/documentation/web-api/reference/get-audio-features
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = read_user_config();
    let scopes = vec!["user-library-read", "playlist-read-private"];

    // Redirect the user to this URL to get the auth code and CSRF token
    let (client, url) = AuthCodeClient::new(
        AuthCodeFlow::new(
            config.client_id.clone(),
            config.client_secret.clone(),
            scopes,
        ),
        RedirectUrl::new(config.redirect_as_uri()).unwrap(),
        true,
    );

    println!("Click the authorization link:");
    println!("{}", url);

    // They will then have to be redirected to the `redirect_url` you specified,
    // with those two parameters present in the URL
    if let Some(credentials) = listen_to_redirect_server(config.redirect_as_addr()) {
        // Finally, exchange the auth code for an access token

        let mut spotify = client
            .authenticate(credentials.auth_code, credentials.csrf_token)
            .await?;
    }

    Ok(())
}
