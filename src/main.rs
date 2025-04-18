mod app;
mod components;
mod models;
mod api;
mod database;
mod utils;

use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Настройка логирования
    env_logger::init();
    
    // Инициализация базы данных
    let db = database::operations::init_database().await?;
    let db = Arc::new(Mutex::new(db));
    
    // Запускаем API-сервер в отдельном потоке
    let api_db = db.clone();
    tokio::spawn(async move {
        println!("Запуск API-сервера для мобильного доступа...");
        if let Err(e) = api::server::start_server(api_db).await {
            eprintln!("Ошибка API-сервера: {}", e);
        }
    });
    
    // Запускаем Dioxus для веб
    println!("Запуск веб-интерфейса планировщика заявок...");
    dioxus_web::launch(app::app);
    
    Ok(())
}
