use crate::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize}; // ✅ Import du schéma généré par Diesel

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub id: i32,
    pub asset: String,
    pub quantity: f64,
    pub price: f64,
    pub date: String,
    pub category: String, // "crypto" ou "stock"
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = assets)]
pub struct Asset {
    pub id: i32,
    pub symbol: String,
    pub name: String,
    pub category: String,       // "crypto" ou "stock"
    pub api_id: Option<String>, // ID de l'API utilisée pour récupérer les données
}

#[derive(Deserialize)]
pub struct NewAsset {
    pub symbol: String,
    pub name: String,
    pub category: String,
    pub api_id: Option<String>,
}

#[derive(Deserialize)]
pub struct NewTransaction {
    pub asset: String,
    pub quantity: f64,
    pub price: f64,
    pub date: String,
    pub category: String,
}
