<template>
  <div class="min-h-screen bg-gray-100 text-gray-900 font-sans p-6">
    <h1
      class="text-4xl font-bold text-center mb-6 flex items-center justify-center"
    >
      Stock Dashboard
    </h1>
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6 max-w-4xl mx-auto">
      <Transactions />
      <Assets />
    </div>
  </div>
</template>
<script setup>
import Transactions from "./components/Transactions.vue";
import Assets from "./components/Assets.vue";
</script>
