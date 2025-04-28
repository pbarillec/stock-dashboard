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
            search_crypto_coingecko,
            get_crypto_price_coingecko
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
async fn search_crypto_coingecko(query: String) -> Result<serde_json::Value, String> {
    let url = format!(
        "https://api.coingecko.com/api/v3/search?query={}",
        urlencoding::encode(&query)
    );

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Erreur requête CoinGecko: {}", e))?;

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Erreur JSON CoinGecko: {}", e))?;

    Ok(json)
}

#[tauri::command]
async fn get_crypto_price_coingecko(crypto_id: String) -> Result<f64, String> {
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=eur",
        urlencoding::encode(&crypto_id)
    );

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Erreur requête CoinGecko: {}", e))?;

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Erreur JSON CoinGecko: {}", e))?;

    let price = json
        .get(&crypto_id)
        .and_then(|c| c.get("eur"))
        .and_then(|p| p.as_f64())
        .ok_or("Prix non trouvé".to_string())?;

    Ok(price)
}
