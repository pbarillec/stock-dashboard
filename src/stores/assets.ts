import { ref, computed } from "vue";
import { defineStore } from "pinia";
import { Asset } from "../models/Asset";
import { useDashboardStore } from "./dashboard";
import {
  fetchAssets as fetchAssetsApi,
  addAsset as addAssetApi,
  deleteAsset as deleteAssetApi,
  getCryptoPrice,
  getStockPrice,
} from "@/api";

export const useAssetStore = defineStore("assets", () => {
  const assets = ref<Asset[]>([]);
  const prices = ref<Record<number, number>>({});

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

  async function fetchAllPrices() {
    for (const asset of assets.value) {
      try {
        let price = 0;
        if (asset.category === "crypto") {
          if (asset.api_id) {
            price = await getCryptoPrice(asset.api_id);
          } else {
            console.error(
              `L'ID API est manquant ou invalide pour l'actif ${asset.symbol}`
            );
          }
        } else if (asset.category === "stock") {
          if (asset.api_id) {
            price = await getStockPrice(asset.api_id);
          } else {
            console.error(
              `L'ID API est manquant ou invalide pour l'actif ${asset.symbol}`
            );
          }
        }
        prices.value[asset.id] = price;
      } catch (error) {
        console.error(
          `Erreur lors du chargement du prix pour ${asset.symbol}:`,
          error
        );
      }
    }
  }

  return {
    assets,
    prices,
    filteredAssets,
    fetchAssets,
    addAsset,
    deleteAsset,
    fetchAllPrices,
  };
});
