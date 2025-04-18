mod components;
mod models;
mod utils;
mod database;
mod api;
mod app;

use app::app;

fn main() {
    // Запускаем веб-приложение
    dioxus_web::launch(app);
}