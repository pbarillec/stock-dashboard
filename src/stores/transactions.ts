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

  async function addTransaction(newTx: Omit<Transaction, "id">) {
    try {
      await invoke("add_transaction", {
        newTx,
      });
      transactions.value.push({
        id: Date.now(), // ID temporaire simulé
        ...newTx,
      });
    } catch (error) {
      console.error("Erreur lors de l'ajout de la transaction :", error);
    }
  }

  async function deleteTransaction(transactionId: number) {
    try {
      await invoke("delete_transaction", {
        transactionId: transactionId, // Utilisez transactionId au lieu de transaction_id
      });
      transactions.value = transactions.value.filter(
        (t) => t.id !== transactionId
      );
    } catch (error) {
      console.error("Erreur lors de la suppression de la transaction :", error);
    }
  }

  return { transactions, fetchTransactions, addTransaction, deleteTransaction };
});
