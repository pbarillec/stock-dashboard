<template>
  <div
    class="bg-white p-4 rounded-lg shadow-md border border-gray-200 max-w-md mx-auto"
  >
    <h2 class="text-lg font-semibold mb-4">➕ Ajouter une transaction</h2>
    <form @submit.prevent>
      <div class="mb-3">
        <label class="block text-sm font-medium mb-1">Nom de l'actif</label>
        <input v-model="form.asset" type="text" class="input" required />
      </div>

      <div class="mb-3">
        <label class="block text-sm font-medium mb-1">Quantité</label>
        <input
          v-model.number="form.quantity"
          type="number"
          min="0"
          step="any"
          class="input"
          required
        />
      </div>

      <div class="mb-3">
        <label class="block text-sm font-medium mb-1">Prix unitaire (€)</label>
        <input
          v-model.number="form.price"
          type="number"
          min="0"
          step="any"
          class="input"
          required
        />
      </div>

      <div class="mb-3">
        <label class="block text-sm font-medium mb-1">Date</label>
        <input v-model="form.date" type="date" class="input" required />
      </div>

      <div class="mb-4">
        <label class="block text-sm font-medium mb-1">Catégorie</label>
        <select v-model="form.category" class="input" required>
          <option value="crypto">Crypto</option>
          <option value="stock">Bourse</option>
        </select>
      </div>

      <button
        type="submit"
        class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded"
      >
        Ajouter
      </button>
    </form>
  </div>
</template>

<script setup lang="ts">
import { reactive } from "vue";

const form = reactive({
  asset: "",
  quantity: 0,
  price: 0,
  date: "",
  category: "crypto",
});
</script>

<style scoped>
.input {
  @apply w-full border rounded px-3 py-2 text-sm focus:outline-none focus:ring focus:ring-blue-200;
}
</style>
