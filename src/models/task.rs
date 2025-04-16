use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Task {
    pub id: i64,
    pub date: NaiveDate,
    pub time: String,
    pub engineer: String,
    pub contract: String,
    pub area: String,
    pub address: String,
    pub phone: String,
    pub description: String,
    pub status: TaskStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

impl Default for Task {
    fn default() -> Self {
        use chrono::Local;
        
        Self {
            id: 0, // New task marker
            date: Local::now().date_naive(),
            time: "09:00".to_string(),
            engineer: "Попов".to_string(),
            contract: "".to_string(),
            area: "".to_string(),
            address: "".to_string(),
            phone: "".to_string(),
            description: "".to_string(),
            status: TaskStatus::Pending,
        }
    }
}

impl Default for TaskStatus {
    fn default() -> Self {
        Self::Pending
    }
}
