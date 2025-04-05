// User API endpoints

#[derive(serde::Serialize)]
pub struct UserResponse {
    id: String,
    username: String,
}

// Placeholder for actual implementation
pub fn routes() -> Vec<rocket::Route> {
    vec![]
}
