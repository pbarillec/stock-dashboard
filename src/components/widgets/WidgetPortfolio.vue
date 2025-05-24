<template>
  <DashboardWidget title="💼 Valeur approximative du portefeuille">
    <div class="text-sm space-y-3">
      <!-- Mode Tous -->
      <template v-if="filter === 'all'">
        <template v-if="performanceMode === 'net'">
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
        </template>

        <template v-else>
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
        </template>
      </template>

      <!-- Crypto uniquement -->
      <template v-else-if="filter === 'crypto'">
        <template v-if="performanceMode === 'net'">
          <p>
            <strong>📊 Performance :</strong>
            <span :class="getClass(percentCrypto)">
              {{ formatPercent(percentCrypto) }}
            </span>
          </p>
        </template>

        <template v-else>
          <p><strong>💰 Investi :</strong> {{ investedCrypto }} €</p>
          <p><strong>📈 Valeur actuelle :</strong> {{ totalCrypto }} €</p>
          <p>
            <strong>📊 Performance :</strong>
            <span :class="getClass(percentCrypto)">
              {{ formatPercent(percentCrypto) }}
            </span>
          </p>
        </template>
      </template>

      <!-- Actions uniquement -->
      <template v-else-if="filter === 'stock'">
        <template v-if="performanceMode === 'net'">
          <p>
            <strong>📊 Performance :</strong>
            <span :class="getClass(percentStock)">
              {{ formatPercent(percentStock) }}
            </span>
          </p>
        </template>

        <template v-else>
          <p><strong>💰 Investi :</strong> {{ investedStock }} €</p>
          <p><strong>📈 Valeur actuelle :</strong> {{ totalStock }} €</p>
          <p>
            <strong>📊 Performance :</strong>
            <span :class="getClass(percentStock)">
              {{ formatPercent(percentStock) }}
            </span>
          </p>
        </template>
      </template>
    </div>
  </DashboardWidget>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useTransactionStore } from "../../stores/transactions";
import { useAssetStore } from "../../stores/assets";
import { useFiltersStore } from "../../stores/filters";
import DashboardWidget from "./DashboardWidget.vue";

const transactionStore = useTransactionStore();
const assetStore = useAssetStore();
const filtersStore = useFiltersStore();

const filter = computed(() => filtersStore.viewMode);
const performanceMode = computed(() => filtersStore.performanceMode);
const timeRange = computed(() => filtersStore.timeRange);

function isDateInRange(dateStr: string): boolean {
  if (timeRange.value === "all") return true;

  const now = new Date();
  const date = new Date(dateStr);
  const yearStart = new Date(now.getFullYear(), 0, 1);

  switch (timeRange.value) {
    case "1d":
      return now.getTime() - date.getTime() <= 24 * 60 * 60 * 1000;
    case "7d":
      return now.getTime() - date.getTime() <= 7 * 24 * 60 * 60 * 1000;
    case "1m":
      return now.getTime() - date.getTime() <= 30 * 24 * 60 * 60 * 1000;
    case "1y":
      return now.getTime() - date.getTime() <= 365 * 24 * 60 * 60 * 1000;
    case "ytd":
      return date >= yearStart;
    default:
      return true;
  }
}

const filteredTransactions = computed(() =>
  transactionStore.transactions.filter((tx) => isDateInRange(tx.date))
);

// 💸 Investi (prix d'achat)
const investedTotal = computed(() => {
  return filteredTransactions.value
    .reduce((acc, tx) => acc + tx.quantity * tx.price, 0)
    .toFixed(2);
});

const investedCrypto = computed(() => {
  return filteredTransactions.value
    .filter((tx) => tx.category === "crypto")
    .reduce((acc, tx) => acc + tx.quantity * tx.price, 0)
    .toFixed(2);
});

const investedStock = computed(() => {
  return filteredTransactions.value
    .filter((tx) => tx.category === "stock")
    .reduce((acc, tx) => acc + tx.quantity * tx.price, 0)
    .toFixed(2);
});

// 🔄 Valeur actuelle (prix temps réel)
const total = computed(() => {
  return filteredTransactions.value
    .reduce((acc, tx) => {
      const asset = assetStore.assets.find((a) => a.symbol === tx.asset);
      const price = asset ? assetStore.prices[asset.id] ?? tx.price : tx.price;
      return acc + tx.quantity * price;
    }, 0)
    .toFixed(2);
});

const totalCrypto = computed(() => {
  return filteredTransactions.value
    .filter((tx) => tx.category === "crypto")
    .reduce((acc, tx) => {
      const asset = assetStore.assets.find((a) => a.symbol === tx.asset);
      const price = asset ? assetStore.prices[asset.id] ?? tx.price : tx.price;
      return acc + tx.quantity * price;
    }, 0)
    .toFixed(2);
});

const totalStock = computed(() => {
  return filteredTransactions.value
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
