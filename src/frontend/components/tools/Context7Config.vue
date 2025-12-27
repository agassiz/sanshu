<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { useMessage } from 'naive-ui'
import { onMounted, ref } from 'vue'
import ConfigSection from '../common/ConfigSection.vue'

const props = defineProps<{ active: boolean }>()
const message = useMessage()

// Config State
const config = ref({ api_key: '' })

// Testing State
const testLoading = ref(false)
const testResult = ref<{ success: boolean; message: string; preview?: string } | null>(null)
const testLibrary = ref('spring-projects/spring-framework')
const testTopic = ref('core')

// Popular Libraries Data
const popularLibs = [
  { label: 'Spring Framework', value: 'spring-projects/spring-framework', category: 'Java' },
  { label: 'Spring Boot', value: 'spring-projects/spring-boot', category: 'Java' },
  { label: 'MyBatis', value: 'mybatis/mybatis-3', category: 'Java' },
  { label: 'React', value: 'facebook/react', category: '前端' },
  { label: 'Vue.js', value: 'vuejs/vue', category: '前端' },
  { label: 'Next.js', value: 'vercel/next.js', category: '前端' },
  { label: 'FastAPI', value: 'tiangolo/fastapi', category: '后端' },
  { label: 'Tokio', value: 'tokio-rs/tokio', category: 'Rust' },
  { label: 'Tauri', value: 'tauri-apps/tauri', category: 'Rust' },
]

// --- Actions ---

async function loadConfig() {
  try {
    const res = await invoke('get_context7_config') as { api_key?: string }
    config.value = { api_key: res.api_key || '' }
  } catch (err) {
    message.error(`加载配置失败: ${err}`)
  }
}

async function saveConfig() {
  try {
    await invoke('save_context7_config', { apiKey: config.value.api_key })
    message.success('Context7 配置已保存')
  } catch (err) {
    message.error(`保存失败: ${err}`)
  }
}

async function runTest() {
  testLoading.value = true
  testResult.value = null
  try {
    const res = await invoke('test_context7_connection', {
      library: testLibrary.value || null,
      topic: testTopic.value || null,
    }) as any
    
    testResult.value = res
    if (res.success) message.success('测试成功')
    else message.error(res.message)
    
  } catch (err) {
    testResult.value = { success: false, message: `System Error: ${err}` }
    message.error(`测试异常: ${err}`)
  } finally {
    testLoading.value = false
  }
}

// Lifecycle
onMounted(() => {
  if (props.active) loadConfig()
})

defineExpose({ saveConfig })
</script>

<template>
  <div class="h-full px-1">
    <n-scrollbar style="max-height: 70vh" class="pr-2">
      <n-space vertical size="large">
        <n-alert type="info" :bordered="false" class="mb-2">
          <template #icon><div class="i-carbon-information" /></template>
          Context7 提供最新的框架和库文档查询服务。
        </n-alert>

        <ConfigSection title="认证设置" description="配置 Context7 API Key 以获得更高的速率限制">
          <n-form-item label="API Key (可选)">
            <n-input
              v-model:value="config.api_key"
              type="password"
              show-password-on="click"
              placeholder="留空即使用免费模式"
              clearable
            />
            <template #feedback>
               免费模式有限制。获取 Key: <a href="https://context7.com/dashboard" target="_blank" class="text-primary-500 hover:underline">context7.com</a>
            </template>
          </n-form-item>
          
          <div class="flex justify-end mt-2">
            <n-button type="primary" @click="saveConfig">保存配置</n-button>
          </div>
        </ConfigSection>

        <ConfigSection title="连接与查询测试" description="测试是否能成功解析指定库的文档">
          <n-space vertical size="medium">
            <n-form-item label="测试目标库">
              <n-auto-complete
                v-model:value="testLibrary"
                :options="popularLibs.map(l => ({ label: l.label, value: l.value, group: l.category }))"
                placeholder="owner/repo (e.g. facebook/react)"
                clearable
              />
            </n-form-item>
            
            <n-form-item label="查询主题 (可选)">
              <n-input v-model:value="testTopic" placeholder="e.g. routing, state management" />
            </n-form-item>

            <div class="flex justify-end">
              <n-button 
                secondary 
                type="info" 
                :loading="testLoading" 
                :disabled="!testLibrary"
                @click="runTest"
              >
                <template #icon><div class="i-carbon-play" /></template>
                测试查询
              </n-button>
            </div>

            <!-- Test Result -->
            <transition name="fade">
              <div v-if="testResult" class="mt-4">
                <div 
                  class="p-3 rounded-lg border text-sm"
                  :class="testResult.success 
                    ? 'bg-green-50 border-green-200 text-green-800 dark:bg-green-900/20 dark:border-green-800 dark:text-green-300' 
                    : 'bg-red-50 border-red-200 text-red-800 dark:bg-red-900/20 dark:border-red-800 dark:text-red-300'"
                >
                  <div class="flex items-center gap-2 font-medium mb-1">
                    <div :class="testResult.success ? 'i-carbon-checkmark-filled' : 'i-carbon-warning-filled'" />
                    {{ testResult.success ? '测试成功' : '测试失败' }}
                  </div>
                  <div>{{ testResult.message }}</div>
                </div>

                <div v-if="testResult.preview" class="mt-2">
                   <div class="text-xs opacity-60 mb-1">响应预览:</div>
                   <div class="bg-gray-50 dark:bg-gray-800 p-2 rounded text-xs font-mono max-h-48 overflow-y-auto border border-gray-200 dark:border-gray-700">
                     {{ testResult.preview }}
                   </div>
                </div>
              </div>
            </transition>
          </n-space>
        </ConfigSection>

        <!-- Quick Tags -->
        <div class="pb-4">
          <div class="text-xs font-medium text-gray-500 mb-2">常用库参考</div>
          <n-space size="small">
             <n-tag 
               v-for="lib in popularLibs" 
               :key="lib.value" 
               size="small" 
               clickable 
               :bordered="false"
               @click="testLibrary = lib.value"
             >
               {{ lib.label }}
             </n-tag>
          </n-space>
        </div>
      </n-space>
    </n-scrollbar>
  </div>
</template>

<style scoped>
.fade-enter-active, .fade-leave-active {
  transition: opacity 0.3s ease;
}
.fade-enter-from, .fade-leave-to {
  opacity: 0;
}
</style>
