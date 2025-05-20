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
import { useFiltersStore } from "../../stores/filters";

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
const filtersStore = useFiltersStore();

const viewMode = computed(() => filtersStore.viewMode);
const performanceMode = computed(() => filtersStore.performanceMode);

const chartData = computed(() => {
  const transactions = transactionStore.filteredTransactions;
  const prices = transactionStore.realTimePrices;

  const groupedByMonth: Record<string, { invested: number; value: number }> =
    {};

  for (const tx of transactions) {
    const month = tx.date.slice(0, 7); // YYYY-MM
    const price = prices[tx.asset] ?? tx.price;

    if (!groupedByMonth[month]) {
      groupedByMonth[month] = { invested: 0, value: 0 };
    }

    groupedByMonth[month].invested += tx.quantity * tx.price;
    groupedByMonth[month].value += tx.quantity * price;
  }

  const sortedMonths = Object.keys(groupedByMonth).sort();
  const dataPoints: number[] = [];
  let cumulativeInvested = 0;
  let cumulativeValue = 0;

  for (const month of sortedMonths) {
    cumulativeInvested += groupedByMonth[month].invested;
    cumulativeValue += groupedByMonth[month].value;

    const value =
      performanceMode.value === "net"
        ? cumulativeValue - cumulativeInvested
        : cumulativeValue;

    dataPoints.push(Number(value.toFixed(2)));
  }

  // Ajout du mois actuel s’il n’est pas présent
  const today = new Date().toISOString().slice(0, 7);
  if (!sortedMonths.includes(today)) {
    const totalCurrentValue = transactions.reduce((acc, tx) => {
      const price = prices[tx.asset] ?? tx.price;
      return acc + tx.quantity * price;
    }, 0);

    const totalInvested = transactions.reduce(
      (acc, tx) => acc + tx.quantity * tx.price,
      0
    );

    sortedMonths.push(today);

    const finalValue =
      performanceMode.value === "net"
        ? totalCurrentValue - totalInvested
        : totalCurrentValue;

    dataPoints.push(Number(finalValue.toFixed(2)));
  }

  return {
    labels: sortedMonths,
    datasets: [
      {
        label:
          viewMode.value === "all"
            ? performanceMode.value === "net"
              ? "Performance cumulée (€)"
              : "Valeur cumulée (€)"
            : `${performanceMode.value === "net" ? "Performance" : "Valeur"} ${
                viewMode.value === "crypto" ? "Crypto" : "Actions"
              }`,
        data: dataPoints,
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
