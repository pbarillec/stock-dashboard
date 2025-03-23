<template>
  <DashboardWidget title="💼 Valeur approximative du portefeuille">
    <div class="text-sm space-y-1">
      <p><strong>Total (approx.) :</strong> {{ total }} €</p>
      <p class="text-green-700">Crypto : {{ totalCrypto }} €</p>
      <p class="text-blue-700">Actions : {{ totalStock }} €</p>
    </div>
  </DashboardWidget>
</template>

<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useTransactionStore } from "../../stores/transactions";
import DashboardWidget from "./DashboardWidget.vue";

const store = useTransactionStore();

const total = computed(() =>
  store.transactions.reduce((acc, t) => acc + t.quantity * t.price, 0)
);

const totalCrypto = computed(() =>
  store.transactions
    .filter((t) => t.category === "crypto")
    .reduce((acc, t) => acc + t.quantity * t.price, 0)
);

const totalStock = computed(() =>
  store.transactions
    .filter((t) => t.category === "stock")
    .reduce((acc, t) => acc + t.quantity * t.price, 0)
);

onMounted(() => {
  store.fetchTransactions();
});
</script>
