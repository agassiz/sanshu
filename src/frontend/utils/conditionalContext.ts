import type { CustomPrompt, McpRequest } from '../types/popup'

// UI/UX 上下文策略状态信息（用于 UI 可视化展示）
export interface ContextPolicyStatus {
  // 是否允许追加上下文
  allowed: boolean
  // 状态图标类名
  icon: string
  // 状态颜色（Tailwind 类名）
  colorClass: string
  // 状态标签文本
  label: string
  // 策略原因（来自 uiux_reason 或自动生成）
  reason: string
  // 原始意图值
  intent: 'none' | 'beautify' | 'page_refactor' | 'uiux_search'
  // 原始策略值
  policy: 'auto' | 'force' | 'forbid'
}

// 意图名称映射（用于 UI 展示）
const INTENT_LABELS: Record<string, string> = {
  none: '无特定意图',
  beautify: 'UI 美化',
  page_refactor: '页面重构',
  uiux_search: 'UI/UX 搜索',
}

// 策略名称映射（用于 UI 展示）
export const POLICY_LABELS: Record<string, string> = {
  auto: '自动',
  force: '强制追加',
  forbid: '禁止追加',
}

/**
 * 获取 UI/UX 上下文策略状态信息（用于 UI 可视化展示）
 * @param request MCP 请求对象
 * @returns 策略状态信息，包含是否允许、图标、颜色、标签、原因等
 */
export function getContextPolicyStatus(request?: McpRequest | null): ContextPolicyStatus {
  const intent = request?.uiux_intent ?? 'none'
  const policy = request?.uiux_context_policy ?? 'auto'
  const reason = request?.uiux_reason
  // 记录是否显式传入 UI/UX 上下文信号，便于区分默认与显式策略
  const hasExplicitSignal = !!(request?.uiux_intent || request?.uiux_context_policy || request?.uiux_reason)

  // 判断是否允许追加上下文
  const isForbidden = policy === 'forbid'
  const isAutoBlocked = policy === 'auto' && intent === 'none'
  const allowed = !isForbidden && !isAutoBlocked

  // 根据状态确定图标和颜色
  let icon: string
  let colorClass: string
  let label: string
  let generatedReason: string

  if (isForbidden) {
    // 策略明确禁止
    icon = 'i-carbon-close-outline'
    colorClass = 'text-red-400'
    label = '上下文已禁止'
    generatedReason = reason || '策略设置为禁止追加上下文'
  } else if (isAutoBlocked) {
    // 自动策略下因无意图而阻止
    icon = 'i-carbon-warning'
    colorClass = 'text-yellow-400'
    label = hasExplicitSignal ? '上下文未追加' : '上下文默认未追加'
    generatedReason = reason || (hasExplicitSignal
      ? '当前无 UI/UX 相关意图，未追加条件性上下文'
      : '未传入 UI/UX 上下文信号，按默认策略未追加')
  } else if (policy === 'force') {
    // 强制追加
    icon = 'i-carbon-checkmark-filled'
    colorClass = 'text-green-400'
    label = '上下文已追加'
    generatedReason = reason || `强制追加上下文（意图：${INTENT_LABELS[intent] || intent}）`
  } else {
    // 自动策略下允许追加（有意图）
    icon = 'i-carbon-checkmark'
    colorClass = 'text-blue-400'
    label = '上下文已追加'
    generatedReason = reason || `基于意图自动追加（${INTENT_LABELS[intent] || intent}）`
  }

  return {
    allowed,
    icon,
    colorClass,
    label,
    reason: generatedReason,
    intent,
    policy,
  }
}

/**
 * 判断是否应该显示策略指示器
 * @param request MCP 请求对象
 * @returns 只有显式传入 UI/UX 信号时才显示策略指示器（YAGNI：不显示用户不需要的信息）
 */
export function shouldShowPolicyIndicator(request?: McpRequest | null): boolean {
  if (!request) return false
  // 只有 AI 显式传入 UI/UX 参数时才显示策略指示器，避免非 UI 美化场景的无关提示
  return !!(request.uiux_intent || request.uiux_context_policy || request.uiux_reason)
}

/**
 * 上下文构建配置选项
 */
export interface BuildContextOptions {
  /** MCP 请求对象，用于 UI/UX 策略判断 */
  request?: McpRequest | null
  /** 上下文追加总开关（默认 true，用于控制普通条件类 prompt） */
  contextAppendEnabled?: boolean
  /** 是否强制包含工具开关类（默认 true，工具开关类始终生效） */
  includeToolSwitches?: boolean
}

/**
 * 构建条件性上下文内容
 *
 * 统一的上下文拼接逻辑，支持：
 * 1. 工具开关类 prompt（有 linked_mcp_tool）：始终生效，不受总开关和策略控制
 * 2. 普通条件类 prompt（无 linked_mcp_tool）：受总开关和 UI/UX 策略控制
 *
 * @param prompts 条件性 prompt 列表
 * @param options 构建配置选项
 * @returns 拼接后的上下文文本
 */
export function buildConditionalContext(
  prompts: CustomPrompt[],
  options?: BuildContextOptions | McpRequest | null,
): string {
  const parts: string[] = []

  // 兼容旧的调用方式：buildConditionalContext(prompts, request)
  let opts: BuildContextOptions = {}
  if (options && typeof options === 'object' && ('uiux_intent' in options || 'uiux_context_policy' in options || 'id' in options)) {
    // 旧格式：第二个参数是 McpRequest
    opts = { request: options as McpRequest | null }
  }
  else if (options) {
    opts = options as BuildContextOptions
  }

  const { request, contextAppendEnabled = true, includeToolSwitches = true } = opts

  // 分离工具开关类和普通条件类
  const toolSwitchPrompts = prompts.filter(p => p.type === 'conditional' && !!p.linked_mcp_tool)
  const regularPrompts = prompts.filter(p => p.type === 'conditional' && !p.linked_mcp_tool)

  // 1. 工具开关类：始终生效（不受总开关和策略控制）
  if (includeToolSwitches) {
    toolSwitchPrompts.forEach((prompt) => {
      const isEnabled = prompt.current_state ?? false
      const template = isEnabled ? prompt.template_true : prompt.template_false
      if (template?.trim()) {
        parts.push(template.trim())
      }
    })
  }

  // 2. 普通条件类：受总开关和 UI/UX 策略控制
  const intent = request?.uiux_intent ?? 'none'
  const policy = request?.uiux_context_policy ?? 'auto'

  // 判断是否应该包含普通条件类
  // - 总开关必须开启
  // - 策略不能是 'forbid'
  // - 如果策略是 'auto'，需要有明确意图（intent !== 'none'），或者没有传入 request（默认允许）
  const shouldIncludeRegular = contextAppendEnabled
    && policy !== 'forbid'
    && (policy === 'force' || intent !== 'none' || !request)

  if (shouldIncludeRegular) {
    regularPrompts.forEach((prompt) => {
      const isEnabled = prompt.current_state ?? false
      const template = isEnabled ? prompt.template_true : prompt.template_false
      if (template?.trim()) {
        parts.push(template.trim())
      }
    })
  }

  return parts.join('\n')
}
