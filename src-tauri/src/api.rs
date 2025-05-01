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

#[tauri::command]
pub async fn search_stock_yahoo(query: String) -> Result<serde_json::Value, String> {
    let url = format!(
        "https://query1.finance.yahoo.com/v1/finance/search?q={}",
        urlencoding::encode(&query)
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await
        .map_err(|e| format!("Erreur requête Yahoo Search: {}", e))?;

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Erreur JSON Yahoo Search: {}", e))?;

    Ok(json)
}

#[tauri::command]
pub async fn get_stock_price_yahoo(stock_id: String) -> Result<f64, String> {
    let url = format!(
        "https://query1.finance.yahoo.com/v8/finance/chart/{}",
        urlencoding::encode(&stock_id)
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await
        .map_err(|e| format!("Erreur requête Yahoo Search: {}", e))?;

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Erreur JSON Yahoo Finance: {}", e))?;

    // Extraire le prix actuel
    let price = json["chart"]["result"][0]["meta"]["regularMarketPrice"]
        .as_f64()
        .ok_or("Prix non trouvé dans la réponse Yahoo")?;

    Ok(price)
}
