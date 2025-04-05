// Application configuration
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite::memory:".to_string());
            
        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "default_secret_change_me".to_string());
            
        Config {
            database_url,
            jwt_secret,
        }
    }
}
