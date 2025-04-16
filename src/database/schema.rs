use rusqlite::{Connection, Result, params};

pub fn init_schema(conn: &Connection) -> Result<()> {
    // Create engineers table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS engineers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        )",
        params![],
    )?;
    
    // Create tasks table
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
            status TEXT NOT NULL,
            FOREIGN KEY (engineer) REFERENCES engineers(name),
            UNIQUE (date, time, engineer)
        )",
        params![],
    )?;
    
    // Insert default engineers if they don't exist
    let engineers = ["Попов", "Соломаха", "Ежиков"];
    for engineer in engineers.iter() {
        conn.execute(
            "INSERT OR IGNORE INTO engineers (name) VALUES (?)",
            params![engineer],
        )?;
    }
    
    Ok(())
}
