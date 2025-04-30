import { ref, computed } from "vue";
import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { Asset } from "../models/Asset";
import { useDashboardStore } from "./dashboard";
import {
  fetchAssets as fetchAssetsApi,
  addAsset as addAssetApi,
  deleteAsset as deleteAssetApi,
} from "@/api";

export const useAssetStore = defineStore("assets", () => {
  const assets = ref<Asset[]>([]);

  async function fetchAssets() {
    try {
      assets.value = await fetchAssetsApi();
    } catch (error) {
      console.error("Erreur lors de la récupération des actifs :", error);
    }
  }

  const filteredAssets = computed(() => {
    const dashboardStore = useDashboardStore();

    return assets.value.filter((a) =>
      dashboardStore.filter === "all"
        ? true
        : a.category === dashboardStore.filter
    );
  });

  async function addAsset(newAsset: Omit<Asset, "id">) {
    try {
      const inserted = await addAssetApi(newAsset);
      assets.value.push(inserted);
    } catch (error) {
      console.error("Erreur lors de l'ajout de l'actif :", error);
    }
  }

  async function deleteAsset(assetId: number) {
    try {
      await deleteAssetApi(assetId); // Appel à l'API pour supprimer l'actif
      assets.value = assets.value.filter((a) => a.id !== assetId);
    } catch (error) {
      console.error("Erreur lors de la suppression de l'actif :", error);
    }
  }

  return {
    assets,
    filteredAssets,
    fetchAssets,
    addAsset,
    deleteAsset,
  };
});
