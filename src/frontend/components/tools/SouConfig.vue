<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { useMessage } from 'naive-ui'
import { onMounted, ref, watch } from 'vue'
import { useAcemcpSync } from '../../composables/useAcemcpSync'
import ProjectIndexManager from '../settings/ProjectIndexManager.vue'
import ConfigSection from '../common/ConfigSection.vue'

// Props & Emits
const props = defineProps<{
  active: boolean
}>()

// Use Message
const message = useMessage()

// Acemcp Sync State
const {
  autoIndexEnabled,
  triggerIndexUpdate,
  fetchProjectStatus,
  setCurrentProject,
  fetchAutoIndexEnabled,
  setAutoIndexEnabled,
  fetchWatchingProjects,
  fetchAllStatus
} = useAcemcpSync()

// Configuration State
const config = ref({
  base_url: '',
  token: '',
  batch_size: 10,
  max_lines_per_blob: 800,
  text_extensions: [] as string[],
  exclude_patterns: [] as string[],
})

const loadingConfig = ref(false)

// Debug State
const debugProjectRoot = ref('')
const debugQuery = ref('')
const debugResult = ref('')
const debugLoading = ref(false)
const indexManagementProjectRoot = ref('')
const indexingInProgress = ref(false)

// Options
const extOptions = ref([
  '.py', '.js', '.ts', '.jsx', '.tsx', '.java', '.go', '.rs', 
  '.cpp', '.c', '.h', '.hpp', '.cs', '.rb', '.php', '.md', 
  '.txt', '.json', '.yaml', '.yml', '.toml', '.xml', '.html', 
  '.css', '.scss', '.sql', '.sh', '.bash'
].map(v => ({ label: v, value: v })))

const excludeOptions = ref([
  '.venv', 'venv', '.env', 'env', 'node_modules', '.next', '.nuxt', 
  '.output', 'out', '.cache', '.turbo', '.vercel', '.netlify', 
  '.swc', '.vite', '.parcel-cache', '.sass-cache', '.eslintcache', 
  '.stylelintcache', 'coverage', '.nyc_output', 'tmp', 'temp', 
  '.tmp', '.temp', '.git', '.svn', '.hg', '__pycache__', 
  '.pytest_cache', '.mypy_cache', '.tox', '.eggs', '*.egg-info', 
  'dist', 'build', '.idea', '.vscode', '.DS_Store', '*.pyc', 
  '*.pyo', '*.pyd', '.Python', 'pip-log.txt', 
  'pip-delete-this-directory.txt', '.coverage', 'htmlcov', 
  '.gradle', 'target', 'bin', 'obj'
].map(v => ({ label: v, value: v })))

// --- Actions ---

async function loadAcemcpConfig() {
  loadingConfig.value = true
  try {
    const res = await invoke('get_acemcp_config') as any
    
    config.value = {
      base_url: res.base_url || '',
      token: res.token || '',
      batch_size: res.batch_size,
      max_lines_per_blob: res.max_lines_per_blob,
      text_extensions: res.text_extensions,
      exclude_patterns: res.exclude_patterns,
    }

    // Ensure options exist
    const extSet = new Set(extOptions.value.map(o => o.value))
    for (const v of config.value.text_extensions) {
      if (!extSet.has(v)) extOptions.value.push({ label: v, value: v })
    }
    const exSet = new Set(excludeOptions.value.map(o => o.value))
    for (const v of config.value.exclude_patterns) {
      if (!exSet.has(v)) excludeOptions.value.push({ label: v, value: v })
    }
  } catch (err) {
    message.error(`加载配置失败: ${err}`)
  } finally {
    loadingConfig.value = false
  }
}

async function saveConfig() {
  try {
    if (!config.value.base_url || !/^https?:\/\//i.test(config.value.base_url)) {
      message.error('URL无效，需以 http(s):// 开头')
      return
    }
    
    await invoke('save_acemcp_config', {
      args: {
        baseUrl: config.value.base_url,
        token: config.value.token,
        batchSize: config.value.batch_size,
        maxLinesPerBlob: config.value.max_lines_per_blob,
        textExtensions: config.value.text_extensions,
        excludePatterns: config.value.exclude_patterns,
      },
    })
    message.success('配置已保存')
  } catch (err) {
    message.error(`保存失败: ${err}`)
  }
}

