<template>
  <DashboardWidget title="📋 Actifs suivis">
    <ul class="text-sm space-y-1">
      <li
        v-for="asset in assetStore.filteredAssets"
        :key="asset.id"
        class="border-b pb-1 flex justify-between items-center"
      >
        <div>
          <strong>{{ asset.symbol }}</strong> - {{ asset.name }} ({{
            asset.category
          }})
        </div>
        <button
          @click="assetStore.deleteAsset(asset.id)"
          class="text-red-600 hover:scale-110 transition"
          title="Supprimer"
        >
          ❌
        </button>
      </li>
    </ul>
  </DashboardWidget>
</template>

<script setup lang="ts">
import { onMounted } from "vue";
import { useAssetStore } from "../../stores/assets";
import DashboardWidget from "./DashboardWidget.vue";

const assetStore = useAssetStore();

onMounted(() => {
  assetStore.fetchAssets();
});
</script>
