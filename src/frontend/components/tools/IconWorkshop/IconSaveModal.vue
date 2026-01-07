<script setup lang="ts">
/**
 * 图标保存模态框组件
 * 配置保存路径和格式，执行保存操作
 */
import type { IconFormat, IconItem, IconSaveRequest } from '../../../types/icon'
import { invoke } from '@tauri-apps/api/core'
import { computed, ref, watch } from 'vue'

// Props
interface Props {
  show: boolean
  icons: IconItem[]
  defaultPath?: string
}
const props = withDefaults(defineProps<Props>(), {
  defaultPath: 'assets/icons',
})

// Emits
const emit = defineEmits<{
  'update:show': [value: boolean]
  'save': [request: IconSaveRequest]
}>()

// 本地状态
const savePath = ref(props.defaultPath)
const format = ref<IconFormat>('svg')
const saving = ref(false)

// 格式选项
const formatOptions = [
  { label: 'SVG 矢量格式', value: 'svg' },
  { label: 'PNG 位图格式', value: 'png' },
  { label: 'SVG + PNG 双格式', value: 'both' },
]

// 监听默认路径变化
watch(() => props.defaultPath, (newPath) => {
  if (newPath) {
    savePath.value = newPath
  }
})

// 计算属性
const dialogVisible = computed({
  get: () => props.show,
  set: (value: boolean) => emit('update:show', value),
})

const iconCount = computed(() => props.icons.length)

const previewIcons = computed(() => props.icons.slice(0, 5))

const hasMoreIcons = computed(() => props.icons.length > 5)

// 选择目录
async function selectDirectory() {
  try {
    const result = await invoke<string | null>('select_icon_save_directory', {
      defaultPath: savePath.value,
    })
    if (result) {
      savePath.value = result
    }
  }
  catch (e) {
    console.error('选择目录失败:', e)
  }
}

// 执行保存
async function handleSave() {
  if (!savePath.value.trim()) {
    return
  }

  saving.value = true
  try {
    emit('save', {
      icons: props.icons,
      savePath: savePath.value,
      format: format.value,
    })
  }
  finally {
    saving.value = false
  }
}

// 取消
function handleCancel() {
  dialogVisible.value = false
}
</script>

<template>
  <n-modal
    v-model:show="dialogVisible"
    preset="card"
    title="保存图标"
    :style="{ width: '500px', maxWidth: '95vw' }"
    :bordered="false"
    size="huge"
    class="save-modal"
  >
    <div class="save-modal-content">
      <!-- 预览区域 -->
      <div class="preview-section">
        <div class="preview-label">
          已选择 {{ iconCount }} 个图标
        </div>
        <div class="preview-icons">
          <div
            v-for="icon in previewIcons"
            :key="icon.id"
            class="preview-icon"
            :title="icon.name"
          >
            <div
              v-if="icon.svgContent"
              class="svg-preview"
              v-html="icon.svgContent"
            />
            <div v-else class="i-carbon-image opacity-30" />
          </div>
          <div v-if="hasMoreIcons" class="preview-more">
            +{{ icons.length - 5 }}
          </div>
        </div>
      </div>

      <!-- 保存路径 -->
      <div class="form-group">
        <label class="form-label">保存路径</label>
        <div class="path-input">
          <n-input
            v-model:value="savePath"
            placeholder="输入保存目录路径"
            size="large"
          >
            <template #prefix>
              <div class="i-carbon-folder opacity-50" />
            </template>
          </n-input>
          <n-button
            type="primary"
            secondary
            size="large"
            @click="selectDirectory"
          >
            <template #icon>
              <div class="i-carbon-folder-open" />
            </template>
            选择
          </n-button>
        </div>
      </div>

      <!-- 保存格式 -->
      <div class="form-group">
        <label class="form-label">保存格式</label>
        <n-radio-group v-model:value="format" size="medium">
          <n-space>
            <n-radio
              v-for="opt in formatOptions"
              :key="opt.value"
              :value="opt.value"
            >
              {{ opt.label }}
            </n-radio>
          </n-space>
        </n-radio-group>
      </div>

      <!-- 提示信息 -->
      <n-alert type="info" :bordered="false" class="mt-4">
        <template #icon>
          <div class="i-carbon-information" />
        </template>
        图标将保存到指定目录，文件名使用图标名称。
      </n-alert>
    </div>

    <!-- 底部操作 -->
    <template #footer>
      <div class="modal-footer">
        <n-button quaternary @click="handleCancel">
          取消
        </n-button>
        <n-button
          type="primary"
          :loading="saving"
          :disabled="!savePath.trim()"
          @click="handleSave"
        >
          <template #icon>
            <div class="i-carbon-download" />
          </template>
          保存 {{ iconCount }} 个图标
        </n-button>
      </div>
    </template>
  </n-modal>
</template>

<style scoped>
.save-modal-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

/* 预览区域 */
.preview-section {
  padding: 16px;
  border-radius: 12px;
  background: var(--color-container, rgba(128, 128, 128, 0.05));
}

:root.dark .preview-section {
  background: rgba(255, 255, 255, 0.03);
}

.preview-label {
  font-size: 13px;
  color: var(--color-on-surface-secondary, #6b7280);
  margin-bottom: 12px;
}

:root.dark .preview-label {
  color: #9ca3af;
}

.preview-icons {
  display: flex;
  gap: 8px;
  align-items: center;
  flex-wrap: wrap;
}

.preview-icon {
  width: 40px;
  height: 40px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--color-container, rgba(255, 255, 255, 0.8));
  border: 1px solid var(--color-border, rgba(128, 128, 128, 0.2));
  overflow: hidden;
}

:root.dark .preview-icon {
  background: rgba(24, 24, 28, 0.9);
  border-color: rgba(255, 255, 255, 0.08);
}

.svg-preview {
  width: 24px;
  height: 24px;
}

.svg-preview :deep(svg) {
  width: 100%;
  height: 100%;
}

.preview-more {
  width: 40px;
  height: 40px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  font-weight: 500;
  color: var(--color-on-surface-secondary, #6b7280);
  background: var(--color-container, rgba(128, 128, 128, 0.1));
}

:root.dark .preview-more {
  color: #9ca3af;
  background: rgba(255, 255, 255, 0.05);
}

/* 表单组 */
.form-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.form-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--color-on-surface, #111827);
}

:root.dark .form-label {
  color: #e5e7eb;
}

.path-input {
  display: flex;
  gap: 8px;
}

.path-input .n-input {
  flex: 1;
}

/* 底部操作 */
.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}
</style>
