<template>
  <DashboardWidget title="🔍 Rechercher une action, un etf (YahooFinance)">
    <form @submit.prevent="search" class="flex gap-2 mb-4">
      <input
        v-model="query"
        type="text"
        class="input flex-1"
        placeholder="Ex: Air Liquide, Msci World, Total..."
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
        class="border rounded p-2 bg-white shadow-sm flex justify-between items-center text-sm"
        @click="selectStock(result)"
      >
        <div>
          <strong>{{
            result.shortname || result.longname || result.symbol
          }}</strong>
          <div class="text-xs text-gray-500">
            Symbol : {{ result.symbol }} • Exchange : {{ result.exchange }} •
            Type : {{ result.quoteType }}
          </div>
        </div>
        <button
          @click.stop="copyToClipboard(result.symbol)"
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
        <h3 class="text-lg font-semibold mb-2">
          {{ selected.shortname || selected.longname || selected.symbol }}
        </h3>
        <p class="text-sm text-gray-600 mb-2">
          <strong>Symbol :</strong> {{ selected.symbol }}<br />
          <strong>Exchange :</strong> {{ selected.exchange }}<br />
          <strong>Type :</strong> {{ selected.quoteType }}<br />
          <span v-if="selected.price !== null">
            <strong>Prix actuel :</strong> {{ selected.price }} €
          </span>
          <span v-else class="text-gray-400 italic"> Prix non chargé </span>
        </p>
      </div>
    </div>
  </DashboardWidget>
</template>
<script setup lang="ts">
import { ref } from "vue";
import { searchStock, getStockPrice } from "@/api";
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
    console.log("Recherche de cryptomonnaies avec CoinGecko :", query.value);

    const res = await searchStock(query.value);
    console.log("Résultats de la recherche CoinGecko :", res);

    results.value = res;
    searched.value = true;
  } catch (error) {
    console.error("Erreur lors de la recherche YahooFinance :", error);
  }
}

function copyToClipboard(text: string) {
  navigator.clipboard.writeText(text).then(() => {
    console.log("ID YahooFinance copié :", text);
  });
}

async function selectStock(stock: any) {
  selected.value = { ...stock, price: null };

  try {
    const price = await getStockPrice(stock.symbol);
    if (selected.value) {
      selected.value.price = price;
    }
  } catch (error) {
    console.error("Erreur lors du chargement du prix Yahoo :", error);
  }
}
</script>

<style scoped>
.input {
  @apply w-full border rounded px-3 py-2 text-sm focus:outline-none focus:ring focus:ring-blue-200;
}
</style>
