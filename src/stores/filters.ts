import { defineStore } from "pinia";
import { ref } from "vue";

export type ViewMode = "all" | "crypto" | "stock";
export type PerformanceMode = "global" | "net";

export const useFiltersStore = defineStore("filters", () => {
  const viewMode = ref<ViewMode>("all");
  const performanceMode = ref<PerformanceMode>("global");

  function setViewMode(mode: ViewMode) {
    viewMode.value = mode;
  }

  function setPerformanceMode(mode: PerformanceMode) {
    performanceMode.value = mode;
  }

  return {
    viewMode,
    performanceMode,
    setViewMode,
    setPerformanceMode,
  };
});
