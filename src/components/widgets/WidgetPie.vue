<template>
  <DashboardWidget title="🥧 Répartition du portefeuille">
    <div class="relative h-64">
      <Pie :data="chartData" :options="chartOptions" />
    </div>
  </DashboardWidget>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { Pie } from "vue-chartjs";
import { Chart as ChartJS, ArcElement, Tooltip, Legend } from "chart.js";
import DashboardWidget from "./DashboardWidget.vue";
import { useTransactionStore } from "../../stores/transactions";
import { useFiltersStore } from "../../stores/filters";

ChartJS.register(ArcElement, Tooltip, Legend);

const transactionStore = useTransactionStore();
const filtersStore = useFiltersStore();

const viewMode = computed(() => filtersStore.viewMode);
const performanceMode = computed(() => filtersStore.performanceMode);

const chartData = computed(() => {
  let investedCrypto = 0;
  let investedStock = 0;
  let valueCrypto = 0;
  let valueStock = 0;

  for (const tx of transactionStore.transactions) {
    const price = transactionStore.realTimePrices[tx.asset] ?? tx.price;
    const value = tx.quantity * price;
    const invested = tx.quantity * tx.price;

    if (tx.category === "crypto") {
      investedCrypto += invested;
      valueCrypto += value;
    }
    if (tx.category === "stock") {
      investedStock += invested;
      valueStock += value;
    }
  }

  // Mode de calcul : valeur ou performance nette
  const dataCrypto =
    performanceMode.value === "net"
      ? valueCrypto - investedCrypto
      : valueCrypto;
  const dataStock =
    performanceMode.value === "net" ? valueStock - investedStock : valueStock;

  // On empêche les valeurs négatives (cas de pertes nettes totales)
  const safe = (v: number) => Math.max(v, 0);

  const datasets = {
    all: {
      labels: ["Crypto", "Actions"],
      data: [safe(dataCrypto), safe(dataStock)],
      colors: ["#10B981", "#3B82F6"],
    },
    crypto: {
      labels: ["Crypto"],
      data: [safe(dataCrypto)],
      colors: ["#10B981"],
    },
    stock: {
      labels: ["Actions"],
      data: [safe(dataStock)],
      colors: ["#3B82F6"],
    },
  };

  const current = datasets[viewMode.value] ?? datasets.all;

  return {
    labels: current.labels,
    datasets: [
      {
        data: current.data,
        backgroundColor: current.colors,
        hoverOffset: 10,
      },
    ],
  };
});

const chartOptions = {
  responsive: true,
  maintainAspectRatio: false,
};
</script>
