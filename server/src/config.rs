pub struct EchoConfig {
    /// Postgres database
    pub database_url: String,
}

impl EchoConfig {
    pub fn new() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        }
    }
}
