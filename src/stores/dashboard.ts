import { ref } from "vue";
import { defineStore } from "pinia";

export type FilterMode = "all" | "crypto" | "stock";

export const useDashboardStore = defineStore("dashboard", () => {
  const filter = ref<FilterMode>("all");

  function setFilter(mode: FilterMode) {
    filter.value = mode;
  }

  return { filter, setFilter };
});
