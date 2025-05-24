import { defineStore } from "pinia";
import { ref } from "vue";

export type ViewMode = "all" | "crypto" | "stock";
export type PerformanceMode = "global" | "net";
export type TimeRange = "1d" | "7d" | "1m" | "ytd" | "1y" | "all";

export const useFiltersStore = defineStore("filters", () => {
  const viewMode = ref<ViewMode>("all");
  const performanceMode = ref<PerformanceMode>("global");
  const timeRange = ref<TimeRange>("all");

  function setViewMode(mode: ViewMode) {
    viewMode.value = mode;
  }

  function setPerformanceMode(mode: PerformanceMode) {
    performanceMode.value = mode;
  }

  function setTimeRange(range: TimeRange) {
    timeRange.value = range;
  }

  return {
    viewMode,
    performanceMode,
    timeRange,
    setViewMode,
    setPerformanceMode,
    setTimeRange,
  };
});
