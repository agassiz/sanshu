import type { CustomPrompt, McpRequest } from '../types/popup'

// 复用条件性 prompt 的上下文拼接逻辑，保持与弹窗输入一致
export function buildConditionalContext(prompts: CustomPrompt[], request?: McpRequest | null): string {
  const conditionalTexts: string[] = []

  // 根据 UI/UX 上下文策略决定是否追加条件性上下文
  const intent = request?.uiux_intent ?? 'none'
  const policy = request?.uiux_context_policy ?? 'auto'
  if (policy === 'forbid' || (policy === 'auto' && intent === 'none')) {
    return ''
  }

  prompts.forEach((prompt) => {
    const isEnabled = prompt.current_state ?? false
    const template = isEnabled ? prompt.template_true : prompt.template_false

    if (template && template.trim()) {
      conditionalTexts.push(template.trim())
    }
  })

  return conditionalTexts.join('\n')
}
