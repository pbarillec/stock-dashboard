mod api;
mod db;
mod models;
mod schema;

pub use api::*;
pub use db::*;

use diesel::prelude::*;
use std::env;

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
            get_crypto_price_coingecko,
            search_stock_yahoo,
            get_stock_price_yahoo,
            get_crypto_price_on_date_coingecko,
            get_stock_price_on_date_yahoo
        ])
        .run(tauri::generate_context!())
        .expect("Erreur lors du lancement de l'application");
}

// Fonction pour établir la connexion à la base SQLite
pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL doit être défini");
    SqliteConnection::establish(&database_url).expect("Erreur lors de la connexion à la base")
}
