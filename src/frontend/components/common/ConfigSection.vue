<script setup lang="ts">
defineProps<{
  title?: string
  description?: string
  noCard?: boolean
}>()
</script>

<template>
  <div class="config-section">
    <div v-if="title || description" class="mb-4">
      <h3 v-if="title" class="text-base font-medium text-gray-900 dark:text-gray-100 flex items-center gap-2">
        <slot name="icon" />
        {{ title }}
      </h3>
      <p v-if="description" class="text-xs text-gray-500 dark:text-gray-400 mt-1">
        {{ description }}
      </p>
    </div>

    <n-card v-if="!noCard" size="small" :bordered="true" class="config-card">
      <slot />
    </n-card>
    <div v-else>
      <slot />
    </div>
  </div>
</template>

<style scoped>
.config-card {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.config-card:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
}

:deep(.n-card-header) {
  padding: 12px 16px;
}

:deep(.n-card__content) {
  padding: 16px;
}
</style>
