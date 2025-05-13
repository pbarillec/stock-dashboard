<template>
  <DashboardWidget title="📈 Mes actifs détenus">
    <ul class="space-y-2 text-sm max-h-80 overflow-y-auto pr-1">
      <li
        v-for="holding in heldAssets"
        :key="holding.asset"
        class="border rounded p-2 bg-white shadow-sm flex justify-between items-center cursor-pointer hover:bg-gray-50"
        @click="selected = holding"
      >
        <div>
          <strong>{{ holding.asset }}</strong> - {{ holding.name }}
          <div class="text-xs text-gray-500 mt-1">
            Quantité : {{ holding.quantity }} • Valeur :
            {{ holding.currentValue.toFixed(2) }} €
          </div>
        </div>
        <div
          :class="[
            'text-sm font-semibold',
            holding.performance >= 0 ? 'text-green-600' : 'text-red-600',
          ]"
        >
          {{ holding.performance >= 0 ? "+" : ""
          }}{{ holding.performance.toFixed(2) }}%
        </div>
      </li>
    </ul>

    <p v-if="heldAssets.length === 0" class="text-sm text-gray-500">
      Aucun actif détenu pour le moment.
    </p>

    <!-- Popup d'infos détaillées -->
    <div
      v-if="selected"
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
    >
      <div class="bg-white p-6 rounded shadow-md max-w-md w-full relative">
        <button
          @click="selected = null"
          class="absolute top-2 right-2 text-gray-500 hover:text-black"
        >
          ✖
        </button>

        <h3 class="text-lg font-semibold mb-4">
          {{ selected.asset }} - {{ selected.name }}
        </h3>

        <div class="text-sm text-gray-700 space-y-1">
          <p><strong>Quantité détenue :</strong> {{ selected.quantity }}</p>
          <p>
            <strong>Montant investi :</strong>
            {{ selected.invested.toFixed(2) }} €
          </p>
          <p>
            <strong>Valeur actuelle :</strong>
            {{ selected.currentValue.toFixed(2) }} €
          </p>
          <p><strong>PRU :</strong> {{ selected.pru.toFixed(2) }} €</p>
          <p>
            <strong>Performance :</strong>
            <span
              :class="
                selected.performance >= 0 ? 'text-green-600' : 'text-red-600'
              "
            >
              {{ selected.performance >= 0 ? "+" : ""
              }}{{ selected.performance.toFixed(2) }}%
            </span>
          </p>
          <p>
            <strong>Prix actuel :</strong>
            {{ selected.currentPrice.toFixed(2) }} €
          </p>
        </div>

        <div class="mt-4">
          <h4 class="text-md font-semibold mb-2">📜 Transactions liées</h4>
          <ul class="text-xs space-y-1 max-h-40 overflow-y-auto pr-1">
            <li
              v-for="tx in getTransactionsFor(selected.asset)"
              :key="tx.id"
              class="border-b pb-1"
            >
              {{ tx.date }} - {{ tx.quantity }} @ {{ tx.price }}€
            </li>
          </ul>
        </div>
      </div>
    </div>
  </DashboardWidget>
</template>
<script setup lang="ts">
import { computed, ref } from "vue";
import DashboardWidget from "./DashboardWidget.vue";
import { useTransactionStore } from "../../stores/transactions";
import { useAssetStore } from "../../stores/assets";
import { useDashboardStore } from "../../stores/dashboard";

const transactionStore = useTransactionStore();
const assetStore = useAssetStore();
const dashboardStore = useDashboardStore();

const selected = ref<any | null>(null);

const heldAssets = computed(() => {
  const filter = dashboardStore.filter;
  const result: {
    asset: string;
    name: string;
    quantity: number;
    invested: number;
    currentPrice: number;
    currentValue: number;
    pru: number;
    performance: number;
  }[] = [];

  assetStore.assets.forEach((asset) => {
    const relatedTx = transactionStore.transactions.filter(
      (tx) =>
        tx.asset === asset.symbol &&
        (filter === "all" || tx.category === filter)
    );

    const totalQuantity = relatedTx.reduce((acc, tx) => acc + tx.quantity, 0);
    if (totalQuantity === 0) return; // skip if none held

    const invested = relatedTx.reduce(
      (acc, tx) => acc + tx.quantity * tx.price,
      0
    );

    const price = assetStore.prices[asset.id] ?? relatedTx[0]?.price ?? 0;
    const currentValue = totalQuantity * price;
    const pru = invested / totalQuantity;
    const performance =
      invested === 0 ? 0 : ((currentValue - invested) / invested) * 100;

    result.push({
      asset: asset.symbol,
      name: asset.name,
      quantity: totalQuantity,
      invested,
      currentPrice: price,
      currentValue,
      pru,
      performance,
    });
  });

  return result;
});

function getTransactionsFor(assetSymbol: string) {
  return transactionStore.transactions.filter((tx) => tx.asset === assetSymbol);
}
</script>

<style scoped>
.input {
  @apply w-full border rounded px-3 py-2 text-sm focus:outline-none focus:ring focus:ring-blue-200;
}
</style>
