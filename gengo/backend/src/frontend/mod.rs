// Server-side rendering for frontend
use rocket::fs::FileServer;
use rocket::fs::relative;
use rocket::Route;

pub fn routes() -> Vec<Route> {
    vec![]
}

pub fn static_files() -> FileServer {
    FileServer::from(relative!("static"))
}
