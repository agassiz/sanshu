import type { CustomPrompt } from '../types/popup'

// 复用条件性 prompt 的上下文拼接逻辑，保持与弹窗输入一致
export function buildConditionalContext(prompts: CustomPrompt[]): string {
  const conditionalTexts: string[] = []

  prompts.forEach((prompt) => {
    const isEnabled = prompt.current_state ?? false
    const template = isEnabled ? prompt.template_true : prompt.template_false

    if (template && template.trim()) {
      conditionalTexts.push(template.trim())
    }
  })

  return conditionalTexts.join('\n')
}