async function testConnection() {
  const loadingMsg = message.loading('正在测试连接...', { duration: 0 })
  try {
    const result = await invoke('test_acemcp_connection', {
      args: {
        baseUrl: config.value.base_url,
        token: config.value.token,
      },
    }) as { success: boolean; message: string }

    if (result.success) {
      message.success(result.message)
    } else {
      message.error(result.message)
    }
  } catch (err) {
    message.error(`连接测试失败: ${err}`)
  } finally {
    loadingMsg.destroy()
  }
}

// Debug actions
async function runToolDebug() {
  if (!debugProjectRoot.value || !debugQuery.value) {
    message.warning('请填写项目路径和查询语句')
    return
  }
  
  debugLoading.value = true
  debugResult.value = ''
  
  try {
    const result = await invoke('debug_acemcp_search', {
      projectRootPath: debugProjectRoot.value,
      query: debugQuery.value,
    }) as { success: boolean; result?: string; error?: string }

    if (result.success) {
      debugResult.value = result.result || '无返回结果'
      message.success('调试执行成功')
    } else {
      debugResult.value = result.error || '执行出错'
      message.error(result.error || '调试失败')
    }
  } catch (e: any) {
    const msg = e?.message || String(e)
    debugResult.value = `Error: ${msg}`
    message.error(`调试异常: ${msg}`)
  } finally {
    debugLoading.value = false
  }
}

async function viewLogs() {
  try {
    const lines = await invoke('read_acemcp_logs') as string[]
    if (lines.length > 0) {
      await navigator.clipboard.writeText(lines.join('\n'))
      message.success(`已复制 ${lines.length} 行日志`)
    } else {
      message.info('日志为空')
    }
  } catch (e) {
    message.error(`读取日志失败: ${e}`)
  }
}

async function clearCache() {
  try {
    message.loading('正在清除...')
    const res = await invoke('clear_acemcp_cache') as string
    message.success(res)
  } catch (e) {
    message.error(`清除失败: ${e}`)
  }
}

// Index Management
async function toggleAutoIndex() {
  try {
    await setAutoIndexEnabled(!autoIndexEnabled.value)
    message.success(`自动索引已${autoIndexEnabled.value ? '启用' : '禁用'}`)
  } catch (e) {
    message.error(String(e))
  }
}

// Watchers
watch(() => config.value.text_extensions, (list) => {
  const norm = Array.from(new Set((list || []).map(s => {
    const t = s.trim().toLowerCase()
    return t ? (t.startsWith('.') ? t : `.${t}`) : ''
  }).filter(Boolean)))
  
  if (norm.join(',') !== list.join(',')) {
    config.value.text_extensions = norm
  }
}, { deep: true })

// Lifecycle
onMounted(async () => {
  if (props.active) {
    await loadAcemcpConfig()
    await Promise.all([
      fetchAutoIndexEnabled(),
      fetchWatchingProjects()
    ])
  }
})

