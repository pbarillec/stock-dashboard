import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Transaction } from "../models/Transaction";

export const useTransactionStore = defineStore("transactions", () => {
  const transactions = ref<Transaction[]>([]);

  async function fetchTransactions() {
    try {
      transactions.value = await invoke<Transaction[]>("fetch_transactions");
    } catch (error) {
      console.error("Erreur lors de la récupération des transactions :", error);
    }
  }

  return { transactions, fetchTransactions };
});
