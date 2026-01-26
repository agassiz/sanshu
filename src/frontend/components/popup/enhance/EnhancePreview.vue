<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  coreText: string
  contextText: string
}

const props = defineProps<Props>()

// 预览区：展示核心提示词与上下文补充
const coreCount = computed(() => props.coreText?.length ?? 0)
const contextCount = computed(() => props.contextText?.length ?? 0)
</script>

<template>
  <div class="grid grid-cols-1 gap-3 md:grid-cols-2">
    <n-card
      size="small"
      class="border border-rose-200/80 bg-rose-50/70 shadow-sm dark:border-rose-700/40 dark:bg-rose-900/20"
    >
      <div class="mb-2 flex items-center justify-between text-xs text-rose-700 dark:text-rose-200">
        <span class="flex items-center gap-2">
          <div class="i-carbon-document w-3.5 h-3.5" />
          核心提示词
        </span>
        <span>{{ coreCount }} 字符</span>
      </div>
      <div class="max-h-48 overflow-y-auto whitespace-pre-wrap text-sm text-rose-900 dark:text-rose-100">
        {{ coreText || '暂无内容' }}
      </div>
    </n-card>

    <n-card
      size="small"
      class="border border-slate-200/80 bg-slate-50/70 shadow-sm dark:border-slate-700/40 dark:bg-slate-900/20"
    >
      <div class="mb-2 flex items-center justify-between text-xs text-slate-600 dark:text-slate-200">
        <span class="flex items-center gap-2">
          <div class="i-carbon-settings-adjust w-3.5 h-3.5" />
          上下文补充
        </span>
        <span>{{ contextCount }} 字符</span>
      </div>
      <div class="max-h-48 overflow-y-auto whitespace-pre-wrap text-sm text-slate-800 dark:text-slate-200">
        {{ contextText || '未添加上下文补充' }}
      </div>
    </n-card>
  </div>
</template>
