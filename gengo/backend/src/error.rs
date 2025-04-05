// Error handling
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

impl<'r> Responder<'r, 'static> for AppError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let status = match self {
            AppError::NotFound(_) => Status::NotFound,
            AppError::Database(_) => Status::InternalServerError,
            AppError::Unauthorized(_) => Status::Unauthorized,
            AppError::BadRequest(_) => Status::BadRequest,
            AppError::Internal(_) => Status::InternalServerError,
        };
        
        Response::build()
            .status(status)
            .sized_body(self.to_string().len(), std::io::Cursor::new(self.to_string()))
            .ok()
    }
}
