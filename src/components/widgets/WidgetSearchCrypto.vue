<template>
  <DashboardWidget title="🔍 Rechercher une cryptomonnaie (CoinGecko)">
    <form @submit.prevent="search" class="flex gap-2 mb-4">
      <input
        v-model="query"
        type="text"
        class="input flex-1"
        placeholder="Ex: Bitcoin, Ethereum, Solana..."
      />
      <button
        type="submit"
        class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded"
      >
        Rechercher
      </button>
    </form>

    <ul
      v-if="results.length"
      class="space-y-2 text-sm max-h-80 overflow-y-auto pr-1"
    >
      <li
        v-for="(result, index) in results"
        :key="index"
        class="border rounded p-2 bg-white shadow-sm flex justify-between items-center"
        @click="selectCrypto(result)"
      >
        <div>
          <strong>{{ result.name }}</strong>
          <div class="text-xs text-gray-500">
            {{ result.symbol.toUpperCase() }}
          </div>
        </div>
        <button
          @click="copyToClipboard(result.id)"
          class="text-xs text-blue-600 hover:underline"
        >
          Copier ID
        </button>
      </li>
    </ul>

    <p v-else-if="searched && !results.length" class="text-sm text-gray-500">
      Aucun résultat trouvé.
    </p>
    <div
      v-if="selected"
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
    >
      <div class="bg-white p-6 rounded shadow-md max-w-md w-full relative">
        <button
          @click="selected = null"
          class="absolute top-2 right-2 text-gray-500 hover:text-black"
        >
          ✖
        </button>

        <div class="flex items-center gap-3 mb-4">
          <img
            v-if="selected.thumb"
            :src="selected.thumb"
            alt="Logo"
            class="w-10 h-10 rounded-full"
          />
          <h3 class="text-lg font-semibold">{{ selected.name }}</h3>
        </div>

        <p class="text-sm text-gray-600 mb-2">
          <strong>Symbol :</strong> {{ selected.symbol.toUpperCase() }}<br />
          <strong>ID CoinGecko :</strong> <code>{{ selected.id }}</code
          ><br />
          <span v-if="selected.price !== null">
            <strong>Prix :</strong> {{ selected.price }} €
          </span>
          <span v-else class="text-gray-400 italic"> Prix indisponible </span>
        </p>
      </div>
    </div>
  </DashboardWidget>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import DashboardWidget from "./DashboardWidget.vue";

const query = ref("");
const results = ref<any[]>([]);
const searched = ref(false);

const selected = ref<any | null>(null);

async function search() {
  searched.value = false;
  results.value = [];

  if (!query.value.trim()) return;

  try {
    const res = await invoke("search_crypto_coingecko", {
      query: query.value,
    });

    results.value = (res as any).coins || [];
    searched.value = true;
  } catch (error) {
    console.error("Erreur lors de la recherche CoinGecko :", error);
  }
}

function copyToClipboard(text: string) {
  navigator.clipboard.writeText(text).then(() => {
    console.log("ID CoinGecko copié :", text);
  });
}

async function selectCrypto(crypto: any) {
  selected.value = { ...crypto, price: null };

  try {
    const price = await invoke<number>("get_crypto_price_coingecko", {
      cryptoId: crypto.id,
    });
    if (selected.value) {
      selected.value.price = price;
    }
  } catch (error) {
    console.error("Erreur lors du chargement du prix :", error);
  }
}
</script>

<style scoped>
.input {
  @apply w-full border rounded px-3 py-2 text-sm focus:outline-none focus:ring focus:ring-blue-200;
}
</style>
