<template>
  <div class="bg-white p-4 rounded-lg shadow-md border border-gray-200">
    <h2 class="text-xl font-semibold mb-4 flex items-center">Actifs suivis</h2>
    <ul class="space-y-2">
      <li
        v-for="asset in assetStore.assets"
        :key="asset.id"
        class="border-b pb-2"
      >
        <strong
          >{{ asset.symbol }} - {{ asset.name }} ({{ asset.category }})
        </strong>
      </li>
    </ul>
  </div>
</template>
<script setup lang="ts">
import { onMounted } from "vue";
import { useAssetStore } from "../stores/assets";

const assetStore = useAssetStore();

onMounted(() => {
  assetStore.fetchAssets();
});
</script>
