<template>
  <DashboardWidget title="📜 Dernière(s) transaction(s)">
    <div class="flex justify-end gap-2 mb-2 text-sm">
      <button
        v-for="option in options"
        :key="option"
        @click="limit = option"
        :class="[
          'px-2 py-1 rounded border',
          limit === option
            ? 'bg-blue-500 text-white border-blue-500'
            : 'bg-white text-gray-700 border-gray-300 hover:bg-gray-100',
        ]"
      >
        {{ option }}
      </button>
    </div>
    <ul class="space-y-2">
      <li
        v-for="transaction in limitedTransactions"
        :key="transaction.id"
        class="border-b pb-1 text-sm cursor-pointer hover:bg-gray-50 p-1 rounded"
        @click="showDetails(transaction)"
      >
        <div class="flex justify-between items-start">
          <div>
            <strong>{{ getAssetName(transaction.asset) }}</strong> -
            {{ transaction.quantity }} @ {{ transaction.price }}€
          </div>
          <button
            @click.stop="transactionStore.deleteTransaction(transaction.id)"
            class="text-red-600 hover:scale-110 transition"
            title="Supprimer"
          >
            ❌
          </button>
        </div>
        <div class="text-gray-500 text-xs mt-1">{{ transaction.date }}</div>
      </li>
    </ul>

    <!-- Modal détails transaction -->
    <div
      v-if="selected"
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
    >
      <div class="bg-white p-6 rounded shadow-md max-w-sm w-full relative">
        <button
          @click="selected = null"
          class="absolute top-2 right-2 text-gray-500 hover:text-black"
        >
          ✖
        </button>

        <h3 class="text-lg font-semibold mb-4">
          {{ getAssetName(selected.asset) }}
        </h3>

        <p class="text-sm text-gray-700 mb-2">
          <strong>Symbole :</strong> {{ selected.asset }}<br />
          <strong>Quantité :</strong> {{ selected.quantity }}<br />
          <strong>Prix unitaire :</strong> {{ selected.price }} €<br />
          <strong>Date d'achat :</strong> {{ selected.date }}<br />
          <strong>Montant total :</strong>
          {{ (selected.quantity * selected.price).toFixed(2) }} €
        </p>
      </div>
    </div>
  </DashboardWidget>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { useTransactionStore } from "../../stores/transactions";
import { useAssetStore } from "../../stores/assets";
import DashboardWidget from "./DashboardWidget.vue";

const transactionStore = useTransactionStore();
const assetStore = useAssetStore();
const options: (number | "All")[] = [5, 10, "All"];
const limit = ref<number | "All">(5);

const selected = ref<any | null>(null);

const limitedTransactions = computed(() => {
  const sorted = [...transactionStore.filteredTransactions].sort(
    (a, b) => new Date(b.date).getTime() - new Date(a.date).getTime()
  );
  return limit.value === "All" ? sorted : sorted.slice(0, limit.value);
});

function getAssetName(symbol: string): string {
  const asset = assetStore.assets.find((a) => a.symbol === symbol);
  return asset ? asset.name : symbol;
}

function showDetails(transaction: any) {
  selected.value = transaction;
}
</script>
