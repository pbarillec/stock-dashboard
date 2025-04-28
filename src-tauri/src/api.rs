use reqwest;
use tauri::command;
use urlencoding::encode;

#[command]
pub async fn search_crypto_coingecko(query: String) -> Result<serde_json::Value, String> {
    let url = format!(
        "https://api.coingecko.com/api/v3/search?query={}",
        encode(&query)
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

#[command]
pub async fn get_crypto_price_coingecko(crypto_id: String) -> Result<f64, String> {
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=eur",
        encode(&crypto_id)
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
