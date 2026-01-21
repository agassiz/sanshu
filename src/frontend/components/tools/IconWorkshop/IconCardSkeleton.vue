<script setup lang="ts">
/**
 * 图标卡片骨架屏
 * 使用 Tailwind CSS 实现 Loading 效果
 */
</script>

<template>
  <div class="icon-card aspect-square p-3 rounded-xl bg-slate-50 dark:bg-[#18181b] border border-slate-100 dark:border-white/5 flex flex-col items-center justify-center gap-2">
    <!-- 图标占位 -->
    <div class="icon-preview skeleton-icon w-10 h-10 rounded-lg bg-slate-200 dark:bg-white/10 animate-pulse relative overflow-hidden">
      <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/20 to-transparent -translate-x-full animate-shimmer" />
    </div>
    
    <!-- 名称占位 -->
    <div class="icon-name w-2/3 h-3 rounded bg-slate-200 dark:bg-white/10 animate-pulse" />
  </div>
</template>

<style scoped>
@keyframes shimmer {
  100% {
    transform: translateX(100%);
  }
}

.animate-shimmer {
  animation: shimmer 1.5s infinite;
}
</style>
