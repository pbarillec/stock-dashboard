use crate::establish_connection;
use crate::models::{Asset, NewAsset, NewTransaction, Transaction};
use crate::schema::*;
use diesel::deserialize::QueryableByName;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Integer;
use diesel::RunQueryDsl;
use tauri::command;

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

#[command]
pub fn fetch_transactions() -> Vec<Transaction> {
    super::db::get_all_transactions()
}

#[command]
pub fn fetch_assets() -> Vec<Asset> {
    super::db::get_all_assets()
}

#[command]
pub fn add_asset(new_asset: NewAsset) -> Result<Asset, String> {
    use crate::schema::assets::dsl::*;
    let mut conn = super::establish_connection();

    diesel::insert_into(assets)
        .values((
            symbol.eq(&new_asset.symbol),
            name.eq(&new_asset.name),
            category.eq(&new_asset.category),
            api_id.eq(&new_asset.api_id),
        ))
        .execute(&mut conn)
        .map_err(|e| e.to_string())?;

    #[derive(QueryableByName)]
    struct LastInsertRowId {
        #[diesel(sql_type = Integer)]
        #[diesel(column_name = "last_insert_rowid()")]
        value: i32,
    }

    // Récupérer l'ID généré automatiquement
    let last_id = sql_query("SELECT last_insert_rowid()")
        .get_result::<LastInsertRowId>(&mut conn)
        .map_err(|e| e.to_string())?
        .value;

    // Charger l'actif complet via son ID
    let inserted_asset = assets
        .filter(id.eq(last_id))
        .first::<Asset>(&mut conn)
        .map_err(|e| e.to_string())?;

    Ok(inserted_asset)
}

#[command]
pub fn delete_asset(asset_id: i32) -> Result<(), String> {
    use self::assets::dsl::*;
    let mut conn = super::establish_connection();

    diesel::delete(assets.filter(id.eq(asset_id)))
        .execute(&mut conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[command]
pub fn add_transaction(new_tx: NewTransaction) -> Result<Transaction, String> {
    use crate::schema::transactions::dsl::*;
    let mut conn = super::establish_connection();

    // Insertion
    diesel::insert_into(transactions)
        .values((
            asset.eq(&new_tx.asset),
            quantity.eq(new_tx.quantity),
            price.eq(new_tx.price),
            date.eq(&new_tx.date),
            category.eq(&new_tx.category),
        ))
        .execute(&mut conn)
        .map_err(|e| e.to_string())?;

    // Récupération de l'ID inséré
    #[derive(diesel::QueryableByName)]
    struct LastInsertRowId {
        #[diesel(sql_type = Integer)]
        #[diesel(column_name = "last_insert_rowid()")]
        value: i32,
    }

    let last_id = diesel::sql_query("SELECT last_insert_rowid()")
        .get_result::<LastInsertRowId>(&mut conn)
        .map_err(|e| e.to_string())?
        .value;

    // Recharger la transaction complète
    let inserted = transactions
        .filter(id.eq(last_id))
        .first::<Transaction>(&mut conn)
        .map_err(|e| e.to_string())?;

    Ok(inserted)
}
#[command]
pub fn delete_transaction(transaction_id: i32) -> Result<(), String> {
    use self::transactions::dsl::*;
    let mut conn = super::establish_connection();

    diesel::delete(transactions.filter(id.eq(transaction_id)))
        .execute(&mut conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}
