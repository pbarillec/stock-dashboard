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

  return { assets, fetchAssets };
});
