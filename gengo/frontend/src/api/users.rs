use serde::{Serialize, Deserialize};
use super::{fetch_json, ApiError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

// Login user
pub async fn login(username: &str, password: &str) -> Result<AuthResponse, ApiError> {
    let request = LoginRequest {
        username: username.to_string(),
        password: password.to_string(),
    };
    
    fetch_json::<AuthResponse, _>("/auth/login", "POST", Some(&request)).await
}

// Get current user profile
pub async fn get_current_user() -> Result<User, ApiError> {
    fetch_json::<User, _>("/users/me", "GET", None::<&()>).await
}
