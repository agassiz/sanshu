// UI/UX Pro Max MCP 工具请求类型

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UiuxOutputFormat {
    Json,
    Text,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UiuxLang {
    Zh,
    En,
}

impl UiuxLang {
    pub fn as_str(&self) -> &'static str {
        match self {
            UiuxLang::Zh => "zh",
            UiuxLang::En => "en",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UiuxMode {
    Search,
    Beautify,
    DesignSystem,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UiuxSearchRequest {
    pub query: String,
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(default)]
    pub max_results: Option<u32>,
    #[serde(default)]
    pub output_format: Option<UiuxOutputFormat>,
    #[serde(default)]
    pub lang: Option<UiuxLang>,
    #[serde(default)]
    pub mode: Option<UiuxMode>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UiuxStackRequest {
    pub query: String,
    pub stack: String,
    #[serde(default)]
    pub max_results: Option<u32>,
    #[serde(default)]
    pub output_format: Option<UiuxOutputFormat>,
    #[serde(default)]
    pub lang: Option<UiuxLang>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UiuxDesignSystemRequest {
    pub query: String,
    #[serde(default)]
    pub project_name: Option<String>,
    #[serde(default)]
    pub format: Option<String>, // ascii | markdown
    #[serde(default)]
    pub persist: Option<bool>,
    #[serde(default)]
    pub page: Option<String>,
    #[serde(default)]
    pub output_dir: Option<String>,
    #[serde(default)]
    pub output_format: Option<UiuxOutputFormat>,
    #[serde(default)]
    pub lang: Option<UiuxLang>,
    #[serde(default)]
    pub mode: Option<UiuxMode>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UiuxSuggestRequest {
    pub text: String,
    #[serde(default)]
    pub output_format: Option<UiuxOutputFormat>,
    #[serde(default)]
    pub lang: Option<UiuxLang>,
}
