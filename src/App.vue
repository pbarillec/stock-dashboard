<template>
  <div class="min-h-screen bg-gray-100 text-gray-900 font-sans p-6">
    <h1 class="text-4xl font-bold text-center mb-6">📊 Stock Dashboard</h1>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-6 max-w-5xl mx-auto">
      <WidgetPortfolio />
      <WidgetPie />
      <WidgetTransactions />
      <WidgetChart />
    </div>
  </div>
</template>

<script setup lang="ts">
import WidgetPortfolio from "./components/widgets/WidgetPortfolio.vue";
import WidgetPie from "./components/widgets/WidgetPie.vue";
import WidgetTransactions from "./components/widgets/WidgetTransactions.vue";
import WidgetChart from "./components/widgets/WidgetChart.vue";
</script>
