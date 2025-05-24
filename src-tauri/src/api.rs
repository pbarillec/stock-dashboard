use chrono::{Datelike, NaiveDate, TimeZone, Utc};
use reqwest;
use serde::Deserialize;
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

#[tauri::command]
pub async fn get_crypto_price_on_date_coingecko(
    crypto_id: String,
    date: String,
) -> Result<f64, String> {
    use chrono::{Datelike, NaiveDate};

    #[derive(Deserialize)]
    struct CoinGeckoHistoryResponse {
        market_data: Option<MarketData>,
    }

    #[derive(Deserialize)]
    struct MarketData {
        current_price: std::collections::HashMap<String, f64>,
    }

    let parsed_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| "Format de date invalide (attendu : YYYY-MM-DD)".to_string())?;

    let url_date = format!(
        "{:02}-{:02}-{}",
        parsed_date.day(),
        parsed_date.month(),
        parsed_date.year()
    );

    let url = format!(
        "https://api.coingecko.com/api/v3/coins/{}/history?date={}&localization=false",
        crypto_id, url_date
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await
        .map_err(|e| format!("Erreur requête CoinGecko: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Erreur HTTP CoinGecko: {}", response.status()));
    }

    let json: CoinGeckoHistoryResponse = response
        .json()
        .await
        .map_err(|e| format!("Erreur parsing JSON: {}", e))?;

    let price = json
        .market_data
        .and_then(|m| m.current_price.get("eur").copied());

    price.ok_or_else(|| "Prix introuvable dans la réponse".to_string())
}

#[command]
pub async fn get_stock_price_on_date_yahoo(stock_id: String, date: String) -> Result<f64, String> {
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct YahooResponse {
        chart: YahooChart,
    }

    #[derive(Deserialize)]
    struct YahooChart {
        result: Vec<YahooResult>,
    }

    #[derive(Deserialize)]
    struct YahooResult {
        timestamp: Vec<i64>,
        indicators: YahooIndicators,
    }

    #[derive(Deserialize)]
    struct YahooIndicators {
        quote: Vec<YahooQuote>,
    }

    #[derive(Deserialize)]
    struct YahooQuote {
        close: Vec<Option<f64>>,
    }

    let parsed_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| "Format de date invalide (attendu : YYYY-MM-DD)".to_string())?;

    let now = Utc::now().naive_utc().date();
    let delta_days = (now - parsed_date).num_days().abs() + 2; // +2 jours de marge

    let url = format!(
        "https://query1.finance.yahoo.com/v8/finance/chart/{}?interval=1d&range={}d",
        stock_id, delta_days
    );

    let response = reqwest::Client::new()
        .get(&url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await
        .map_err(|e| format!("Erreur requête Yahoo Finance: {}", e))?;

    let data: YahooResponse = response
        .json()
        .await
        .map_err(|e| format!("Erreur parsing JSON Yahoo: {}", e))?;

    let result = data
        .chart
        .result
        .get(0)
        .ok_or("Résultat Yahoo Finance vide")?;

    let timestamps = &result.timestamp;
    let closes = &result
        .indicators
        .quote
        .get(0)
        .ok_or("Données manquantes")?
        .close;

    for (i, ts) in timestamps.iter().enumerate() {
        let ts_date = Utc.timestamp_opt(*ts, 0).unwrap().date_naive();
        if ts_date == parsed_date {
            let price = closes.get(i).copied().flatten();
            return price.ok_or("Aucune valeur de clôture ce jour-là".to_string());
        }
    }

    Err("Date non trouvée dans les données Yahoo".to_string())
}
