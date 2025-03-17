<template>
  <div class="bg-white p-4 rounded-lg shadow-md border border-gray-200">
    <h2 class="text-xl font-semibold mb-4 flex items-center">
      📜 Transactions
    </h2>
    <ul class="space-y-2">
      <li
        v-for="transaction in transactionStore.transactions"
        :key="transaction.id"
        class="border-b pb-2"
      >
        <strong>{{ transaction.asset }}</strong> - {{ transaction.quantity }} @
        {{ transaction.price }}€
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from "vue";
import { useTransactionStore } from "../stores/transactions";

const transactionStore = useTransactionStore();

onMounted(() => {
  transactionStore.fetchTransactions();
});
</script>
