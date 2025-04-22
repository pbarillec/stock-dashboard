<template>
  <div class="min-h-screen bg-gray-100 text-gray-900 font-sans p-6">
    <h1 class="text-4xl font-bold text-center mb-6">📊 Stock Dashboard</h1>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-6 max-w-5xl mx-auto mb-8">
      <WidgetPortfolio />
      <WidgetPie />
      <WidgetTransactions />
      <WidgetChart />
      <WidgetAssetList />
      <AddWidget
        title="➕ Ajouter une transaction"
        :component="AddTransaction"
      />
      <AddWidget title="➕ Ajouter un actif" :component="AddAsset" />
      <!-- 👈 nouveau widget -->
    </div>
  </div>
</template>

<script setup lang="ts">
import WidgetPortfolio from "./components/widgets/WidgetPortfolio.vue";
import WidgetPie from "./components/widgets/WidgetPie.vue";
import WidgetTransactions from "./components/widgets/WidgetTransactions.vue";
import WidgetChart from "./components/widgets/WidgetChart.vue";
import AddWidget from "./components/widgets/AddWidget.vue";
import AddTransaction from "./components/AddTransaction.vue";
import AddAsset from "./components/AddAsset.vue";
import WidgetAssetList from "./components/widgets/WidgetAssetList.vue";
</script>
