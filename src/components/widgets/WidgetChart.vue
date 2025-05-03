<template>
  <DashboardWidget title="📊 Évolution du portefeuille">
    <div class="relative h-64">
      <Line :data="chartData" :options="chartOptions" />
    </div>
  </DashboardWidget>
</template>

<script setup lang="ts">
import { computed, onMounted } from "vue";
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

// ✅ On charge TOUT en même temps
onMounted(async () => {
  await transactionStore.fetchTransactions();
  await assetStore.fetchAssets();
  await transactionStore.fetchAllPrices(assetStore.assets.value);
});

// 🔥 Met à jour le graphique avec les prix temps réel
const chartData = computed(() => {
  const transactions = transactionStore.filteredTransactions;
  const realTimePrices = transactionStore.realTimePrices;

  // Grouper par mois/année → ex: "2025-03"
  const groupedByMonth: Record<string, number> = {};

  for (const tx of transactions) {
    const month = tx.date.slice(0, 7); // YYYY-MM

    // On utilise le prix temps réel si dispo, sinon fallback sur le prix d'achat
    const realTimePrice = realTimePrices[tx.asset] ?? tx.price;
    const totalValue = tx.quantity * realTimePrice;

    if (!groupedByMonth[month]) groupedByMonth[month] = 0;
    groupedByMonth[month] += totalValue;
  }

  const sortedMonths = Object.keys(groupedByMonth).sort();

  // Construire les cumuls
  const cumulativeData: number[] = [];
  let total = 0;

  for (const month of sortedMonths) {
    total += groupedByMonth[month];
    cumulativeData.push(total);
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
