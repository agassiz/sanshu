<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { NButton } from 'naive-ui'
import IconWorkshop from './IconWorkshop.vue'

interface Props {
  initialQuery?: string
  initialStyle?: string
  initialSavePath?: string
  projectRoot?: string
}

const props = defineProps<Props>()

async function handleCancel() {
  try {
    const response = {
      saved_count: 0,
      save_path: '',
      saved_names: [],
      cancelled: true,
    }
    await invoke('send_mcp_response', { response })
    await invoke('exit_app')
  }
  catch (error) {
    console.error('Failed to cancel icon popup:', error)
    await invoke('exit_app')
  }
}
</script>

<template>
  <div class="h-screen flex flex-col bg-surface">
    <!-- 顶部导航栏 -->
    <div class="flex-shrink-0 h-14 border-b border-border flex items-center justify-between px-4 bg-surface-variant">
      <div class="flex items-center gap-2">
        <div class="i-carbon-image text-xl text-primary" />
        <span class="font-medium">图标工坊</span>
      </div>
      
      <NButton secondary type="error" size="small" @click="handleCancel">
        取消 / 关闭
      </NButton>
    </div>

    <!-- 主内容区 -->
    <div class="flex-1 overflow-hidden p-4">
      <IconWorkshop
        mode="popup"
        :active="true"
        :initial-query="props.initialQuery"
        :initial-style="props.initialStyle"
        :initial-save-path="props.initialSavePath"
        :project-root="props.projectRoot"
      />
    </div>
  </div>
</template>
