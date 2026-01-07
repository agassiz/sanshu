use serde::{Deserialize, Serialize};

// ============ 图标搜索相关类型 ============

/// 图标搜索请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconSearchRequest {
    /// 搜索关键词（必填）
    pub query: String,
    /// 图标风格: line(线性) | fill(面性) | flat(扁平) | all(全部)
    #[serde(default)]
    pub style: Option<String>,
    /// 填充类型: single(单色) | multi(多色) | all(全部)
    #[serde(default)]
    pub fills: Option<String>,
    /// 排序方式: relate(相关度) | new(最新) | hot(最热)
    #[serde(default)]
    pub sort_type: Option<String>,
    /// 页码，默认 1
    #[serde(default)]
    pub page: Option<u32>,
    /// 每页数量，默认 50
    #[serde(default)]
    pub page_size: Option<u32>,
    /// 是否仅从精选集搜索
    #[serde(default)]
    pub from_collection: Option<bool>,
}

impl Default for IconSearchRequest {
    fn default() -> Self {
        Self {
            query: String::new(),
            style: None,
            fills: None,
            sort_type: Some("relate".to_string()),
            page: Some(1),
            page_size: Some(50),
            from_collection: None,
        }
    }
}

/// 单个图标项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconItem {
    /// 图标 ID
    pub id: u64,
    /// 图标名称
    pub name: String,
    /// 图标标识（用于 CSS 类名）
    pub font_class: String,
    /// 图标 Unicode 编码
    #[serde(default)]
    pub unicode: Option<String>,
    /// SVG 内容（可选，需要单独请求）
    #[serde(default)]
    pub svg_content: Option<String>,
    /// 缩略图 URL
    #[serde(default)]
    pub preview_url: Option<String>,
    /// 作者名称
    #[serde(default)]
    pub author: Option<String>,
    /// 所属图标库名称
    #[serde(default)]
    pub repository_name: Option<String>,
    /// 所属图标库 ID
    #[serde(default)]
    pub repository_id: Option<u64>,
    /// 创建时间
    #[serde(default)]
    pub created_at: Option<String>,
}

/// 图标搜索结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconSearchResult {
    /// 图标列表
    pub icons: Vec<IconItem>,
    /// 匹配的总数量
    pub total: u32,
    /// 当前页码
    pub page: u32,
    /// 每页数量
    pub page_size: u32,
    /// 是否有下一页
    pub has_more: bool,
}

impl Default for IconSearchResult {
    fn default() -> Self {
        Self {
            icons: Vec::new(),
            total: 0,
            page: 1,
            page_size: 50,
            has_more: false,
        }
    }
}

// ============ 图标下载/保存相关类型 ============

/// 图标格式枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IconFormat {
    /// SVG 矢量格式
    Svg,
    /// PNG 位图格式
    Png,
    /// 同时保存 SVG 和 PNG
    Both,
}

impl Default for IconFormat {
    fn default() -> Self {
        Self::Svg
    }
}

/// 图标保存请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconSaveRequest {
    /// 要保存的图标列表
    pub icons: Vec<IconItem>,
    /// 保存目录路径
    pub save_path: String,
    /// 保存格式
    #[serde(default)]
    pub format: IconFormat,
    /// PNG 尺寸（仅当 format 包含 PNG 时有效）
    #[serde(default)]
    pub png_size: Option<u32>,
}

/// 单个图标保存结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconSaveItem {
    /// 图标 ID
    pub id: u64,
    /// 图标名称
    pub name: String,
    /// 保存成功
    pub success: bool,
    /// 保存的文件路径列表
    pub saved_paths: Vec<String>,
    /// 错误信息（如果失败）
    #[serde(default)]
    pub error: Option<String>,
}

/// 图标保存结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconSaveResult {
    /// 保存结果列表
    pub items: Vec<IconSaveItem>,
    /// 成功数量
    pub success_count: u32,
    /// 失败数量
    pub failed_count: u32,
    /// 保存目录
    pub save_path: String,
}

