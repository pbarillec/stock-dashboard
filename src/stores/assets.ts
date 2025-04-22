import { ref } from "vue";
import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import type { Asset } from "../models/Asset";

export const useAssetStore = defineStore("assets", () => {
  const assets = ref<Asset[]>([]);

  async function fetchAssets() {
    try {
      assets.value = await invoke<Asset[]>("fetch_assets");
    } catch (error) {
      console.error("Erreur lors de la récupération des actifs :", error);
    }
  }

  async function addAsset(newAsset: Omit<Asset, "id">) {
    try {
      await invoke("add_asset", {
        newAsset, // 👈 correspond à la struct Rust NewAsset
      });
      // Ajouter localement dans le store (id simulé temporairement si besoin)
      assets.value.push({
        id: Date.now(), // id fictif (à remplacer si retour backend un jour)
        ...newAsset,
      });
    } catch (error) {
      console.error("Erreur lors de l'ajout de l'actif :", error);
    }
  }

  async function deleteAsset(assetId: number) {
    try {
      await invoke("delete_asset", {
        assetId: assetId, // Utilisez assetId au lieu de asset_id
      });
      assets.value = assets.value.filter((a) => a.id !== assetId);
    } catch (error) {
      console.error("Erreur lors de la suppression de l'actif :", error);
    }
  }

  return {
    assets,
    fetchAssets,
    addAsset,
    deleteAsset,
  };
});
