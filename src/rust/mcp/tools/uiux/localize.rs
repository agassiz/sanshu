// UI/UX Pro Max 文案本地化
// 仅提供 zh/en 简洁文案，避免过度设计

use super::engine::{SearchResult, SuggestResult};
use super::types::{UiuxLang, UiuxMode};

pub fn localize_text(lang: UiuxLang, zh: &str, en: &str) -> String {
    match lang {
        UiuxLang::Zh => zh.to_string(),
        UiuxLang::En => en.to_string(),
    }
}

pub fn error_text(lang: UiuxLang, message: &str) -> String {
    match lang {
        UiuxLang::Zh => format!("发生错误: {}", message),
        UiuxLang::En => format!("Error: {}", message),
    }
}

pub fn search_summary(lang: UiuxLang, mode: UiuxMode, result: &SearchResult) -> String {
    if let Some(err) = &result.error {
        return error_text(lang, err);
    }

    match mode {
        UiuxMode::Beautify => localize_text(lang, "已生成 UI 美化建议。", "UI beautify suggestions generated."),
        _ => {
            let zh = format!("已获取检索结果，领域：{}，共 {} 条。", result.domain, result.count);
            let en = format!("Search completed. Domain: {}. Results: {}.", result.domain, result.count);
            localize_text(lang, &zh, &en)
        }
    }
}

pub fn stack_summary(lang: UiuxLang, result: &SearchResult) -> String {
    if let Some(err) = &result.error {
        return error_text(lang, err);
    }

    let stack = result.stack.clone().unwrap_or_else(|| "-".to_string());
    let zh = format!("已获取栈指南：{}，共 {} 条。", stack, result.count);
    let en = format!("Stack guidelines: {}. Results: {}.", stack, result.count);
    localize_text(lang, &zh, &en)
}

pub fn design_system_summary(lang: UiuxLang, project_name: &str, persisted: bool) -> String {
    if persisted {
        let zh = format!("已生成并写入设计系统：{}。", project_name);
        let en = format!("Design system generated and persisted: {}.", project_name);
        localize_text(lang, &zh, &en)
    } else {
        let zh = format!("已生成设计系统建议：{}。", project_name);
        let en = format!("Design system recommendations generated: {}.", project_name);
        localize_text(lang, &zh, &en)
    }
}

pub fn beautify_summary(lang: UiuxLang) -> String {
    localize_text(lang, "已生成 UI 美化建议。", "UI beautify suggestions generated.")
}

pub fn suggest_summary(lang: UiuxLang, result: &SuggestResult) -> String {
    if result.should_suggest {
        let keywords = if result.matched_keywords.is_empty() {
            "-".to_string()
        } else {
            result.matched_keywords.join(", ")
        };
        let zh = format!("建议使用 UI/UX 工具，匹配关键词：{}。", keywords);
        let en = format!("UI/UX tool suggested. Matched keywords: {}.", keywords);
        localize_text(lang, &zh, &en)
    } else {
        localize_text(lang, "暂无明显 UI/UX 需求。", "No strong UI/UX signal detected.")
    }
}
