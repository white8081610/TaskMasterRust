use rusqlite::{Connection, Result, params};
use crate::models::task::Task;
use crate::models::engineer::Engineer;
use crate::models::task::TaskStatus;

// Инициализация базы данных с созданием необходимых таблиц
pub async fn init_database() -> Result<Connection, rusqlite::Error> {
    // Создаем подключение к БД в памяти (для тестирования)
    // В реальности тут будем использовать файл БД
    let conn = Connection::open_in_memory()?;
    
    // Создаем таблицу для задач
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            date TEXT NOT NULL,
            time TEXT NOT NULL,
            engineer TEXT NOT NULL,
            contract TEXT NOT NULL,
            area TEXT NOT NULL,
            address TEXT NOT NULL,
            phone TEXT NOT NULL,
            description TEXT NOT NULL,
            status TEXT NOT NULL
        )",
        [],
    )?;
    
    // Создаем таблицу для инженеров
    conn.execute(
        "CREATE TABLE IF NOT EXISTS engineers (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;
    
    // Добавляем начальных инженеров для тестирования
    let engineers = vec![
        "Попов",
        "Соломаха",
        "Ежиков"
    ];
    
    for engineer in engineers {
        conn.execute(
            "INSERT OR IGNORE INTO engineers (name) VALUES (?1)",
            params![engineer],
        )?;
    }
    
    Ok(conn)
}

// Получение всех инженеров
pub fn get_all_engineers(conn: &Connection) -> Result<Vec<Engineer>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT name FROM engineers ORDER BY name")?;
    let engineer_iter = stmt.query_map([], |row| {
        let name: String = row.get(0)?;
        Ok(Engineer::new(name))
    })?;
    
    let mut engineers = Vec::new();
    for engineer in engineer_iter {
        engineers.push(engineer?);
    }
    
    Ok(engineers)
}

// Получение задач на определенную дату
pub fn get_tasks_by_date(conn: &Connection, date_str: &str) -> Result<Vec<Task>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, date, time, engineer, contract, area, address, phone, description, status
         FROM tasks WHERE date = ?1 ORDER BY time"
    )?;
    
    let task_iter = stmt.query_map(params![date_str], |row| {
        Ok(Task {
            id: row.get(0)?,
            date: chrono::NaiveDate::parse_from_str(&row.get::<_, String>(1)?, "%Y-%m-%d").unwrap(),
            time: row.get(2)?,
            engineer: row.get(3)?,
            contract: row.get(4)?,
            area: row.get(5)?,
            address: row.get(6)?,
            phone: row.get(7)?,
            description: row.get(8)?,
            status: match row.get::<_, String>(9)?.as_str() {
                "Pending" => TaskStatus::Pending,
                "InProgress" => TaskStatus::InProgress,
                "Completed" => TaskStatus::Completed,
                "Cancelled" => TaskStatus::Cancelled,
                _ => TaskStatus::Pending,
            },
        })
    })?;
    
    let mut tasks = Vec::new();
    for task in task_iter {
        tasks.push(task?);
    }
    
    Ok(tasks)
}

// Сохранение задачи (добавление или обновление)
pub fn save_task(conn: &Connection, task: &Task) -> Result<(), rusqlite::Error> {
    let status_str = match task.status {
        TaskStatus::Pending => "Pending",
        TaskStatus::InProgress => "InProgress",
        TaskStatus::Completed => "Completed",
        TaskStatus::Cancelled => "Cancelled",
    };
    
    let date_str = task.date.format("%Y-%m-%d").to_string();
    
    if task.id == 0 {
        // Новая задача
        conn.execute(
            "INSERT INTO tasks (date, time, engineer, contract, area, address, phone, description, status)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                date_str,
                task.time,
                task.engineer,
                task.contract,
                task.area,
                task.address,
                task.phone,
                task.description,
                status_str
            ],
        )?;
    } else {
        // Обновление существующей
        conn.execute(
            "UPDATE tasks SET 
                date = ?1, 
                time = ?2, 
                engineer = ?3, 
                contract = ?4, 
                area = ?5, 
                address = ?6, 
                phone = ?7, 
                description = ?8, 
                status = ?9
             WHERE id = ?10",
            params![
                date_str,
                task.time,
                task.engineer,
                task.contract,
                task.area,
                task.address,
                task.phone,
                task.description,
                status_str,
                task.id
            ],
        )?;
    }
    
    Ok(())
}