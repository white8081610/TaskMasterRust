use std::path::Path;
use rusqlite::{Connection, Result, params, OptionalExtension};
use chrono::NaiveDate;
use crate::models::task::{Task, TaskStatus};
use crate::models::engineer::Engineer;
use crate::database::schema;

#[derive(Debug)]
pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }
    
    // Get all tasks, optionally filtered by date and/or engineer
    pub async fn get_tasks(&self, date: Option<NaiveDate>, engineer: Option<String>) -> Result<Vec<Task>> {
        let mut query = String::from("SELECT id, date, time, engineer, contract, area, address, phone, description, status FROM tasks");
        let mut where_clauses = Vec::new();
        let mut params = Vec::new();
        
        if let Some(d) = date {
            where_clauses.push("date = ?");
            params.push(d.format("%Y-%m-%d").to_string());
        }
        
        if let Some(e) = engineer {
            where_clauses.push("engineer = ?");
            params.push(e);
        }
        
        if !where_clauses.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&where_clauses.join(" AND "));
        }
        
        query.push_str(" ORDER BY time");
        
        let mut stmt = self.conn.prepare(&query)?;
        let task_iter = stmt.query_map(rusqlite::params_from_iter(params.iter()), |row| {
            let date_str: String = row.get(1)?;
            let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap();
            
            let status_str: String = row.get(9)?;
            let status = match status_str.as_str() {
                "Pending" => TaskStatus::Pending,
                "InProgress" => TaskStatus::InProgress,
                "Completed" => TaskStatus::Completed,
                "Cancelled" => TaskStatus::Cancelled,
                _ => TaskStatus::Pending,
            };
            
            Ok(Task {
                id: row.get(0)?,
                date,
                time: row.get(2)?,
                engineer: row.get(3)?,
                contract: row.get(4)?,
                area: row.get(5)?,
                address: row.get(6)?,
                phone: row.get(7)?,
                description: row.get(8)?,
                status,
            })
        })?;
        
        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }
        
        Ok(tasks)
    }
    
    // Get a single task by ID
    pub async fn get_task(&self, id: i64) -> Result<Option<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, date, time, engineer, contract, area, address, phone, description, status 
             FROM tasks WHERE id = ?"
        )?;
        
        let task = stmt.query_row(params![id], |row| {
            let date_str: String = row.get(1)?;
            let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap();
            
            let status_str: String = row.get(9)?;
            let status = match status_str.as_str() {
                "Pending" => TaskStatus::Pending,
                "InProgress" => TaskStatus::InProgress,
                "Completed" => TaskStatus::Completed,
                "Cancelled" => TaskStatus::Cancelled,
                _ => TaskStatus::Pending,
            };
            
            Ok(Task {
                id: row.get(0)?,
                date,
                time: row.get(2)?,
                engineer: row.get(3)?,
                contract: row.get(4)?,
                area: row.get(5)?,
                address: row.get(6)?,
                phone: row.get(7)?,
                description: row.get(8)?,
                status,
            })
        }).optional()?;
        
        Ok(task)
    }
    
    // Create a new task
    pub async fn create_task(&mut self, task: Task) -> Result<Task> {
        let status_str = match task.status {
            TaskStatus::Pending => "Pending",
            TaskStatus::InProgress => "InProgress",
            TaskStatus::Completed => "Completed",
            TaskStatus::Cancelled => "Cancelled",
        };
        
        self.conn.execute(
            "INSERT INTO tasks (date, time, engineer, contract, area, address, phone, description, status)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                task.date.format("%Y-%m-%d").to_string(),
                task.time,
                task.engineer,
                task.contract,
                task.area,
                task.address,
                task.phone,
                task.description,
                status_str,
            ],
        )?;
        
        let id = self.conn.last_insert_rowid();
        
        let mut task = task;
        task.id = id;
        
        Ok(task)
    }
    
    // Update an existing task
    pub async fn update_task(&mut self, task: Task) -> Result<Task> {
        let status_str = match task.status {
            TaskStatus::Pending => "Pending",
            TaskStatus::InProgress => "InProgress",
            TaskStatus::Completed => "Completed",
            TaskStatus::Cancelled => "Cancelled",
        };
        
        self.conn.execute(
            "UPDATE tasks SET date = ?, time = ?, engineer = ?, contract = ?, area = ?, address = ?, phone = ?, description = ?, status = ?
             WHERE id = ?",
            params![
                task.date.format("%Y-%m-%d").to_string(),
                task.time,
                task.engineer,
                task.contract,
                task.area,
                task.address,
                task.phone,
                task.description,
                status_str,
                task.id,
            ],
        )?;
        
        Ok(task)
    }
    
    // Delete a task
    pub async fn delete_task(&mut self, id: i64) -> Result<bool> {
        let rows_affected = self.conn.execute("DELETE FROM tasks WHERE id = ?", params![id])?;
        Ok(rows_affected > 0)
    }
    
    // Get all engineers
    pub async fn get_engineers(&self) -> Result<Vec<Engineer>> {
        let mut stmt = self.conn.prepare("SELECT name FROM engineers ORDER BY name")?;
        let engineer_iter = stmt.query_map(params![], |row| {
            Ok(Engineer {
                name: row.get(0)?,
            })
        })?;
        
        let mut engineers = Vec::new();
        for engineer in engineer_iter {
            engineers.push(engineer?);
        }
        
        Ok(engineers)
    }
}

// Initialize the database
pub async fn init_database() -> Result<Database> {
    // Use a file path for persistence
    let db_path = Path::new("scheduler.db");
    let is_new_db = !db_path.exists();
    
    let conn = Connection::open(db_path)?;
    
    if is_new_db {
        schema::init_schema(&conn)?;
    }
    
    Ok(Database::new(conn))
}
