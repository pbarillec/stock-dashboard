<template>
  <div class="flex gap-2 flex-wrap justify-center">
    <button
      v-for="range in ranges"
      :key="range.value"
      @click="setTimeRange(range.value)"
      :class="[
        'px-2 py-1 rounded border text-xs',
        timeRange === range.value
          ? 'bg-purple-600 text-white border-purple-600'
          : 'bg-white text-gray-700 border-gray-300 hover:bg-gray-100',
      ]"
    >
      {{ range.label }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { useFiltersStore, TimeRange } from "@/stores/filters";
import { storeToRefs } from "pinia";

const filters = useFiltersStore();
const { timeRange } = storeToRefs(filters);
const { setTimeRange } = filters;

const ranges: { label: string; value: TimeRange }[] = [
  { label: "1j", value: "1d" },
  { label: "7j", value: "7d" },
  { label: "1m", value: "1m" },
  { label: "YTD", value: "ytd" },
  { label: "1an", value: "1y" },
  { label: "Tout", value: "all" },
];
</script>
