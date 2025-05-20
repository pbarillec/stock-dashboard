import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { Transaction } from "../models/Transaction";
import { useFiltersStore } from "./filters";
import {
  fetchTransactions as fetchTransactionsApi,
  addTransaction as addTransactionApi,
  deleteTransaction as deleteTransactionApi,
  getCryptoPrice,
  getStockPrice,
} from "@/api";
import { Asset } from "@/models/Asset";

export const useTransactionStore = defineStore("transactions", () => {
  const transactions = ref<Transaction[]>([]);
  const realTimePrices = ref<Record<string, number>>({});

  const filteredTransactions = computed(() => {
    const filtersStore = useFiltersStore();

    return transactions.value.filter((t) =>
      filtersStore.viewMode === "all"
        ? true
        : t.category === filtersStore.viewMode
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

  async function fetchAllPrices(assets: Asset[]) {
    for (const asset of assets) {
      try {
        let price: number | null = null;

        if (asset.category === "crypto" && asset.api_id) {
          price = await getCryptoPrice(asset.api_id); // appel à l'api.ts
        } else if (asset.category === "stock" && asset.api_id) {
          price = await getStockPrice(asset.api_id); // appel à l'api.ts
        }

        if (price !== null) {
          realTimePrices.value[asset.symbol] = price;
        }
      } catch (error) {
        console.error(
          `Erreur lors de la récupération du prix pour ${asset.symbol}:`,
          error
        );
      }
    }
  }

  return {
    transactions,
    realTimePrices,
    filteredTransactions,
    fetchTransactions,
    addTransaction,
    deleteTransaction,
    fetchAllPrices,
  };
});
