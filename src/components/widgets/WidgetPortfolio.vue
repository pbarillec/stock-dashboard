<template>
  <DashboardWidget title="💼 Valeur approximative du portefeuille">
    <div class="text-sm space-y-3">
      <div>
        <p><strong>💰 Investi :</strong> {{ investedTotal }} €</p>
        <p class="text-green-700">Crypto : {{ investedCrypto }} €</p>
        <p class="text-blue-700">Actions : {{ investedStock }} €</p>
      </div>

      <div>
        <p><strong>📈 Valeur actuelle :</strong> {{ total }} €</p>
        <p class="text-green-700">Crypto : {{ totalCrypto }} €</p>
        <p class="text-blue-700">Actions : {{ totalStock }} €</p>
      </div>

      <div>
        <p>
          <strong>📊 Performance :</strong>
          <span :class="getClass(percentTotal)">
            {{ formatPercent(percentTotal) }}
          </span>
        </p>
        <p>
          Crypto :
          <span :class="getClass(percentCrypto)">
            {{ formatPercent(percentCrypto) }}
          </span>
        </p>
        <p>
          Actions :
          <span :class="getClass(percentStock)">
            {{ formatPercent(percentStock) }}
          </span>
        </p>
      </div>
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

// 💸 Investi (prix d'achat)
const investedTotal = computed(() => {
  return transactionStore.transactions
    .reduce((acc, tx) => acc + tx.quantity * tx.price, 0)
    .toFixed(2);
});

const investedCrypto = computed(() => {
  return transactionStore.transactions
    .filter((tx) => tx.category === "crypto")
    .reduce((acc, tx) => acc + tx.quantity * tx.price, 0)
    .toFixed(2);
});

const investedStock = computed(() => {
  return transactionStore.transactions
    .filter((tx) => tx.category === "stock")
    .reduce((acc, tx) => acc + tx.quantity * tx.price, 0)
    .toFixed(2);
});

// 🔄 Valeur actuelle (prix temps réel)
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

// 📊 Pourcentage d'évolution
function calcPercent(current: number, invested: number) {
  if (invested === 0) return 0;
  return ((current - invested) / invested) * 100;
}

const percentTotal = computed(() =>
  calcPercent(Number(total.value), Number(investedTotal.value))
);
const percentCrypto = computed(() =>
  calcPercent(Number(totalCrypto.value), Number(investedCrypto.value))
);
const percentStock = computed(() =>
  calcPercent(Number(totalStock.value), Number(investedStock.value))
);

// 🌈 Style pour le % (vert/rouge/gris)
function getClass(percent: number) {
  if (percent > 0) return "text-green-600";
  if (percent < 0) return "text-red-600";
  return "text-gray-500";
}

function formatPercent(value: number) {
  return `${value >= 0 ? "+" : ""}${value.toFixed(2)}%`;
}
</script>

<style scoped>
.input {
  @apply w-full border rounded px-3 py-2 text-sm focus:outline-none focus:ring focus:ring-blue-200;
}
</style>
