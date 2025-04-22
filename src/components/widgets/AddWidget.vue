<template>
  <DashboardWidget :title="title">
    <component :is="component" />
  </DashboardWidget>
</template>

<script setup lang="ts">
import DashboardWidget from "./DashboardWidget.vue";

defineProps<{
  title: string;
  component: any;
}>();
</script>
t
