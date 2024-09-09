import Database from "@tauri-apps/plugin-sql";

async function loadDatabase() {
    const db = await Database.load("sqlite:database.sqlite");
    return db
}

export default loadDatabase;
