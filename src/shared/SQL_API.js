import Database from "@tauri-apps/plugin-sql";

const db = await Database.load("sqlite:database.sqlite");
export default db;
