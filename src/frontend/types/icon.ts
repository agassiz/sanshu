// 图标工坊类型定义

// ============ 搜索相关类型 ============

/** 图标搜索请求参数 */
export interface IconSearchParams {
  /** 搜索关键词（必填） */
  query: string
  /** 图标风格: line(线性) | fill(面性) | flat(扁平) | all(全部) */
  style?: 'line' | 'fill' | 'flat' | 'all'
  /** 填充类型: single(单色) | multi(多色) | all(全部) */
  fills?: 'single' | 'multi' | 'all'
  /** 排序方式: relate(相关度) | new(最新) | hot(最热) */
  sortType?: 'relate' | 'new' | 'hot'
  /** 页码，默认 1 */
  page?: number
  /** 每页数量，默认 50 */
  pageSize?: number
  /** 是否仅从精选集搜索 */
  fromCollection?: boolean
}

/** 单个图标项 */
export interface IconItem {
  /** 图标 ID */
  id: number
  /** 图标名称 */
  name: string
  /** 图标标识（用于 CSS 类名） */
  fontClass: string
  /** 图标 Unicode 编码 */
  unicode?: string
  /** SVG 内容 */
  svgContent?: string
  /** 缩略图 URL */
  previewUrl?: string
  /** 作者名称 */
  author?: string
  /** 所属图标库名称 */
  repositoryName?: string
  /** 所属图标库 ID */
  repositoryId?: number
  /** 创建时间 */
  createdAt?: string
}

/** 图标搜索结果 */
export interface IconSearchResult {
  /** 图标列表 */
  icons: IconItem[]
  /** 匹配的总数量 */
  total: number
  /** 当前页码 */
  page: number
  /** 每页数量 */
  pageSize: number
  /** 是否有下一页 */
  hasMore: boolean
}

// ============ 保存相关类型 ============

/** 图标格式枚举 */
export type IconFormat = 'svg' | 'png' | 'both'

/** 图标保存请求 */
export interface IconSaveRequest {
  /** 要保存的图标列表 */
  icons: IconItem[]
  /** 保存目录路径 */
  savePath: string
  /** 保存格式 */
  format: IconFormat
  /** PNG 尺寸（仅当 format 包含 PNG 时有效） */
  pngSize?: number
}

/** 单个图标保存结果 */
export interface IconSaveItem {
  /** 图标 ID */
  id: number
  /** 图标名称 */
  name: string
  /** 保存成功 */
  success: boolean
  /** 保存的文件路径列表 */
  savedPaths: string[]
  /** 错误信息（如果失败） */
  error?: string
}

/** 图标保存结果 */
export interface IconSaveResult {
  /** 保存结果列表 */
  items: IconSaveItem[]
  /** 成功数量 */
  successCount: number
  /** 失败数量 */
  failedCount: number
  /** 保存目录 */
  savePath: string
}

// ============ 内容获取相关类型 ============

/** 获取图标内容请求 */
export interface IconContentRequest {
  /** 图标 ID */
  id: number
  /** 请求格式 */
  format?: IconFormat
  /** PNG 尺寸（仅 PNG 格式有效） */
  pngSize?: number
}

/** 图标内容响应 */
export interface IconContentResult {
  /** 图标 ID */
  id: number
  /** 图标名称 */
  name: string
  /** SVG 内容 */
  svgContent?: string
  /** PNG Base64 编码内容 */
  pngBase64?: string
  /** 文件 MIME 类型 */
  mimeType: string
}

// ============ 缓存管理相关类型 ============

/** 缓存统计信息 */
export interface IconCacheStats {
  /** 缓存条目总数 */
  totalEntries: number
  /** 有效条目数（未过期） */
  validEntries: number
  /** 已过期条目数 */
  expiredEntries: number
  /** 缓存过期时间（分钟） */
  cacheExpiryMinutes: number
  /** 预估内存占用（字节） */
  memoryUsageBytes?: number
}

/** 清空缓存请求 */
export interface ClearCacheRequest {
  /** 仅清空已过期的缓存项 */
  expiredOnly?: boolean
}

/** 清空缓存结果 */
export interface ClearCacheResult {
  /** 清空的缓存条目数 */
  clearedCount: number
  /** 剩余缓存条目数 */
  remainingCount: number
}

// ============ 配置相关类型 ============

/** 图标工坊配置 */
export interface IconConfig {
  /** 默认保存路径 */
  defaultSavePath?: string
  /** 默认保存格式 */
  defaultFormat?: IconFormat
  /** 默认 PNG 尺寸 */
  defaultPngSize?: number
  /** 缓存过期时间（分钟） */
  cacheExpiryMinutes?: number
}

// ============ UI 状态类型 ============

/** 筛选选项配置 */
export interface IconFilterOptions {
  /** 图标风格选项 */
  styles: Array<{ label: string, value: string }>
  /** 填充类型选项 */
  fills: Array<{ label: string, value: string }>
  /** 排序方式选项 */
  sortTypes: Array<{ label: string, value: string }>
}

/** 图标工坊 UI 状态 */
export interface IconWorkshopState {
  /** 是否正在加载 */
  loading: boolean
  /** 当前搜索参数 */
  searchParams: IconSearchParams
  /** 搜索结果 */
  searchResult: IconSearchResult | null
  /** 选中的图标 ID 列表 */
  selectedIds: Set<number>
  /** 错误信息 */
  error: string | null
  /** 是否显示筛选面板 */
  showFilters: boolean
  /** 是否显示保存模态框 */
  showSaveModal: boolean
}

/** 默认筛选选项 */
export const DEFAULT_FILTER_OPTIONS: IconFilterOptions = {
  styles: [
    { label: '全部', value: 'all' },
    { label: '线性', value: 'line' },
    { label: '面性', value: 'fill' },
    { label: '扁平', value: 'flat' },
  ],
  fills: [
    { label: '全部', value: 'all' },
    { label: '单色', value: 'single' },
    { label: '多色', value: 'multi' },
  ],
  sortTypes: [
    { label: '相关度', value: 'relate' },
    { label: '最新', value: 'new' },
    { label: '最热', value: 'hot' },
  ],
}

/** 默认搜索参数 */
export const DEFAULT_SEARCH_PARAMS: IconSearchParams = {
  query: '',
  style: 'all',
  fills: 'all',
  sortType: 'relate',
  page: 1,
  pageSize: 50,
  fromCollection: false,
}
