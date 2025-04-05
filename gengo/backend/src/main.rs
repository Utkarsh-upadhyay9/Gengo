// filepath: c:\Users\utkar\Desktop\Gengo\gengo\src\main.rs
#[macro_use] extern crate rocket;

mod auth;
mod api;
mod db;
mod speech;
mod ai;
mod frontend;
mod config;
mod error;

use dotenv::dotenv;
use rocket::fs::{FileServer, relative};
use db::connection::establish_connection;

// Add this function to initialize the database schema
async fn initialize_database(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    // Create users table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            created_at TIMESTAMP NOT NULL,
            updated_at TIMESTAMP NOT NULL
        )"
    )
    .execute(pool)
    .await?;
    
    // Create language profiles table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS language_profiles (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            target_language TEXT NOT NULL,
            native_language TEXT NOT NULL,
            proficiency_level TEXT NOT NULL,
            learning_goals TEXT,
            created_at TIMESTAMP NOT NULL,
            updated_at TIMESTAMP NOT NULL,
            FOREIGN KEY(user_id) REFERENCES users(id)
        )"
    )
    .execute(pool)
    .await?;
    
    // Create practice sessions table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS practice_sessions (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            topic TEXT NOT NULL,
            duration_seconds INTEGER NOT NULL,
            completed BOOLEAN NOT NULL,
            created_at TIMESTAMP NOT NULL,
            completed_at TIMESTAMP,
            FOREIGN KEY(user_id) REFERENCES users(id)
        )"
    )
    .execute(pool)
    .await?;
    
    // Create session analytics table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS session_analytics (
            id TEXT PRIMARY KEY,
            session_id TEXT NOT NULL UNIQUE,
            pronunciation_score REAL,
            grammar_score REAL,
            vocabulary_score REAL,
            fluency_score REAL,
            overall_score REAL,
            feedback TEXT NOT NULL,
            created_at TIMESTAMP NOT NULL,
            FOREIGN KEY(session_id) REFERENCES practice_sessions(id)
        )"
    )
    .execute(pool)
    .await?;
    
    // Create feedback table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS feedback (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            session_id TEXT,
            text TEXT NOT NULL,
            created_at TIMESTAMP NOT NULL,
            FOREIGN KEY(user_id) REFERENCES users(id),
            FOREIGN KEY(session_id) REFERENCES practice_sessions(id)
        )"
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to Gengo Language Learning App! 🌍"
}

#[get("/health")]
fn health() -> &'static str {
    "OK"
}

#[launch]
async fn rocket() -> _ {
    // Load environment variables
    dotenv().ok();
    
    // Initialize database connection with error handling
    let db_pool = match establish_connection().await {
        Ok(pool) => {
            println!("✅ Database connection successful");
            
            // Initialize database schema
            match initialize_database(&pool).await {
                Ok(_) => println!("✅ Database schema initialized successfully"),
                Err(e) => println!("⚠️ Error initializing database schema: {}", e)
            }
            
            Some(pool)
        },
        Err(e) => {
            println!("⚠️ Database connection failed: {}. Running without database.", e);
            None
        }
    };
    
    // Basic rocket instance
    let mut rocket_build = rocket::build()
        .mount("/", routes![index, health])
        .mount("/static", FileServer::from(relative!("static")));
        
    // Add database if available and mount API routes
    if let Some(pool) = db_pool {
        rocket_build = rocket_build
            .manage(pool)
            .mount("/api", api::routes());
    }
    
    rocket_build
}