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
import { useDashboardStore } from "../../stores/dashboard";

ChartJS.register(ArcElement, Tooltip, Legend);

const transactionStore = useTransactionStore();
const dashboardStore = useDashboardStore();

const chartData = computed(() => {
  const filter = dashboardStore.filter;
  let totalCrypto = 0;
  let totalStock = 0;

  for (const tx of transactionStore.transactions) {
    const price = transactionStore.realTimePrices[tx.asset] ?? tx.price;
    const value = tx.quantity * price;

    if (tx.category === "crypto") totalCrypto += value;
    if (tx.category === "stock") totalStock += value;
  }

  const datasets = {
    all: {
      labels: ["Crypto", "Actions"],
      data: [totalCrypto, totalStock],
      colors: ["#10B981", "#3B82F6"],
    },
    crypto: {
      labels: ["Crypto"],
      data: [totalCrypto],
      colors: ["#10B981"],
    },
    stock: {
      labels: ["Actions"],
      data: [totalStock],
      colors: ["#3B82F6"],
    },
  };

  const current = datasets[filter] ?? datasets.all;

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
