<template>
  <DashboardWidget title="📜 Dernière(s) transaction(s)">
    <div class="flex justify-end gap-2 mb-2 text-sm">
      <button
        v-for="n in [1, 5, 10]"
        :key="n"
        @click="limit = n"
        :class="[
          'px-2 py-1 rounded border',
          limit === n
            ? 'bg-blue-500 text-white border-blue-500'
            : 'bg-white text-gray-700 border-gray-300 hover:bg-gray-100',
        ]"
      >
        {{ n }}
      </button>
    </div>
    <ul class="space-y-2">
      <li
        v-for="transaction in limitedTransactions"
        :key="transaction.id"
        class="border-b pb-1 text-sm"
      >
        <strong>{{ transaction.asset }}</strong> - {{ transaction.quantity }} @
        {{ transaction.price }}€
        <span class="block text-gray-500 text-xs">{{ transaction.date }}</span>
      </li>
    </ul>
  </DashboardWidget>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useTransactionStore } from "../../stores/transactions";
import DashboardWidget from "./DashboardWidget.vue";

const transactionStore = useTransactionStore();
const limit = ref(1); // Par défaut : 1 transaction

const limitedTransactions = computed(() =>
  transactionStore.transactions.slice(0, limit.value)
);

onMounted(() => {
  transactionStore.fetchTransactions();
});
</script>
