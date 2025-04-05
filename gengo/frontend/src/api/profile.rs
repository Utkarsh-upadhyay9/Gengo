use serde::{Serialize, Deserialize};
use super::{fetch_json, ApiError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageProfile {
    pub id: String,
    pub target_language: String,
    pub proficiency_level: String,
}

// Get user's language profile
pub async fn get_profile() -> Result<LanguageProfile, ApiError> {
    fetch_json::<LanguageProfile, _>("/profile", "GET", None::<&()>).await
}
