<template>
  <div class="flex flex-wrap justify-center gap-4 mb-6">
    <!-- Vue : Tous / Crypto / Actions -->
    <div class="flex gap-2">
      <button
        v-for="mode in ['all', 'crypto', 'stock']"
        :key="mode"
        @click="setViewMode(mode as ViewMode)"
        :class="[
          'px-3 py-1 rounded border text-sm capitalize',
          viewMode === mode
            ? 'bg-blue-600 text-white border-blue-600'
            : 'bg-white text-gray-700 border-gray-300 hover:bg-gray-100',
        ]"
      >
        {{ mode === "all" ? "Tous" : mode === "crypto" ? "Crypto" : "Actions" }}
      </button>
    </div>

    <!-- Mode de calcul : Global / Performance nette -->
    <div class="flex gap-2">
      <button
        v-for="mode in ['global', 'net']"
        :key="mode"
        @click="setPerformanceMode(mode as PerformanceMode)"
        :class="[
          'px-3 py-1 rounded border text-sm capitalize',
          performanceMode === mode
            ? 'bg-green-600 text-white border-green-600'
            : 'bg-white text-gray-700 border-gray-300 hover:bg-gray-100',
        ]"
      >
        {{ mode === "global" ? "Global" : "Performance nette" }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useFiltersStore, ViewMode, PerformanceMode } from "@/stores/filters";
import { storeToRefs } from "pinia";

const filters = useFiltersStore();
const { viewMode, performanceMode } = storeToRefs(filters);
const { setViewMode, setPerformanceMode } = filters;
</script>
