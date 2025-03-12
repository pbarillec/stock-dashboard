// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    dotenvy::dotenv().ok(); // Charge les variables d'environnement (DATABASE_URL)
    stock_dashboard_lib::run(); // Lance l'application Tauri via lib.rs
}
