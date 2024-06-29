use std::fs;

use rusqlite::Connection;

const DB_FILE_PATH: &str = ".notes.sqlite";

fn file_exists() -> bool {
    fs::metadata(DB_FILE_PATH).is_ok()
}

fn create_file() {
    fs::write(DB_FILE_PATH, "").expect("Error creating db file");
    define_db_tables()
}

fn open_db_connection() -> Connection {
    Connection::open(DB_FILE_PATH).expect("Error getting connection")
}

fn define_db_tables() {
    let mut connection = open_db_connection();

    let tx = connection
        .transaction()
        .expect("Error getting transtaction when defining tables");

    tx.execute(
        "
        CREATE TABLE IF NOT EXISTS note (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        );
    ",
        [],
    )
    .expect("Error defining notes table");

    tx.commit()
        .expect("Error commiting create tables operation");
}

pub fn get_db() -> Connection {
    if !file_exists() {
        create_file();
    }
    open_db_connection()
}
