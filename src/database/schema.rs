use rusqlite::{Connection, Result};

// Инициализация схемы базы данных
pub fn init_schema(conn: &Connection) -> Result<()> {
    // Создаем таблицу tasks
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
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
    
    // Создаем таблицу engineers
    conn.execute(
        "CREATE TABLE IF NOT EXISTS engineers (
            name TEXT PRIMARY KEY
        )",
        [],
    )?;
    
    // Добавляем некоторых инженеров изначально
    let engineers = ["Попов", "Соломаха", "Ежиков"];
    for engineer in engineers {
        conn.execute(
            "INSERT OR IGNORE INTO engineers (name) VALUES (?)",
            [engineer],
        )?;
    }
    
    Ok(())
}