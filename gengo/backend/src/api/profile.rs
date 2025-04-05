// Profile API endpoints

#[derive(serde::Serialize)]
pub struct ProfileResponse {
    id: String,
    user_id: String,
    language: String,
}

// Placeholder for actual implementation
pub fn routes() -> Vec<rocket::Route> {
    vec![]
}
