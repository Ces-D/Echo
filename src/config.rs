pub struct UserConfig {
    pub redirect_port: String,
    pub client_id: String,
    pub client_secret: String,
}

impl UserConfig {
    pub fn redirect_as_uri(&self) -> String {
        format!("http://localhost:{}", self.redirect_port)
    }
    pub fn redirect_as_addr(&self) -> String {
        format!("127.0.0.1:{}", self.redirect_port)
    }
}

pub fn read_user_config() -> UserConfig {
    UserConfig {
        redirect_port: std::env::var("redirect_port").unwrap(),
        client_id: std::env::var("client_id").unwrap(),
        client_secret: std::env::var("client_secret").unwrap(),
    }
}
