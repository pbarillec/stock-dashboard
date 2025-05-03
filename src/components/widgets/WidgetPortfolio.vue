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
import { computed } from "vue";
import { useTransactionStore } from "../../stores/transactions";
import { useAssetStore } from "../../stores/assets";
import DashboardWidget from "./DashboardWidget.vue";

const transactionStore = useTransactionStore();
const assetStore = useAssetStore();

// 🔄 Calcul du total basé sur le prix temps réel
const total = computed(() => {
  return transactionStore.transactions
    .reduce((acc, tx) => {
      const asset = assetStore.assets.find((a) => a.symbol === tx.asset);
      const price = asset ? assetStore.prices[asset.id] ?? tx.price : tx.price;
      return acc + tx.quantity * price;
    }, 0)
    .toFixed(2);
});

const totalCrypto = computed(() => {
  return transactionStore.transactions
    .filter((tx) => tx.category === "crypto")
    .reduce((acc, tx) => {
      const asset = assetStore.assets.find((a) => a.symbol === tx.asset);
      const price = asset ? assetStore.prices[asset.id] ?? tx.price : tx.price;
      return acc + tx.quantity * price;
    }, 0)
    .toFixed(2);
});

const totalStock = computed(() => {
  return transactionStore.transactions
    .filter((tx) => tx.category === "stock")
    .reduce((acc, tx) => {
      const asset = assetStore.assets.find((a) => a.symbol === tx.asset);
      const price = asset ? assetStore.prices[asset.id] ?? tx.price : tx.price;
      return acc + tx.quantity * price;
    }, 0)
    .toFixed(2);
});
</script>