// Expose save method (though currently button is inside component, parent might want to trigger it)
defineExpose({ saveConfig })
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- Main Tabs -->
    <n-tabs type="line" animated class="flex-1">
      <n-tab-pane name="basic" tab="基础配置">
        <n-scrollbar style="max-height: 60vh" class="pr-4">
          <n-space vertical size="large">
            <ConfigSection title="连接设置" description="配置代码搜索服务的连接信息">
              <n-grid :x-gap="24" :y-gap="24" :cols="1">
                <n-grid-item>
                  <n-form-item label="API端点URL">
                    <n-input v-model:value="config.base_url" placeholder="https://api.example.com" clearable />
                  </n-form-item>
                </n-grid-item>
                <n-grid-item>
                  <n-form-item label="认证令牌">
                    <n-input
                      v-model:value="config.token"
                      type="password"
                      show-password-on="click"
                      placeholder="Enter token"
                      clearable
                    />
                  </n-form-item>
                </n-grid-item>
              </n-grid>
            </ConfigSection>

            <ConfigSection title="性能参数" description="调整处理批量和文件大小限制">
              <n-grid :x-gap="24" :cols="2">
                <n-grid-item>
                  <n-form-item label="批处理大小">
                    <n-input-number v-model:value="config.batch_size" :min="1" :max="100" class="w-full" />
                  </n-form-item>
                </n-grid-item>
                <n-grid-item>
                  <n-form-item label="最大行数/块">
                    <n-input-number v-model:value="config.max_lines_per_blob" :min="100" :max="5000" class="w-full" />
                  </n-form-item>
                </n-grid-item>
              </n-grid>
            </ConfigSection>
            
            <div class="flex justify-end pt-4">
               <n-button type="primary" @click="saveConfig" size="medium">
                 保存配置
               </n-button>
            </div>
          </n-space>
        </n-scrollbar>
      </n-tab-pane>

      <n-tab-pane name="advanced" tab="高级配置">
        <n-scrollbar style="max-height: 60vh" class="pr-4">
          <n-space vertical size="large">
            <ConfigSection title="文件过滤" description="设置需索引的文件类型和排除规则">
              <n-space vertical size="medium">
                <n-form-item label="包含扩展名">
                  <n-select
                    v-model:value="config.text_extensions"
                    :options="extOptions"
                    multiple tag filterable clearable
                    placeholder="输入或选择扩展名 (.py)"
                  />
                  <template #feedback>小写，点开头，自动去重</template>
                </n-form-item>

                <n-form-item label="排除模式">
                  <n-select
                    v-model:value="config.exclude_patterns"
                    :options="excludeOptions"
                    multiple tag filterable clearable
                    placeholder="输入或选择排除模式 (node_modules)"
                  />
                  <template #feedback>支持 glob 通配符</template>
                </n-form-item>
              </n-space>
            </ConfigSection>

            <div class="flex justify-end pt-4">
               <n-button type="primary" @click="saveConfig" size="medium">
                 保存配置
               </n-button>
            </div>
          </n-space>
        </n-scrollbar>
      </n-tab-pane>

      <n-tab-pane name="debug" tab="日志与调试">
        <n-scrollbar style="max-height: 60vh" class="pr-4">
          <n-space vertical size="large">
            <ConfigSection title="工具状态" :no-card="true">
              <n-alert type="info" :bordered="false" class="mb-4">
                <template #icon><div class="i-carbon-terminal" /></template>
                日志路径: <code>~/.sanshu/log/acemcp.log</code>
              </n-alert>
              
              <n-space>
                 <n-button size="small" secondary @click="testConnection">
                   <template #icon><div class="i-carbon-connection-signal" /></template>
                   测试连接
                 </n-button>
                 <n-button size="small" secondary @click="viewLogs">
                   <template #icon><div class="i-carbon-document" /></template>
                   查看日志
                 </n-button>
                 <n-button size="small" secondary @click="clearCache">
                   <template #icon><div class="i-carbon-clean" /></template>
                   清除缓存
                 </n-button>
              </n-space>
            </ConfigSection>

            <ConfigSection title="搜索调试" description="模拟搜索请求以验证配置">
              <n-space vertical size="medium">
                <n-form-item label="项目根路径" :show-feedback="false">
                  <n-input v-model:value="debugProjectRoot" placeholder="/abs/path/to/project" />
                </n-form-item>
                <n-form-item label="查询语句" :show-feedback="false">
                  <n-input v-model:value="debugQuery" type="textarea" :rows="2" placeholder="输入搜索意图..." />
                </n-form-item>
                
                <div>
                   <n-button
                    type="primary"
                    ghost
                    :loading="debugLoading"
                    :disabled="!debugProjectRoot || !debugQuery"
                    @click="runToolDebug"
                  >
                    <template #icon><div class="i-carbon-play" /></template>
                    运行调试
                  </n-button>
                </div>

                <div v-if="debugResult" class="mt-2">
                   <div class="text-xs text-gray-500 mb-1">结果输出:</div>
                   <div class="bg-gray-50 dark:bg-gray-800 p-2 rounded text-xs font-mono whitespace-pre-wrap max-h-60 overflow-y-auto border border-gray-200 dark:border-gray-700">
                     {{ debugResult }}
                   </div>
                </div>
              </n-space>
            </ConfigSection>
          </n-space>
        </n-scrollbar>
      </n-tab-pane>

      <n-tab-pane name="index" tab="索引管理">
        <n-scrollbar style="max-height: 60vh" class="pr-4">
          <n-space vertical size="large">
            <ConfigSection title="全局策略">
               <div class="flex items-center justify-between">
                 <div class="flex items-center gap-3">
                   <div class="p-2 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
                     <div class="i-carbon-automatic w-5 h-5 text-blue-500" />
                   </div>
                   <div>
                     <div class="font-medium text-gray-900 dark:text-gray-100">自动索引</div>
                     <div class="text-xs text-gray-500">文件变更时自动更新 (1.5s 防抖)</div>
                   </div>
                 </div>
                 <n-switch :value="autoIndexEnabled" @update:value="toggleAutoIndex" />
               </div>
            </ConfigSection>

            <div class="mt-2">
              <ProjectIndexManager />
            </div>
          </n-space>
        </n-scrollbar>
      </n-tab-pane>
    </n-tabs>
  </div>
</template>
