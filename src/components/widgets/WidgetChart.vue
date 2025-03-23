<template>
  <DashboardWidget title="📊 Évolution du portefeuille">
    <div class="relative h-64">
      <Line :data="chartData" :options="chartOptions" />
    </div>
  </DashboardWidget>
</template>

<script setup lang="ts">
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

ChartJS.register(
  Title,
  Tooltip,
  Legend,
  LineElement,
  CategoryScale,
  LinearScale,
  PointElement
);

// 🔧 Données mockées pour affichage
const chartData = {
  labels: [
    "2025-03-01",
    "2025-03-02",
    "2025-03-03",
    "2025-03-04",
    "2025-03-05",
  ],
  datasets: [
    {
      label: "Valeur portefeuille (€)",
      data: [1500, 1800, 1600, 2000, 2200],
      borderColor: "#3B82F6",
      backgroundColor: "rgba(59, 130, 246, 0.2)",
      tension: 0.3,
      fill: true,
    },
  ],
};

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
