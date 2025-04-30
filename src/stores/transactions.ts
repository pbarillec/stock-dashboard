import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Transaction } from "../models/Transaction";
import { useDashboardStore } from "./dashboard";
import {
  fetchTransactions as fetchTransactionsApi,
  addTransaction as addTransactionApi,
  deleteTransaction as deleteTransactionApi,
} from "@/api";

export const useTransactionStore = defineStore("transactions", () => {
  const transactions = ref<Transaction[]>([]);

  const filteredTransactions = computed(() => {
    const dashboardStore = useDashboardStore();

    return transactions.value.filter((t) =>
      dashboardStore.filter === "all"
        ? true
        : t.category === dashboardStore.filter
    );
  });

  async function fetchTransactions() {
    try {
      transactions.value = await fetchTransactionsApi();
    } catch (error) {
      console.error("Erreur lors de la récupération des transactions :", error);
    }
  }

  async function addTransaction(newTx: Omit<Transaction, "id">) {
    try {
      const inserted = await addTransactionApi(newTx);
      transactions.value.push(inserted);
    } catch (error) {
      console.error("Erreur lors de l'ajout de la transaction :", error);
    }
  }

  async function deleteTransaction(transactionId: number) {
    try {
      await deleteTransactionApi(transactionId); // Appel à l'API pour supprimer une transaction
      transactions.value = transactions.value.filter(
        (t) => t.id !== transactionId
      );
    } catch (error) {
      console.error("Erreur lors de la suppression de la transaction :", error);
    }
  }

  return {
    transactions,
    filteredTransactions,
    fetchTransactions,
    addTransaction,
    deleteTransaction,
  };
});
