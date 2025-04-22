<template>
  <div
    class="bg-white p-4 rounded-lg shadow-md border border-gray-200 max-w-md mx-auto"
  >
    <h2 class="text-lg font-semibold mb-4">➕ Ajouter une transaction</h2>
    <form @submit.prevent="submit">
      <div class="mb-3">
        <label class="block text-sm font-medium mb-1">Actif</label>
        <select v-model="form.asset" class="input" required>
          <option disabled value="">-- Choisir un actif --</option>
          <option
            v-for="asset in assetStore.assets"
            :key="asset.id"
            :value="asset.symbol"
          >
            {{ asset.symbol }} - {{ asset.name }}
          </option>
        </select>
      </div>

      <div class="mb-3">
        <label class="block text-sm font-medium mb-1">Quantité</label>
        <input
          v-model.number="form.quantity"
          type="number"
          min="0"
          step="any"
          class="input"
          required
        />
      </div>

      <div class="mb-3">
        <label class="block text-sm font-medium mb-1">Prix unitaire (€)</label>
        <input
          v-model.number="form.price"
          type="number"
          min="0"
          step="any"
          class="input"
          required
        />
      </div>

      <div class="mb-3">
        <label class="block text-sm font-medium mb-1">Date</label>
        <input v-model="form.date" type="date" class="input" required />
      </div>

      <div class="mb-4">
        <label class="block text-sm font-medium mb-1">Catégorie</label>
        <select v-model="form.category" class="input" required>
          <option value="crypto">Crypto</option>
          <option value="stock">Bourse</option>
        </select>
      </div>

      <button
        type="submit"
        class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded"
      >
        Ajouter
      </button>
    </form>
  </div>
</template>

<script setup lang="ts">
import { reactive, onMounted } from "vue";
import { useTransactionStore } from "../stores/transactions";
import type { Transaction } from "../models/Transaction";
import { useAssetStore } from "../stores/assets";

const assetStore = useAssetStore();
const transactionStore = useTransactionStore();

const form = reactive<Omit<Transaction, "id">>({
  asset: "",
  quantity: 0,
  price: 0,
  date: "",
  category: "crypto",
});

onMounted(() => {
  assetStore.fetchAssets();
});

async function submit() {
  try {
    await transactionStore.addTransaction(form);

    // Reset du formulaire
    form.asset = "";
    form.quantity = 0;
    form.price = 0;
    form.date = "";
    form.category = "crypto";
  } catch (error) {
    console.error("Erreur lors de l'ajout de la transaction :", error);
  }
}
</script>

<style scoped>
.input {
  @apply w-full border rounded px-3 py-2 text-sm focus:outline-none focus:ring focus:ring-blue-200;
}
</style>
