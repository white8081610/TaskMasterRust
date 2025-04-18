use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

// Статус задачи
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled
}

// Структура задачи/заявки
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub date: NaiveDate,
    pub time: String,
    pub engineer: String,
    pub contract: String,
    pub area: String,
    pub address: String,
    pub phone: String,
    pub description: String,
    pub status: TaskStatus
}

impl Task {
    // Создать новую задачу с начальными значениями
    pub fn new(
        date: NaiveDate,
        time: String,
        engineer: String,
        contract: String,
        area: String,
        address: String,
        phone: String,
        description: String
    ) -> Self {
        Task {
            id: 0, // 0 означает новую задачу
            date,
            time,
            engineer,
            contract,
            area,
            address,
            phone,
            description,
            status: TaskStatus::Pending
        }
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}