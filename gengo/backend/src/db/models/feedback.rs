// Feedback model
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Feedback {
    pub id: String,
    pub session_id: String,
    pub text: String,
}
