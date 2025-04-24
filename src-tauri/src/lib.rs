mod db;
mod models;
mod schema;

use db::{get_all_assets, get_all_transactions};
use diesel::prelude::*;
use diesel::RunQueryDsl;
use models::{Asset, NewAsset, NewTransaction, Transaction};
use reqwest;
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
            add_asset,
            delete_asset,
            add_transaction,
            delete_transaction,
            search_asset_twelve_data
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

#[command]
fn delete_asset(asset_id: i32) -> Result<(), String> {
    use schema::assets::dsl::*;
    let mut conn = establish_connection();

    diesel::delete(assets.filter(id.eq(asset_id)))
        .execute(&mut conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[command]
fn add_transaction(new_tx: NewTransaction) -> Result<(), String> {
    use schema::transactions::dsl::*;
    let mut conn = establish_connection();

    diesel::insert_into(transactions)
        .values((
            asset.eq(new_tx.asset),
            quantity.eq(new_tx.quantity),
            price.eq(new_tx.price),
            date.eq(new_tx.date),
            category.eq(new_tx.category),
        ))
        .execute(&mut conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[command]
fn delete_transaction(transaction_id: i32) -> Result<(), String> {
    use schema::transactions::dsl::*;
    let mut conn = establish_connection();

    diesel::delete(transactions.filter(id.eq(transaction_id)))
        .execute(&mut conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn search_asset_twelve_data(query: String) -> Result<serde_json::Value, String> {
    let api_key = std::env::var("TWELVE_API_KEY")
        .map_err(|_| "Clé TWELVE_API_KEY introuvable".to_string())?;

    let url = format!(
        "https://api.twelvedata.com/symbol_search?symbol={}&apikey={}",
        urlencoding::encode(&query),
        api_key
    );

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Erreur requête: {}", e))?;

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Erreur JSON: {}", e))?;

    Ok(json)
}