// ============ 图标内容获取相关类型 ============

/// 获取图标内容请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconContentRequest {
    /// 图标 ID
    pub id: u64,
    /// 请求格式
    #[serde(default)]
    pub format: IconFormat,
    /// PNG 尺寸（仅 PNG 格式有效）
    #[serde(default)]
    pub png_size: Option<u32>,
}

/// 图标内容响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconContentResult {
    /// 图标 ID
    pub id: u64,
    /// 图标名称
    pub name: String,
    /// SVG 内容
    #[serde(default)]
    pub svg_content: Option<String>,
    /// PNG Base64 编码内容
    #[serde(default)]
    pub png_base64: Option<String>,
    /// 文件 MIME 类型
    pub mime_type: String,
}

// ============ 缓存管理相关类型 ============

/// 缓存统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconCacheStats {
    /// 缓存条目总数
    pub total_entries: usize,
    /// 有效条目数（未过期）
    pub valid_entries: usize,
    /// 已过期条目数
    pub expired_entries: usize,
    /// 缓存过期时间（分钟）
    pub cache_expiry_minutes: u64,
    /// 预估内存占用（字节）
    #[serde(default)]
    pub memory_usage_bytes: Option<u64>,
}

/// 清空缓存请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearCacheRequest {
    /// 仅清空已过期的缓存项
    #[serde(default)]
    pub expired_only: bool,
}

/// 清空缓存结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearCacheResult {
    /// 清空的缓存条目数
    pub cleared_count: usize,
    /// 剩余缓存条目数
    pub remaining_count: usize,
}

// ============ 配置相关类型 ============

/// 图标工坊配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconConfig {
    /// 默认保存路径
    #[serde(default)]
    pub default_save_path: Option<String>,
    /// 默认保存格式
    #[serde(default)]
    pub default_format: IconFormat,
    /// 默认 PNG 尺寸
    #[serde(default)]
    pub default_png_size: Option<u32>,
    /// 缓存过期时间（分钟）
    #[serde(default)]
    pub cache_expiry_minutes: Option<u64>,
}

impl Default for IconConfig {
    fn default() -> Self {
        Self {
            default_save_path: Some("assets/icons".to_string()),
            default_format: IconFormat::Svg,
            default_png_size: Some(64),
            cache_expiry_minutes: Some(30),
        }
    }
}

// ============ Iconfont API 响应类型（内部使用）============

/// Iconfont API 原始响应结构
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct IconfontApiResponse {
    pub code: i32,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub data: Option<IconfontSearchData>,
}

/// Iconfont 搜索数据结构
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct IconfontSearchData {
    #[serde(default)]
    pub icons: Vec<IconfontIcon>,
    #[serde(default)]
    pub count: u32,
}

/// Iconfont 原始图标结构
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct IconfontIcon {
    pub id: u64,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub font_class: Option<String>,
    #[serde(default)]
    pub unicode: Option<String>,
    #[serde(default)]
    pub show_svg: Option<String>,
    #[serde(default)]
    pub user: Option<IconfontUser>,
    #[serde(default)]
    pub repository: Option<IconfontRepository>,
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Iconfont 用户信息
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct IconfontUser {
    #[serde(default)]
    pub nickname: Option<String>,
}

/// Iconfont 图标库信息
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct IconfontRepository {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
}

// ============ 类型转换实现 ============

impl From<IconfontIcon> for IconItem {
    fn from(icon: IconfontIcon) -> Self {
        Self {
            id: icon.id,
            name: icon.name.unwrap_or_else(|| format!("icon_{}", icon.id)),
            font_class: icon.font_class.unwrap_or_default(),
            unicode: icon.unicode,
            svg_content: icon.show_svg,
            preview_url: None, // 需要单独构建
            author: icon.user.and_then(|u| u.nickname),
            repository_name: icon.repository.as_ref().and_then(|r| r.name.clone()),
            repository_id: icon.repository.and_then(|r| r.id),
            created_at: icon.created_at,
        }
    }
}
