<template>
  <DashboardWidget title="📜 Dernière(s) transaction(s)">
    <div class="flex justify-end gap-2 mb-2 text-sm">
      <button
        v-for="option in options"
        :key="option"
        @click="limit = option"
        :class="[
          'px-2 py-1 rounded border',
          limit === option
            ? 'bg-blue-500 text-white border-blue-500'
            : 'bg-white text-gray-700 border-gray-300 hover:bg-gray-100',
        ]"
      >
        {{ option }}
      </button>
    </div>
    <ul class="space-y-2">
      <li
        v-for="transaction in limitedTransactions"
        :key="transaction.id"
        class="border-b pb-1 text-sm"
      >
        <div class="flex justify-between items-start">
          <div>
            <strong>{{ transaction.asset }}</strong> -
            {{ transaction.quantity }} @ {{ transaction.price }}€
          </div>
          <button
            @click="transactionStore.deleteTransaction(transaction.id)"
            class="text-red-600 hover:scale-110 transition"
            title="Supprimer"
          >
            ❌
          </button>
        </div>
        <div class="text-gray-500 text-xs mt-1">{{ transaction.date }}</div>
      </li>
    </ul>
  </DashboardWidget>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useTransactionStore } from "../../stores/transactions";
import DashboardWidget from "./DashboardWidget.vue";

const transactionStore = useTransactionStore();
const options: (number | "All")[] = [5, 10, "All"];
const limit = ref<number | "All">(5);

const limitedTransactions = computed(() => {
  const sorted = [...transactionStore.filteredTransactions].sort(
    (a, b) => new Date(b.date).getTime() - new Date(a.date).getTime()
  );

  return limit.value === "All" ? sorted : sorted.slice(0, limit.value);
});

onMounted(() => {
  transactionStore.fetchTransactions();
});
</script>
