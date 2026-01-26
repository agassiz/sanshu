<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  isEnhancing: boolean
  hasCompleted: boolean
  errorMessage: string
  streamContent: string
  enhancedPrompt: string
  progress: number
  statusText: string
  projectRootPath: string
  blobCount: number | null
  historyCount: number | null
  blobSourceRoot: string
}

const props = defineProps<Props>()

// 结果区：统一渲染流式、成功与错误状态
const containerClass = computed(() => {
  if (props.errorMessage) {
    return 'border-rose-300/80 bg-rose-50/60 dark:border-rose-500/40 dark:bg-rose-900/20'
  }
  if (props.hasCompleted) {
    return 'border-emerald-300/80 bg-emerald-50/60 dark:border-emerald-500/40 dark:bg-emerald-900/20'
  }
  return 'border-slate-200/80 bg-white/70 dark:border-slate-700/40 dark:bg-slate-900/30'
})

const statusIconClass = computed(() => {
  if (props.errorMessage) return 'i-carbon-warning-alt text-rose-500'
  if (props.hasCompleted) return 'i-carbon-checkmark-filled text-emerald-500'
  return 'i-carbon-magic-wand text-slate-400'
})

const showSkeleton = computed(() => props.isEnhancing && !props.streamContent)

const blobCountText = computed(() => {
  if (props.blobCount === null) return '未返回'
  return `已加载 ${props.blobCount} 个代码块`
})

const historyCountText = computed(() => {
  if (props.historyCount === null) return '未返回'
  return `已加载 ${props.historyCount} 条记录`
})

const showSourceRoot = computed(() => {
  return !!props.blobSourceRoot
})

const sourceMismatch = computed(() => {
  return !!props.blobSourceRoot && !!props.projectRootPath && props.blobSourceRoot !== props.projectRootPath
})
</script>

<template>
  <div class="rounded-2xl border p-4 shadow-sm transition-colors" :class="containerClass">
    <div class="mb-3 flex items-center justify-between text-xs">
      <div class="flex items-center gap-2 text-slate-600 dark:text-slate-300">
        <div class="w-4 h-4" :class="statusIconClass" />
        <span>{{ statusText }}</span>
      </div>
      <span v-if="isEnhancing" class="text-slate-500 dark:text-slate-400">{{ progress }}%</span>
    </div>

    <!-- 诊断信息：项目路径与上下文统计 -->
    <div class="mb-3 space-y-1 text-xs text-slate-600 dark:text-slate-300">
      <div class="flex items-start gap-2">
        <div class="i-carbon-folder h-3.5 w-3.5 text-slate-400" />
        <span class="text-slate-500 dark:text-slate-400">项目：</span>
        <span class="break-all text-slate-700 dark:text-slate-200">
          {{ projectRootPath || '未提供项目路径' }}
        </span>
      </div>
      <div class="flex items-center gap-2">
        <div class="i-carbon-package h-3.5 w-3.5 text-slate-400" />
        <span class="text-slate-500 dark:text-slate-400">代码上下文：</span>
        <span class="text-slate-700 dark:text-slate-200">{{ blobCountText }}</span>
      </div>
      <div class="flex items-center gap-2">
        <div class="i-carbon-chat h-3.5 w-3.5 text-slate-400" />
        <span class="text-slate-500 dark:text-slate-400">对话历史：</span>
        <span class="text-slate-700 dark:text-slate-200">{{ historyCountText }}</span>
      </div>
      <div v-if="showSourceRoot" class="flex items-start gap-2 text-[11px] text-amber-600 dark:text-amber-300">
        <div class="i-carbon-information h-3.5 w-3.5" />
        <span class="text-slate-500 dark:text-slate-400">
          索引来源{{ sourceMismatch ? '（与项目路径不一致）' : '' }}：
        </span>
        <span class="break-all">{{ blobSourceRoot }}</span>
      </div>
    </div>

    <n-progress
      v-if="isEnhancing"
      type="line"
      :percentage="progress"
      :height="6"
      :border-radius="3"
      :show-indicator="false"
      status="info"
    />

    <div class="mt-3">
      <div v-if="errorMessage" class="text-sm text-rose-600 dark:text-rose-200">
        ❌ {{ errorMessage }}
      </div>

      <div v-else-if="hasCompleted && enhancedPrompt" class="whitespace-pre-wrap text-sm text-emerald-700 dark:text-emerald-200">
        {{ enhancedPrompt }}
      </div>

      <div v-else-if="streamContent" class="whitespace-pre-wrap text-sm text-slate-700 dark:text-slate-200">
        {{ streamContent }}
        <span v-if="isEnhancing" class="ml-1 inline-block h-4 w-2 animate-pulse rounded-sm bg-slate-400" />
      </div>

      <div v-else-if="showSkeleton" class="space-y-2">
        <n-skeleton height="14px" width="80%" class="animate-pulse" />
        <n-skeleton height="14px" width="92%" class="animate-pulse" />
        <n-skeleton height="14px" width="88%" class="animate-pulse" />
      </div>

      <div v-else class="flex items-center gap-2 text-xs text-slate-500 dark:text-slate-400">
        <n-spin size="small" />
        正在准备增强...
      </div>
    </div>
  </div>
</template>
