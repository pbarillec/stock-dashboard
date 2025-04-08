mod db;
mod models;
mod schema;

use db::{get_all_assets, get_all_transactions};
use diesel::prelude::*;
use diesel::RunQueryDsl;
use models::{Asset, NewAsset, Transaction};
use std::env;
use tauri::command;

#[command]
fn fetch_transactions() -> Vec<Transaction> {
    get_all_transactions()
}

#[command]
fn fetch_assets() -> Vec<Asset> {
    get_all_assets()
}

// Fonction pour établir la connexion à la base SQLite
pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL doit être défini");
    SqliteConnection::establish(&database_url).expect("Erreur lors de la connexion à la base")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            fetch_transactions,
            fetch_assets,
            add_asset
        ])
        .run(tauri::generate_context!())
        .expect("Erreur lors du lancement de l'application");
}

#[command]
fn add_asset(new_asset: NewAsset) -> Result<(), String> {
    use schema::assets::dsl::*;
    let mut conn = establish_connection();

    diesel::insert_into(assets)
        .values((
            symbol.eq(new_asset.symbol),
            name.eq(new_asset.name),
            category.eq(new_asset.category),
            api_id.eq(new_asset.api_id),
        ))
        .execute(&mut conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}
