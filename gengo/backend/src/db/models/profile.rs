// Profile model
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub user_id: String,
    pub target_language: String,
}
