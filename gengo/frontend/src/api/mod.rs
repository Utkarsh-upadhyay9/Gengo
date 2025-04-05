pub mod users;
pub mod sessions;
pub mod profile;

use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fmt;

// Base URL for all API requests
pub const API_BASE_URL: &str = "http://localhost:8000/api";

#[derive(Debug)]
pub enum ApiError {
    RequestError(reqwest::Error),
    SerdeError(serde_json::Error),
    ApiResponseError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::RequestError(e) => write!(f, "Request error: {}", e),
            ApiError::SerdeError(e) => write!(f, "Serialization error: {}", e),
            ApiError::ApiResponseError(msg) => write!(f, "API error: {}", msg),
        }
    }
}

impl Error for ApiError {}

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        ApiError::RequestError(error)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(error: serde_json::Error) -> Self {
        ApiError::SerdeError(error)
    }
}

// Get auth token from storage
fn get_auth_token() -> Option<String> {
    // In a real app, get from storage
    None
}

// Fixed function with Sized constraint on B
pub async fn fetch_json<T, B>(
    method: &str,
    url: &str,
    body: Option<B>
) -> Result<T, ApiError>
where
    T: for<'de> Deserialize<'de>,
    B: Serialize
{
    let client = Client::new();
    
    let mut request = client.request(
        method.parse().map_err(|_| ApiError::ApiResponseError("Invalid HTTP method".to_string()))?,
        url
    );
    
    if let Some(body_data) = body {
        request = request.json(&body_data);
    }
    
    let response = request.send().await?;
    
    if !response.status().is_success() {
        return Err(ApiError::ApiResponseError(format!(
            "API returned error status: {}", response.status()
        )));
    }
    
    let data = response.json::<T>().await?;
    Ok(data)
}

// Example of a specific API call
pub async fn get_user_profile() -> Result<UserProfile, ApiError> {
    fetch_json::<UserProfile, ()>(
        "GET",
        "http://localhost:8000/api/users/me",
        None
    ).await
}

// Example model
#[derive(Debug, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub name: String,
    pub email: String,
}
