use std::sync::Arc;
use tokio::sync::Mutex;

mod app;
mod components;
mod models;
mod api;
mod database;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the database
    let db = database::operations::init_database().await?;
    let db = Arc::new(Mutex::new(db));
    
    // Start the API server in a separate thread
    let api_db = db.clone();
    tokio::spawn(async move {
        api::server::start_server(api_db).await.unwrap();
    });
    
    // Launch the Dioxus app
    dioxus_desktop::launch_cfg(
        app::App,
        dioxus_desktop::Config::new()
            .with_window(
                dioxus_desktop::WindowBuilder::new()
                    .with_title("Планировщик заявок версия 0.6.0")
                    .with_inner_size(dioxus_desktop::LogicalSize::new(1000, 800))
            )
    );
    
    Ok(())
}
