<template>
  <div
    class="bg-white p-4 rounded-lg shadow-md border border-gray-200 max-w-md mx-auto"
  >
    <h2 class="text-lg font-semibold mb-4">➕ Ajouter un actif</h2>
    <form @submit.prevent="submit">
      <div class="mb-3">
        <label class="block text-sm font-medium mb-1"
          >Symbole (ex: BTC, AAPL)</label
        >
        <input v-model="form.symbol" type="text" class="input" required />
      </div>

      <div class="mb-3">
        <label class="block text-sm font-medium mb-1">Nom complet</label>
        <input v-model="form.name" type="text" class="input" required />
      </div>

      <div class="mb-4">
        <label class="block text-sm font-medium mb-1">Catégorie</label>
        <select v-model="form.category" class="input" required>
          <option value="crypto">Crypto</option>
          <option value="stock">Bourse</option>
        </select>
      </div>

      <div class="mb-4">
        <label class="block text-sm font-medium mb-1"
          >Identifiant API (facultatif)</label
        >
        <input v-model="form.api_id" type="text" class="input" />
      </div>

      <button
        type="submit"
        class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded"
      >
        Ajouter
      </button>
    </form>

    <hr class="my-6" />

    <h3 class="text-md font-semibold mb-2">📋 Actifs suivis</h3>
    <ul class="text-sm space-y-1">
      <li
        v-for="asset in assetStore.assets"
        :key="asset.id"
        class="border-b pb-1"
      >
        <strong>{{ asset.symbol }}</strong> - {{ asset.name }} ({{
          asset.category
        }})
        <button
          @click="assetStore.deleteAsset(asset.id)"
          class="text-red-600 hover:scale-110 transition"
        >
          ❌
        </button>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { reactive, onMounted } from "vue";
import { useAssetStore } from "../stores/assets";
import type { Asset } from "../models/Asset";

const assetStore = useAssetStore();

const form = reactive<Omit<Asset, "id">>({
  symbol: "",
  name: "",
  category: "crypto",
  api_id: "",
});

onMounted(() => {
  assetStore.fetchAssets();
});

async function submit() {
  try {
    await assetStore.addAsset(form);
    assetStore.fetchAssets(); // 🔄 mettre à jour la liste locale
    form.symbol = "";
    form.name = "";
    form.category = "crypto";
    form.api_id = "";
  } catch (error) {
    console.error("Erreur lors de l'ajout de l'actif :", error);
  }
}
</script>

<style scoped>
.input {
  @apply w-full border rounded px-3 py-2 text-sm focus:outline-none focus:ring focus:ring-blue-200;
}
</style>
