<template>
  <div>
    <h2>Transactions</h2>
    <ul>
      <li
        v-for="transaction in transactionStore.transactions"
        :key="transaction.id"
      >
        {{ transaction.asset }} - {{ transaction.quantity }} @
        {{ transaction.price }}€
      </li>
    </ul>
  </div>
</template>
<script setup>
import { onMounted } from "vue";
import { useTransactionStore } from "../stores/transactions";

const transactionStore = useTransactionStore();

onMounted(() => {
  transactionStore.fetchTransactions();
});
</script>
