// Feedback API endpoints

#[derive(serde::Serialize)]
pub struct FeedbackResponse {
    id: String,
    session_id: String,
    text: String,
}

// Placeholder for actual implementation
pub fn routes() -> Vec<rocket::Route> {
    vec![]
}
