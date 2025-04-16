use std::sync::Arc;
use tokio::sync::Mutex;
use axum::{
    Router,
    routing::{get, post, put, delete},
    http::{HeaderValue, Method},
    Extension,
};
use tower_http::cors::{CorsLayer, Any};
use crate::database::operations::Database;
use crate::api::handlers;

pub async fn start_server(db: Arc<Mutex<Database>>) -> Result<(), Box<dyn std::error::Error>> {
    // Create a CORS layer that allows any origin
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);
    
    // Build our application with a route
    let app = Router::new()
        // Task routes
        .route("/api/tasks", get(handlers::get_tasks))
        .route("/api/tasks/:id", get(handlers::get_task))
        .route("/api/tasks", post(handlers::create_task))
        .route("/api/tasks/:id", put(handlers::update_task))
        .route("/api/tasks/:id", delete(handlers::delete_task))
        
        // Engineer routes
        .route("/api/engineers", get(handlers::get_engineers))
        
        // Add the database to all routes
        .layer(Extension(db))
        
        // Add CORS to all routes
        .layer(cors);
    
    // Run the server
    let addr = "0.0.0.0:8000";
    println!("API server listening on {addr}");
    
    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;
    
    Ok(())
}
