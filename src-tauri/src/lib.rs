// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use tauri_plugin_sql::{Migration, MigrationKind};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![
        // Define your migrations here
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: "CREATE TABLE IF NOT EXISTS book (
                PK_id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                level INTEGER DEFAULT NULL
              );
              CREATE TABLE IF NOT EXISTS student (
                PK_id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT,
                firstname TEXT NOT NULL
              );
              CREATE TABLE IF NOT EXISTS loan (
                PK_id INTEGER PRIMARY KEY AUTOINCREMENT,
                FK_book INTEGER NOT NULL,
                FK_student INTEGER NOT NULL,
                FOREIGN KEY (FK_book) REFERENCES book(PK_id),
                FOREIGN KEY (FK_student) REFERENCES student(PK_id)
              );",
            kind: MigrationKind::Up,
        }
    ];
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().add_migrations("sqlite:database.sqlite", migrations).build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
