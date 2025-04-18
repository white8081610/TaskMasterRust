use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,        // Ожидание
    InProgress,     // В процессе
    Completed,      // Завершено
    Cancelled,      // Отменено
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub date: NaiveDate,
    pub time: String,           // Время (09:30, 10:00, etc.)
    pub engineer: String,       // Имя инженера
    pub contract: String,       // Номер договора
    pub area: String,           // Район (СО-2, etc.)
    pub address: String,        // Адрес
    pub phone: String,          // Номер телефона
    pub description: String,    // Описание проблемы
    pub status: TaskStatus,     // Статус задачи
}

impl Task {
    pub fn new(
        date: NaiveDate,
        time: String,
        engineer: String,
        contract: String,
        area: String,
        address: String,
        phone: String,
        description: String,
    ) -> Self {
        Self {
            id: 0, // ID будет назначен при сохранении в БД
            date,
            time,
            engineer,
            contract,
            area,
            address,
            phone,
            description,
            status: TaskStatus::Pending,
        }
    }
}