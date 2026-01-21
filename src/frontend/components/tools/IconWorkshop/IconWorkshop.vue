<script setup lang="ts">
/**
 * 图标工坊 - 主组件
 * 提供图标搜索、预览、复制和保存功能
 */
import { invoke } from '@tauri-apps/api/core'
import { useMessage } from 'naive-ui'
import { computed, onMounted, ref, watch } from 'vue'
import { useIconSearch } from '../../../composables/useIconSearch'
import type { IconItem, IconSaveRequest } from '../../../types/icon'
import { DEFAULT_FILTER_OPTIONS } from '../../../types/icon'
import IconCard from './IconCard.vue'
import IconCardSkeleton from './IconCardSkeleton.vue'
import IconSaveModal from './IconSaveModal.vue'

interface Props {
  active?: boolean
  // 弹窗模式相关参数
  mode?: 'normal' | 'popup'
  initialQuery?: string
  initialStyle?: string
  initialSavePath?: string
  projectRoot?: string
  // 外部保存模式（由父组件接管保存流程）
  externalSave?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  active: false,
  mode: 'normal',
  initialQuery: '',
  initialStyle: 'all',
  initialSavePath: '',
  projectRoot: '',
  externalSave: false,
})

const emit = defineEmits<{
  // 外部保存请求（由父组件处理）
  save: [request: IconSaveRequest]
  // 选中图标变化通知（用于弹窗模式的编辑器）
  'selection-change': [icons: IconItem[]]
  // 双击图标打开编辑器
  'icon-dblclick': [icon: IconItem]
  // 右键图标打开上下文菜单
  'icon-contextmenu': [icon: IconItem, event: MouseEvent]
}>()

// 消息提示
const message = useMessage()

// 图标搜索 Hook
const {
  loading,
  error,
  searchParams,
  icons,
  total,
  hasMore,
  currentPage,
  selectedIds,
  selectedIcons,
  selectedCount,
  hasSelection,
  isAllSelected,
  showFilters,
  showSaveModal,
  config,
  search,
  loadMore,
  toggleSelect,
  toggleSelectAll,
  clearSelection,
  copyToClipboard,
  saveIcons,
  loadConfig,
} = useIconSearch()

// 本地状态
const searchInput = ref('')
const showCacheModal = ref(false)

// 计算属性
const hasResults = computed(() => icons.value.length > 0)
const isEmpty = computed(() => !loading.value && searchInput.value && !hasResults.value)
const showEmptyState = computed(() => !loading.value && !searchInput.value && !hasResults.value)

// 通知父组件选中图标变化
watch(selectedIcons, (value) => {
  emit('selection-change', value)
}, { immediate: true })

// 默认保存路径
const defaultSavePath = computed(() => {
  // 如果有初始保存路径（来自 CLI 参数），优先使用
  if (props.initialSavePath) {
    return props.initialSavePath
  }
  return config.value?.defaultSavePath || 'assets/icons'
})

// 执行搜索
async function handleSearch() {
  if (!searchInput.value.trim()) {
    message.warning('请输入搜索关键词')
    return
  }
  searchParams.query = searchInput.value.trim()
  await search()
}

// 回车搜索
function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter') {
    handleSearch()
  }
}

// 复制图标
async function handleCopy(icon: IconItem) {
  const success = await copyToClipboard(icon)
  if (success) {
    message.success(`已复制 ${icon.name} 到剪贴板`)
  }
  else {
    message.error('复制失败')
  }
}

// 双击图标 - 转发给父组件
function handleIconDblClick(icon: IconItem) {
  emit('icon-dblclick', icon)
}

// 右键图标 - 转发给父组件
function handleIconContextMenu(icon: IconItem, event: MouseEvent) {
  emit('icon-contextmenu', icon, event)
}

// 打开保存模态框
function openSaveModal() {
  if (!hasSelection.value) {
    message.warning('请先选择要保存的图标')
    return
  }
  showSaveModal.value = true
}

// 保存选中的图标
async function handleSave(request: IconSaveRequest) {
  // 外部保存模式：由父组件负责保存与后续流程
  if (props.externalSave) {
    showSaveModal.value = false
    emit('save', request)
    return
  }

  const result = await saveIcons(request)
  if (result) {
    message.success(`成功保存 ${result.successCount} 个图标`)
    showSaveModal.value = false
    clearSelection()

    // 如果是弹窗模式，保存成功后退出应用
    if (props.mode === 'popup') {
      try {
        // 构建响应数据
        const response = {
          saved_count: result.successCount,
          save_path: result.savePath,
          saved_names: result.items.filter((i: any) => i.success).map((i: any) => i.name),
          cancelled: false,
        }
        
        // 发送响应并退出
        await invoke('send_mcp_response', { response })
        await invoke('exit_app')
      } catch (e) {
        console.error('Failed to send response or exit:', e)
      }
    }
  }
}

