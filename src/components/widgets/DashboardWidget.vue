<template>
  <div class="bg-white p-4 rounded-lg shadow-md border border-gray-200 w-full">
    <div class="flex justify-between items-center mb-2">
      <h3 class="text-lg font-semibold">{{ title }}</h3>
      <button
        v-if="onRemove"
        @click="onRemove"
        class="text-gray-400 hover:text-red-500 transition"
      >
        ✕
      </button>
    </div>
    <slot />
  </div>
</template>

<script setup lang="ts">
defineProps<{
  title: string;
  onRemove?: () => void;
}>();
</script>
