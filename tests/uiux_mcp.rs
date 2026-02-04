use rmcp::model::CallToolResult;
use serde_json::json;

use sanshu::tools::UiuxTool;

fn extract_first_text(result: &CallToolResult) -> String {
    let v = serde_json::to_value(&result.content).expect("Content 应可序列化为 JSON");
    let arr = v.as_array().expect("CallToolResult.content 应为数组");
    let first = arr.first().expect("CallToolResult.content 不应为空");

    // 优先按 MCP 规范的 text 内容字段提取
    if let Some(text) = first.get("text").and_then(|x| x.as_str()) {
        return text.to_string();
    }

    // 兼容：部分实现可能使用 data 字段承载文本
    if let Some(text) = first.get("data").and_then(|x| x.as_str()) {
        return text.to_string();
    }

    panic!("无法从 content 提取文本: {}", v);
}

fn parse_uiux_json(text: &str) -> serde_json::Value {
    serde_json::from_str(text).expect("uiux_* 工具应输出 JSON 文本")
}

#[tokio::test]
async fn uiux_suggest_detects_uiux_signal() {
    let result = UiuxTool::call_tool("uiux_suggest", json!({ "text": "UI/UX 美化一下" }))
        .await
        .expect("uiux_suggest 调用应成功");
    let text = extract_first_text(&result);
    let v = parse_uiux_json(&text);

    assert_eq!(v["meta"]["tool"].as_str(), Some("uiux_suggest"));
    assert_eq!(v["data"]["result"]["should_suggest"].as_bool(), Some(true));
}

#[tokio::test]
async fn uiux_suggest_detects_zh_triggers() {
    let result = UiuxTool::call_tool("uiux_suggest", json!({ "text": "这个界面太丑了，帮我美化优化一下" }))
        .await
        .expect("uiux_suggest 调用应成功");
    let text = extract_first_text(&result);
    let v = parse_uiux_json(&text);

    assert_eq!(v["data"]["result"]["should_suggest"].as_bool(), Some(true));
}

#[tokio::test]
async fn uiux_search_routes_zh_color_query_to_color_domain() {
    let result = UiuxTool::call_tool(
        "uiux_search",
        json!({
            "query": "配色 颜色 方案",
            "output_format": "json"
        }),
    )
    .await
    .expect("uiux_search 调用应成功");
    let text = extract_first_text(&result);
    let v = parse_uiux_json(&text);

    assert_eq!(v["meta"]["tool"].as_str(), Some("uiux_search"));
    assert_eq!(v["data"]["result"]["domain"].as_str(), Some("color"));
    assert!(v["data"]["result"]["count"].as_u64().unwrap_or(0) > 0);
}

#[tokio::test]
async fn uiux_search_fuzzy_fallback_handles_typo() {
    let result = UiuxTool::call_tool(
        "uiux_search",
        json!({
            "query": "glasomorphism",
            "domain": "style",
            "output_format": "json"
        }),
    )
    .await
    .expect("uiux_search 调用应成功");
    let text = extract_first_text(&result);
    let v = parse_uiux_json(&text);

    assert_eq!(v["data"]["result"]["domain"].as_str(), Some("style"));
    let count = v["data"]["result"]["count"].as_u64().unwrap_or(0);
    assert!(
        count > 0,
        "uiux_search fuzzy 回退未返回结果（count=0），响应如下：\n{}",
        text
    );
}

