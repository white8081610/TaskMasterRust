use chrono::{NaiveDate, Datelike};

// Форматирует дату в русском формате (например, "10 Апреля 2025, Четверг")
pub fn format_date_russian(date: &NaiveDate) -> String {
    // Массивы для русских названий месяцев и дней недели
    let months = [
        "Января", "Февраля", "Марта", "Апреля", "Мая", "Июня",
        "Июля", "Августа", "Сентября", "Октября", "Ноября", "Декабря"
    ];
    
    let weekdays = [
        "Понедельник", "Вторник", "Среда", "Четверг", 
        "Пятница", "Суббота", "Воскресенье"
    ];
    
    let day = date.day();
    let month_idx = (date.month() as usize) - 1;
    let year = date.year();
    
    // В Rust 0 - это понедельник, а 6 - воскресенье
    let weekday_idx = date.weekday().num_days_from_monday() as usize;
    
    format!("{} {}, {}, {}", 
        day, 
        months[month_idx], 
        year,
        weekdays[weekday_idx]
    )
}