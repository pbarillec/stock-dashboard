<template>
  <DashboardWidget title="📊 Évolution du portefeuille">
    <div class="relative h-64">
      <Line :data="chartData" :options="chartOptions" />
    </div>
  </DashboardWidget>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { Line } from "vue-chartjs";
import {
  Chart as ChartJS,
  Title,
  Tooltip,
  Legend,
  LineElement,
  CategoryScale,
  LinearScale,
  PointElement,
} from "chart.js";
import DashboardWidget from "./DashboardWidget.vue";
import { useTransactionStore } from "../../stores/transactions";
import { useAssetStore } from "../../stores/assets";
import { useDashboardStore } from "../../stores/dashboard";

ChartJS.register(
  Title,
  Tooltip,
  Legend,
  LineElement,
  CategoryScale,
  LinearScale,
  PointElement
);

const transactionStore = useTransactionStore();
const assetStore = useAssetStore();
const dashboardStore = useDashboardStore();

const chartData = computed(() => {
  const transactions = transactionStore.filteredTransactions;
  const prices = transactionStore.realTimePrices;

  // Grouper par mois/année → ex: "2025-03"
  const groupedByMonth: Record<string, number> = {};

  for (const tx of transactions) {
    const month = tx.date.slice(0, 7); // YYYY-MM
    if (!groupedByMonth[month]) groupedByMonth[month] = 0;
    groupedByMonth[month] += tx.quantity * tx.price;
  }

  const sortedMonths = Object.keys(groupedByMonth).sort();

  // Construire les cumuls
  const cumulativeData: number[] = [];
  let total = 0;

  for (const month of sortedMonths) {
    total += groupedByMonth[month];
    cumulativeData.push(total);
  }

  // 🔥 Ajouter la valeur actuelle (aujourd'hui)
  const today = new Date();
  const todayLabel = today.toISOString().slice(0, 7); // YYYY-MM

  const alreadyIncluded = sortedMonths.includes(todayLabel);
  let totalActual = 0;

  for (const tx of transactions) {
    const price = prices[tx.asset] ?? tx.price; // fallback sur prix d'achat
    totalActual += tx.quantity * price;
  }

  if (!alreadyIncluded) {
    sortedMonths.push(todayLabel);
    cumulativeData.push(totalActual);
  } else {
    // Si le mois existe déjà, on peut remplacer la dernière valeur pour avoir la valeur actuelle
    cumulativeData[cumulativeData.length - 1] = totalActual;
  }

  return {
    labels: sortedMonths,
    datasets: [
      {
        label:
          dashboardStore.filter === "all"
            ? "Évolution portefeuille (€)"
            : `Évolution ${
                dashboardStore.filter === "crypto" ? "Crypto" : "Actions"
              }`,
        data: cumulativeData,
        borderColor: "#3B82F6",
        backgroundColor: "rgba(59, 130, 246, 0.2)",
        tension: 0.3,
        fill: true,
      },
    ],
  };
});

const chartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  scales: {
    y: {
      beginAtZero: true,
    },
  },
};
</script>
