// Authentication module
pub mod jwt;
pub mod middleware;

#[derive(Debug, Clone)]
pub struct Auth {
    pub user_id: Option<String>,
    pub username: Option<String>,
}
