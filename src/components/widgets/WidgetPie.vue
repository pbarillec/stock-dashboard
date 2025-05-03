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
  let totalCrypto = 0;
  let totalStock = 0;

  for (const tx of transactionStore.transactions) {
    const realTimePrice = transactionStore.realTimePrices[tx.asset];
    const effectivePrice = realTimePrice ?? tx.price; // fallback si pas de prix live

    const value = tx.quantity * effectivePrice;

    if (tx.category === "crypto") {
      totalCrypto += value;
    } else if (tx.category === "stock") {
      totalStock += value;
    }
  }

  // 🔎 Gérer le filtre global
  let labels: string[] = [];
  let data: number[] = [];
  let bgColors: string[] = [];

  if (dashboardStore.filter === "crypto") {
    labels = ["Crypto"];
    data = [totalCrypto];
    bgColors = ["#10B981"];
  } else if (dashboardStore.filter === "stock") {
    labels = ["Actions"];
    data = [totalStock];
    bgColors = ["#3B82F6"];
  } else {
    labels = ["Crypto", "Actions"];
    data = [totalCrypto, totalStock];
    bgColors = ["#10B981", "#3B82F6"];
  }

  return {
    labels,
    datasets: [
      {
        data,
        backgroundColor: bgColors,
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
