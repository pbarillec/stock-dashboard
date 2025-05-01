import { invoke } from "@tauri-apps/api/core";
import type { Asset } from "./models/Asset";
import type { Transaction } from "./models/Transaction";

// 🚀 Gestion des Assets
export async function fetchAssets(): Promise<Asset[]> {
  return await invoke("fetch_assets");
}

export async function addAsset(newAsset: Omit<Asset, "id">): Promise<Asset> {
  return await invoke("add_asset", { newAsset });
}

export async function deleteAsset(assetId: number): Promise<void> {
  await invoke("delete_asset", { assetId: assetId });
}

// 🚀 Gestion des Transactions
export async function fetchTransactions(): Promise<Transaction[]> {
  return await invoke("fetch_transactions");
}

export async function addTransaction(
  newTx: Omit<Transaction, "id">
): Promise<Transaction> {
  return await invoke("add_transaction", { newTx });
}

export async function deleteTransaction(transactionId: number): Promise<void> {
  await invoke("delete_transaction", { transactionId: transactionId });
}

// 🚀 CoinGecko API
export async function searchCrypto(query: string): Promise<any[]> {
  const res = await invoke<{ coins: any[] }>("search_crypto_coingecko", {
    query,
  });
  return res.coins || [];
}

export async function getCryptoPrice(cryptoId: string): Promise<number> {
  return await invoke<number>("get_crypto_price_coingecko", { cryptoId });
}

// 🚀 Yahoo Finance API
export async function searchStock(query: string): Promise<any[]> {
  const res = await invoke<{ quotes: any[] }>("search_stock_yahoo", { query });
  return res.quotes || [];
}

export async function getStockPrice(stockId: string): Promise<number> {
  return await invoke<number>("get_stock_price_yahoo", { stockId });
}
