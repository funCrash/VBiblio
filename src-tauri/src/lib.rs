// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use tauri_plugin_sql::{Migration, MigrationKind};
use tauri_plugin_updater::UpdaterExt;

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
                loan_date TEXT NOT NULL,
                returned INTEGER NOT NULL DEFAULT 0,
                returned_date TEXT,
                FOREIGN KEY (FK_book) REFERENCES book(PK_id),
                FOREIGN KEY (FK_student) REFERENCES student(PK_id)
              );",
            kind: MigrationKind::Up,
        },
    ];
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
          let handle = app.handle().clone();
          tauri::async_runtime::spawn(async move {
            let _ = update(handle).await;
          });
          Ok(())
        })
        .plugin(
            tauri_plugin_sql::Builder::new()
                .add_migrations("sqlite:database.sqlite", migrations)
                .build(),
        )
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
async fn update(app: tauri::AppHandle) -> Result<(), tauri_plugin_updater::Error> {
  if let Some(update) = app.updater()?.check().await? {
    let mut downloaded = 0;

    // alternatively we could also call update.download() and update.install() separately
    update.download_and_install(|chunk_length, content_length| {
      downloaded += chunk_length;
      println!("downloaded {downloaded} from {content_length:?}");
    }, || {
      println!("download finished");
    }).await?;

    println!("update installed");
    app.restart();
  }

  Ok(())
}
