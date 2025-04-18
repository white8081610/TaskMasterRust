use axum::{
    routing::{get, post},
    Router, Json, extract::State,
    http::StatusCode,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use rusqlite::Connection;
use crate::models::task::Task;

// Общее состояние для сервера API
struct AppState {
    db: Arc<Mutex<Connection>>,
}

// Запуск API сервера для мобильного доступа
pub async fn start_server(db: Arc<Mutex<Connection>>) -> Result<(), Box<dyn std::error::Error>> {
    let state = Arc::new(AppState { db });
    
    // Создаем маршрутизатор API
    let app = Router::new()
        .route("/api/tasks", get(get_tasks))
        .route("/api/tasks", post(create_task))
        .route("/api/tasks/:id", get(get_task))
        .with_state(state);
    
    // Запускаем сервер на порту 3001
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("API сервер работает на http://{}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    
    Ok(())
}

// Обработчик получения всех задач
async fn get_tasks(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Task>>, StatusCode> {
    // Тут будет получение задач из БД
    Ok(Json(Vec::new()))
}

// Обработчик получения информации о конкретной задаче
async fn get_task(State(state): State<Arc<AppState>>) -> Result<Json<Task>, StatusCode> {
    // Тут будет получение задачи из БД
    Err(StatusCode::NOT_FOUND)
}

// Обработчик создания новой задачи
async fn create_task(State(state): State<Arc<AppState>>, Json(task): Json<Task>) -> Result<Json<Task>, StatusCode> {
    // Тут будет сохранение задачи в БД
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}