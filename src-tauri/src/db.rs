use crate::establish_connection;
use crate::models::{Asset, Transaction};
use crate::schema::{assets, transactions};
use diesel::prelude::*;

/// Récupérer tous les actifs suivis
pub fn get_all_assets() -> Vec<Asset> {
    let mut conn = establish_connection();
    assets::table
        .load::<Asset>(&mut conn)
        .expect("Erreur lors de la récupération des actifs")
}
pub fn get_all_transactions() -> Vec<Transaction> {
    let mut conn = establish_connection();
    transactions::table
        .load::<Transaction>(&mut conn)
        .expect("Erreur lors de la récupération des transactions")
}
