// API endpoints module
use rocket::Route;

pub mod users;
pub mod sessions;
pub mod profile;
pub mod feedback;

// Collect and export all routes
pub fn routes() -> Vec<Route> {
    vec![]
}
