# 图标工坊集成技术规范

> 创建时间: 2026-01-07
> 状态: 已完成

## 一、项目背景

### 1.1 源项目分析

原项目 `mcp-server-icon-main` 是一个基于 Node.js 的 MCP 服务器，主要功能：

| 功能 | 技术实现 | 集成决策 |
|------|----------|----------|
| 图标搜索 | Iconfont API (`https://www.iconfont.cn/api/icon/search.json`) | ✅ 迁移至 Rust |
| 图标预览 | Web 界面 (Node.js HTTP Server + WebSocket) | ✅ 替换为 Vue 组件 |
| 图标下载/复制 | HTTP 请求 + 文件系统操作 | ✅ Tauri 原生实现 |
| 缓存管理 | 内存 Map 缓存 | ✅ Rust 内存缓存 |
| Web 服务器 | Node.js HTTP + WebSocket | ❌ 不需要（原生集成） |

### 1.2 集成方案选型

**选定方案: 原生深度集成（方案 A）**

- **后端**: Rust (Tauri) - 直接调用 Iconfont API
- **前端**: Vue 组件 - 集成到现有工具箱
- **优势**: 无 Node.js 依赖，单一可执行文件，UI 体验统一

---

## 二、功能范围

### 2.1 已实现的功能（5项）

1. **search_icons** - 图标关键词搜索
2. **get_icon_content** - 获取图标内容（SVG）
3. **save_icons** - 保存图标到项目
4. **get_icon_cache_stats** - 获取缓存统计
5. **clear_icon_cache** - 清空缓存

### 2.2 筛选维度

| 筛选项 | API 参数 | 默认值 |
|--------|----------|--------|
| 关键词 | `q` | 必填 |
| 图标风格 | `sType` | `all` (线性/面性/扁平) |
| 填充类型 | `fills` | `all` (单色/多色) |
| 排序方式 | `sortType` | `relate` |
| 每页数量 | `pageSize` | `50` |

---

## 三、架构设计

### 3.1 后端结构 (Rust)

```
src/rust/mcp/tools/
├── mod.rs              ← 添加 icon 模块导出
└── icon/               ← 新增模块
    ├── mod.rs          ← 模块入口
    ├── mcp.rs          ← MCP 工具定义 (IconTool)
    ├── types.rs        ← 类型定义
    ├── api.rs          ← Iconfont API 封装
    └── commands.rs     ← Tauri 命令
```

### 3.2 前端结构 (Vue)

```
src/frontend/
├── components/
│   └── tools/
│       └── IconWorkshop/           ← 新增组件目录
│           ├── index.ts            ← 组件导出
│           ├── IconWorkshop.vue    ← 主组件（集成到 McpToolsTab）
│           ├── IconCard.vue        ← 单个图标卡片
│           └── IconSaveModal.vue   ← 保存配置模态框
├── composables/
│   └── useIconSearch.ts            ← 图标搜索 Hook
└── types/
    └── icon.ts                     ← TypeScript 类型定义
```

---

## 四、Tauri 命令清单

| 命令名称 | 参数 | 返回值 | 说明 |
|----------|------|--------|------|
| `search_icons` | `IconSearchRequest` | `IconSearchResult` | 搜索图标 |
| `get_icon_content` | `{ id, format }` | `{ content, mimeType }` | 获取图标内容 |
| `save_icons` | `IconSaveRequest` | `{ success, savedPaths }` | 保存图标文件 |
| `get_icon_cache_stats` | - | `CacheStats` | 获取缓存统计 |
| `clear_icon_cache` | `{ expiredOnly? }` | `{ cleared }` | 清空缓存 |
| `get_icon_config` | - | `{ defaultPath }` | 获取保存配置 |
| `set_icon_config` | `{ defaultPath }` | - | 设置保存配置 |
| `copy_icon_to_clipboard` | `IconItem` | - | 复制 SVG 到剪贴板 |
| `select_icon_save_directory` | `{ defaultPath? }` | `string \| null` | 选择保存目录 |

---

## 五、UI/UX 设计规范

### 5.1 集成位置

- **入口**: `McpToolsTab` 中新增 "图标工坊" 工具卡片
- **打开方式**: 点击"配置"按钮打开模态框

### 5.2 交互设计

| 交互点 | 设计 |
|--------|------|
| 搜索栏 | 输入框 + 筛选折叠面板 |
| 图标网格 | 4-6 列自适应网格，响应式 |
| 悬停效果 | 放大 1.05x + 阴影加深 |
| 选中状态 | 右上角勾选标记 + 边框高亮 |
| 加载状态 | 骨架屏（网格占位卡片 + 脉冲动画） |
| 空状态 | 插画 + 提示文字 |

### 5.3 视觉风格

- **配色**: 紫色主题 (#8b5cf6)
- **卡片**: 玻璃态背景 + 柔和阴影
- **动画**: 0.2s 过渡，cubic-bezier(0.4, 0, 0.2, 1)

---

## 六、API 对接

### 6.1 Iconfont 搜索 API

**端点**: `https://www.iconfont.cn/api/icon/search.json`

**方法**: `POST` (Form-encoded)

**参数**:
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `q` | string | ✅ | 搜索关键词 |
| `sortType` | string | ❌ | 排序方式 (relate/new/hot) |
| `page` | number | ❌ | 页码，默认 1 |
| `pageSize` | number | ❌ | 每页数量，默认 50 |
| `sType` | string | ❌ | 图标风格 |
| `fills` | string | ❌ | 填充类型 |

---

## 七、文件修改清单

### 后端 (Rust)

| 文件路径 | 操作 |
|----------|------|
| `src/rust/mcp/tools/icon/mod.rs` | 新增 |
| `src/rust/mcp/tools/icon/types.rs` | 新增 |
| `src/rust/mcp/tools/icon/api.rs` | 新增 |
| `src/rust/mcp/tools/icon/commands.rs` | 新增 |
| `src/rust/mcp/tools/icon/mcp.rs` | 新增 |
| `src/rust/mcp/tools/mod.rs` | 修改 |
| `src/rust/mcp/commands.rs` | 修改 |
| `src/rust/app/builder.rs` | 修改 |

### 前端 (TypeScript/Vue)

| 文件路径 | 操作 |
|----------|------|
| `src/frontend/types/icon.ts` | 新增 |
| `src/frontend/composables/useIconSearch.ts` | 新增 |
| `src/frontend/components/tools/IconWorkshop/index.ts` | 新增 |
| `src/frontend/components/tools/IconWorkshop/IconWorkshop.vue` | 新增 |
| `src/frontend/components/tools/IconWorkshop/IconCard.vue` | 新增 |
| `src/frontend/components/tools/IconWorkshop/IconSaveModal.vue` | 新增 |
| `src/frontend/components/tabs/McpToolsTab.vue` | 修改 |

---

## 八、设计原则遵循

- ✅ **KISS**: 最小化依赖，无 Node.js 运行时
- ✅ **YAGNI**: 仅实现确认的 5 项核心功能
- ✅ **SOLID**: 模块分离，类型安全，职责单一
