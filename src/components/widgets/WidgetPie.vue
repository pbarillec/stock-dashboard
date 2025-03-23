<template>
  <DashboardWidget title="🥧 Répartition du portefeuille">
    <div class="relative h-64">
      <Pie :data="chartData" :options="chartOptions" />
    </div>
  </DashboardWidget>
</template>

<script setup lang="ts">
import { computed, onMounted } from "vue";
import { Pie } from "vue-chartjs";
import { Chart as ChartJS, ArcElement, Tooltip, Legend } from "chart.js";
import DashboardWidget from "./DashboardWidget.vue";
import { useTransactionStore } from "../../stores/transactions";

ChartJS.register(ArcElement, Tooltip, Legend);

const store = useTransactionStore();

onMounted(() => {
  store.fetchTransactions();
});

const totalCrypto = computed(() =>
  store.transactions
    .filter((t) => t.category === "crypto")
    .reduce((acc, t) => acc + t.quantity * t.price, 0)
);

const totalStock = computed(() =>
  store.transactions
    .filter((t) => t.category === "stock")
    .reduce((acc, t) => acc + t.quantity * t.price, 0)
);

const chartData = computed(() => ({
  labels: ["Crypto", "Bourse"],
  datasets: [
    {
      data: [totalCrypto.value, totalStock.value],
      backgroundColor: ["#10B981", "#3B82F6"], // vert / bleu
      hoverOffset: 10,
    },
  ],
}));

const chartOptions = {
  responsive: true,
  maintainAspectRatio: false,
};
</script>
