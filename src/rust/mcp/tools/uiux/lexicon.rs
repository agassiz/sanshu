//! UI/UX 轻量词典与中英文映射
//!
//! 设计目标：
//! - 提升 `uiux_suggest` 的触发率（尤其是中文/中英混合输入）
//! - 为 BM25 提供“查询扩展（Query Expansion）”，在不引入 embedding 的前提下改善召回
//!
//! 约束：
//! - 不引入重量级 NLP 依赖；保持可维护、可扩展
//! - 词表应“少而精”，优先覆盖高频场景，可随使用反馈迭代

/// 中文 UI/UX 意图强触发词（出现即可认为有较强 UI/UX 诉求）。
///
/// 注意：这里尽量放“名词/动作词”，避免单独的风格形容词（如“优雅”）导致误触发。
pub const ZH_UIUX_STRONG_TRIGGERS: &[&str] = &[
    // 直接意图
    "美化", "优化", "改版", "重构",
    // 典型对象
    "界面", "页面", "登录", "注册", "落地页", "仪表盘",
    // 设计要素
    "布局", "配色", "颜色", "色彩", "字体", "排版", "动效", "动画", "交互",
    "按钮", "组件", "图标", "导航", "表单", "弹窗", "卡片", "列表", "表格", "图表",
    // 体验/规范
    "无障碍", "可访问性", "可用性", "易用性", "一致性", "对齐", "间距", "响应式", "适配",
];

/// 中文关键词 → UIUX 域名提示（用于 `detect_domain()`）。
///
/// 域名必须与 `DOMAIN_CONFIGS` 的 key 保持一致。
pub const ZH_DOMAIN_HINTS: &[(&str, &str)] = &[
    // 颜色 / 配色
    ("配色", "color"),
    ("颜色", "color"),
    ("色彩", "color"),
    ("色板", "color"),
    // 字体 / 排版
    ("字体", "typography"),
    ("排版", "typography"),
    ("字重", "typography"),
    ("字号", "typography"),
    // 图表 / 可视化
    ("图表", "chart"),
    ("可视化", "chart"),
    ("趋势图", "chart"),
    // 落地页
    ("落地页", "landing"),
    ("着陆页", "landing"),
    ("首页", "landing"),
    // 图标
    ("图标", "icons"),
    // 体验/规范
    ("无障碍", "ux"),
    ("可访问性", "ux"),
    ("可用性", "ux"),
    ("易用性", "ux"),
    ("交互", "ux"),
    // 技术/实现提示词
    ("提示词", "prompt"),
    ("tailwind", "prompt"),
    ("css", "prompt"),
    // 性能/框架（更偏“指导”类）
    ("react", "react"),
    ("性能", "react"),
    ("web", "web"),
];

/// Query Expansion：中文短语 → 英文关键词（尽量选择在内嵌 CSV 中高概率出现/有用的 token）。
///
/// 用途：
/// - 纯中文/中英混合输入时，为 BM25 补充英文 token，提高召回
/// - 通过同义概念扩展缓解“现代感 vs 科技感”等语义差异
pub const ZH_TO_EN_EXPANSIONS: &[(&str, &[&str])] = &[
    // 风格/气质（尽量映射到 styles.csv / prompts.csv 常见词）
    // 说明：这里的扩展会影响 BM25 召回，因此优先补充“在 style 语料中也常见”的 token（如 minimalism/swiss）。
    ("优雅", &["elegant", "refined", "premium", "minimalism", "swiss", "serif"]),
    ("高级", &["premium", "luxury", "elegant", "minimalism"]),
    ("专业", &["professional", "enterprise", "saas"]),
    ("简约", &["minimal", "minimalism", "clean"]),
    ("极简", &["minimal", "minimalism", "clean"]),
    ("清爽", &["clean", "minimal", "spacious"]),
    ("现代", &["modern", "clean", "minimal"]),
    ("科技感", &["futuristic", "neon", "hud", "cyberpunk", "retro-futurism", "aurora", "vaporwave"]),
    ("科幻", &["futuristic", "hud", "retro-futurism"]),
    ("赛博", &["cyberpunk", "neon", "retro-futurism"]),
    ("霓虹", &["neon", "glow"]),
    ("渐变", &["gradient", "aurora", "mesh"]),
    ("质感", &["premium", "luxury", "glassmorphism"]),
    ("未来", &["futuristic", "neon", "hud"]),
    ("暗黑", &["dark", "oled", "night"]),
    ("深色", &["dark", "oled", "night"]),
    ("玻璃", &["glassmorphism", "glass", "blur"]),
    ("毛玻璃", &["glassmorphism", "blur", "glass"]),
    ("新拟物", &["neumorphism", "soft", "embossed"]),
    ("扁平", &["flat", "design"]),
    ("粗野", &["brutalism", "stark"]),
    ("新粗野", &["neubrutalism", "brutalism"]),
    ("复古", &["retro", "vintage"]),
    ("可爱", &["playful", "soft"]),
    ("活泼", &["playful", "vibrant"]),
    ("立体", &["hyperrealism"]),
    // 页面/场景
    ("登录", &["login", "auth", "signin"]),
    ("注册", &["signup", "register"]),
    ("落地页", &["landing", "cta", "hero", "conversion"]),
    ("仪表盘", &["dashboard", "analytics", "kpi"]),
    // 设计要素
    ("配色", &["color", "palette", "contrast"]),
    ("字体", &["typography", "font", "heading"]),
    ("排版", &["typography", "hierarchy", "heading"]),
    ("动效", &["animation", "motion", "transition"]),
    ("交互", &["usability", "navigation", "focus"]),
    ("无障碍", &["accessibility", "wcag", "aria"]),
    ("图表", &["chart", "graph", "visualization"]),
    ("图标", &["icon", "icons", "svg"]),
    ("按钮", &["button", "cta"]),
    ("表单", &["form", "input"]),
    ("间距", &["spacing", "grid"]),
    ("对齐", &["alignment", "grid"]),
];

/// Query Expansion：英文 token → 同义/相关 token（轻量补充，避免过度扩展）。
pub const EN_SYNONYMS: &[(&str, &[&str])] = &[
    ("modern", &["contemporary", "clean", "minimal"]),
    ("futuristic", &["neon", "cyberpunk", "hud"]),
    ("elegant", &["refined", "premium", "luxury"]),
    ("glassmorphism", &["glass", "blur", "liquid"]),
    ("neumorphism", &["soft", "embossed", "debossed"]),
    ("aurora", &["gradient", "mesh", "iridescent"]),
    ("cyberpunk", &["neon", "hud", "vaporwave", "retro-futurism"]),
    ("brutalism", &["neubrutalism", "stark", "raw"]),
    ("dashboard", &["analytics", "kpi", "chart"]),
    ("landing", &["hero", "cta", "conversion"]),
    ("login", &["auth", "signin"]),
];