// 加载更多
async function handleLoadMore() {
  if (loading.value || !hasMore.value)
    return
  await loadMore()
}

// 组件挂载时加载配置
onMounted(async () => {
  await loadConfig()

  // 如果是弹窗模式，初始化参数并自动搜索
  if (props.mode === 'popup') {
    if (props.initialQuery) {
      searchInput.value = props.initialQuery
      // 这里的 searchParams 是 reactive 对象，直接赋值即可
      searchParams.query = props.initialQuery
      
      if (props.initialStyle && props.initialStyle !== 'all') {
        // 简单的类型检查，确保它是合法的样式值
        if (['line', 'fill', 'flat', 'all'].includes(props.initialStyle)) {
          searchParams.style = props.initialStyle as 'line' | 'fill' | 'flat' | 'all'
        }
      }
      
      // 自动执行搜索
      await search()
    }
  }
})
</script>

<template>
  <div class="h-full flex flex-col gap-4 bg-white dark:bg-[#121214] overflow-hidden">
    <!-- 搜索区域 -->
    <div class="flex flex-col gap-3 px-1 pt-1">
      <!-- 搜索栏 -->
      <div class="flex items-center gap-2">
        <div class="flex-1 relative group">
          <n-input
            v-model:value="searchInput"
            placeholder="输入关键词搜索图标..."
            size="large"
            clearable
            class="!rounded-xl shadow-sm group-hover:shadow-md transition-shadow"
            @keydown="handleKeydown"
          >
            <template #prefix>
              <div class="i-carbon-search text-lg text-slate-400" />
            </template>
          </n-input>
        </div>
        
        <n-button
          type="primary"
          size="large"
          class="!rounded-xl !px-6 shadow-indigo-500/20 shadow-lg"
          :loading="loading"
          @click="handleSearch"
        >
          <template #icon>
            <div class="i-carbon-search" />
          </template>
          搜索
        </n-button>
        
        <n-button
          quaternary
          size="large"
          class="!rounded-xl"
          :type="showFilters ? 'primary' : 'default'"
          @click="showFilters = !showFilters"
        >
          <template #icon>
            <div class="i-carbon-filter" />
          </template>
          筛选
        </n-button>
      </div>

      <!-- 筛选面板 -->
      <transition
        enter-active-class="transition-all duration-300 ease-out"
        enter-from-class="opacity-0 -translate-y-2 scale-95"
        enter-to-class="opacity-100 translate-y-0 scale-100"
        leave-active-class="transition-all duration-200 ease-in"
        leave-from-class="opacity-100 translate-y-0 scale-100"
        leave-to-class="opacity-0 -translate-y-2 scale-95"
      >
        <div 
          v-if="showFilters" 
          class="p-4 rounded-xl bg-slate-50 dark:bg-white/5 border border-slate-100 dark:border-white/5 flex flex-wrap gap-6 shadow-inner"
        >
          <div class="flex items-center gap-3">
            <span class="text-xs font-semibold text-slate-400 uppercase tracking-wider">风格</span>
            <n-radio-group v-model:value="searchParams.style" size="small">
              <n-radio-button
                v-for="opt in DEFAULT_FILTER_OPTIONS.styles"
                :key="opt.value"
                :value="opt.value"
                class="!rounded-md"
              >
                {{ opt.label }}
              </n-radio-button>
            </n-radio-group>
          </div>
          
          <div class="flex items-center gap-3">
            <span class="text-xs font-semibold text-slate-400 uppercase tracking-wider">填充</span>
            <n-radio-group v-model:value="searchParams.fills" size="small">
              <n-radio-button
                v-for="opt in DEFAULT_FILTER_OPTIONS.fills"
                :key="opt.value"
                :value="opt.value"
              >
                {{ opt.label }}
              </n-radio-button>
            </n-radio-group>
          </div>
          
          <div class="flex items-center gap-3">
            <span class="text-xs font-semibold text-slate-400 uppercase tracking-wider">排序</span>
            <n-radio-group v-model:value="searchParams.sortType" size="small">
              <n-radio-button
                v-for="opt in DEFAULT_FILTER_OPTIONS.sortTypes"
                :key="opt.value"
                :value="opt.value"
              >
                {{ opt.label }}
              </n-radio-button>
            </n-radio-group>
          </div>
        </div>
      </transition>
    </div>

    <!-- 操作栏 -->
    <div v-if="hasResults || hasSelection" class="flex justify-between items-center px-2 py-2 bg-slate-50 dark:bg-white/5 rounded-lg mx-1">
      <div class="flex items-center gap-4">
        <n-checkbox
          :checked="isAllSelected"
          :indeterminate="hasSelection && !isAllSelected"
          class="ml-2"
          @update:checked="toggleSelectAll"
        >
          全选
        </n-checkbox>
        <div class="h-4 w-px bg-slate-200 dark:bg-white/10" />
        <span class="text-xs text-slate-500 dark:text-slate-400">
          共 {{ total }} 个结果 · 第 {{ currentPage }} 页
        </span>
      </div>
      
      <div class="flex items-center gap-3">
        <transition name="fade">
          <n-button
            v-if="hasSelection"
            size="small"
            quaternary
            type="error"
            @click="clearSelection"
          >
            清空 ({{ selectedCount }})
          </n-button>
        </transition>
        
        <n-button
          type="primary"
          size="small"
          :disabled="!hasSelection"
          class="shadow-sm"
          @click="openSaveModal"
        >
          <template #icon>
            <div class="i-carbon-download" />
          </template>
          保存选中 ({{ selectedCount }})
        </n-button>
      </div>
    </div>

    <!-- 滚动容器 -->
    <div class="flex-1 overflow-y-auto min-h-0 pr-2 custom-scrollbar relative">
      <!-- 骨架屏 Loading -->
      <div v-if="loading && !hasMore" class="grid grid-cols-4 sm:grid-cols-5 md:grid-cols-6 lg:grid-cols-8 gap-3">
        <IconCardSkeleton v-for="i in 32" :key="i" />
      </div>

      <!-- 结果网格 -->
      <div v-else-if="hasResults" class="grid grid-cols-4 sm:grid-cols-5 md:grid-cols-6 lg:grid-cols-8 gap-3 content-start">
        <IconCard
          v-for="icon in icons"
          :key="icon.id"
          :icon="icon"
          :selected="selectedIds.has(icon.id)"
          @toggle="toggleSelect(icon.id)"
          @copy="handleCopy(icon)"
          @dblclick="handleIconDblClick(icon)"
          @contextmenu="handleIconContextMenu(icon, $event)"
        />
        
        <!-- 加载更多骨架 -->
        <template v-if="loading && hasMore">
           <IconCardSkeleton v-for="i in 16" :key="`more-${i}`" />
        </template>
      </div>

      <!-- 空状态 - 无搜索结果 -->
      <div v-else-if="isEmpty" class="h-full flex flex-col items-center justify-center text-slate-400">
        <div class="w-24 h-24 rounded-full bg-slate-50 dark:bg-white/5 flex items-center justify-center mb-4">
          <div class="i-carbon-search-locate text-4xl opacity-50" />
        </div>
        <p class="text-sm">未找到相关图标，请尝试其他关键词</p>
      </div>

      <!-- 空状态 - 初始 -->
      <div v-else-if="showEmptyState" class="h-full flex flex-col items-center justify-center text-slate-300 dark:text-slate-600">
        <div class="i-carbon-image text-8xl opacity-10 mb-6" />
        <p class="text-lg font-medium opacity-80 mb-2">搜索 Iconfont 图标库</p>
        <p class="text-sm opacity-50">输入关键词开始探索无限创意</p>
      </div>
      
      <!-- 加载更多按钮 (非自动加载场景) -->
      <div v-if="hasMore && !loading" class="flex justify-center py-6">
        <n-button secondary size="large" @click="handleLoadMore">
          加载更多
        </n-button>
      </div>
    </div>

    <!-- 错误提示 -->
    <div v-if="error" class="fixed bottom-4 left-1/2 transform -translate-x-1/2 z-50">
      <n-alert type="error" closable title="出错了" class="shadow-xl">
        {{ error }}
      </n-alert>
    </div>

    <!-- 保存模态框 -->
    <IconSaveModal
      v-model:show="showSaveModal"
      :icons="selectedIcons"
      :default-path="defaultSavePath"
      @save="handleSave"
    />
  </div>
</template>

<style scoped>
/* 自定义滚动条 */
.custom-scrollbar::-webkit-scrollbar {
  width: 6px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background-color: rgba(156, 163, 175, 0.2);
  border-radius: 3px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background-color: rgba(156, 163, 175, 0.4);
}

/* 过渡动画 */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
