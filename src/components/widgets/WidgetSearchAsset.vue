<template>
  <DashboardWidget title="🔍 Rechercher un actif (Twelve Data)">
    <form @submit.prevent="search" class="flex gap-2 mb-4">
      <input
        v-model="query"
        type="text"
        class="input flex-1"
        placeholder="Ex: Gaztransport, Air Liquide, Bitcoin..."
      />
      <button
        type="submit"
        class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded"
      >
        Rechercher
      </button>
    </form>
    <div class="flex items-center gap-4 text-xs text-gray-600 mb-2">
      <div class="flex items-center gap-1">
        <div class="w-3 h-3 rounded-full bg-green-500"></div>
        Monnaie numérique
      </div>
      <div class="flex items-center gap-1">
        <div class="w-3 h-3 rounded-full bg-blue-500"></div>
        Action / ETF
      </div>
    </div>
    <ul
      v-if="results.length"
      class="space-y-2 text-sm max-h-80 overflow-y-auto pr-1"
    >
      <li
        v-for="(result, index) in results"
        :key="index"
        class="border rounded p-2 bg-white shadow-sm"
        :class="{
          'border-green-500 bg-green-50':
            result.instrument_type === 'Digital Currency',
          'border-blue-500 bg-blue-50':
            result.instrument_type === 'Common Stock' ||
            result.instrument_type === 'ETF',
        }"
      >
        <div class="flex justify-between items-center mb-1">
          <strong>
            {{ result.instrument_name }}
          </strong>
          <button
            @click="copyToClipboard(result.symbol)"
            class="text-xs text-blue-600 hover:underline"
          >
            Copier
          </button>
        </div>
        <div class="text-gray-600 text-xs mt-1">
          {{ result.exchange }} • {{ result.instrument_type }} •
          {{ result.country }}
        </div>
      </li>
    </ul>

    <p v-else-if="searched && !results.length" class="text-sm text-gray-500">
      Aucun résultat trouvé.
    </p>
  </DashboardWidget>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import DashboardWidget from "./DashboardWidget.vue";

const query = ref("");
const results = ref<any[]>([]);
const searched = ref(false);

async function search() {
  searched.value = false;
  results.value = [];

  if (!query.value.trim()) return;

  try {
    const res = await invoke("search_asset_twelve_data", {
      query: query.value,
    });

    const data = (res as any).data || [];
    results.value = data;
    searched.value = true;
  } catch (error) {
    console.error("Erreur lors de la recherche :", error);
  }
}

function copyToClipboard(text: string) {
  navigator.clipboard.writeText(text).then(() => {
    console.log("Symbol copié :", text);
  });
}
</script>

<style scoped>
.input {
  @apply w-full border rounded px-3 py-2 text-sm focus:outline-none focus:ring focus:ring-blue-200;
}
</style>
