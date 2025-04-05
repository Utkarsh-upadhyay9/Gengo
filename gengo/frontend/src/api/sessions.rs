use serde::{Serialize, Deserialize};
use super::{fetch_json, ApiError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub topic: String,
    pub created_at: String,
    pub completed: bool,
    pub score: Option<i32>,
}

// Get list of recent practice sessions
pub async fn get_recent_sessions() -> Result<Vec<Session>, ApiError> {
    fetch_json::<Vec<Session>, _>("/sessions/recent", "GET", None::<&()>).await
}

// Get a specific session by ID
pub async fn get_session(id: &str) -> Result<Session, ApiError> {
    fetch_json::<Session, _>(&format!("/sessions/{}", id), "GET", None::<&()>).await
}
