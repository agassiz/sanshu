# 测试图标弹窗模式脚本
# 用法: ./test_icon_popup.ps1 [release|debug]

param (
    [string]$Mode = "debug"
)

$ErrorActionPreference = "Stop"

# 确定可执行文件路径
$ExePath = "target/$Mode/sanshu.exe"

if (-not (Test-Path $ExePath)) {
    Write-Host "错误: 找不到可执行文件 $ExePath" -ForegroundColor Red
    Write-Host "请先尝试编译: cargo build" + $(if ($Mode -eq "release") { " --release" } else { "" })
    exit 1
}

Write-Host "=== 开始测试图标弹窗模式 ($Mode) ===" -ForegroundColor Cyan

# 测试用例 1: 基础搜索
Write-Host "`n[测试 1] 基础搜索 'settings'" -ForegroundColor Yellow
Start-Process -FilePath $ExePath -ArgumentList "--icon-search", "settings" -Wait
Write-Host "测试 1 完成" -ForegroundColor Green

# 测试用例 2: 指定风格 (线性) 和 关键词
Write-Host "`n[测试 2] 搜索 'user'，风格 'line'" -ForegroundColor Yellow
$Env:SANSHU_LOG = "debug" # 开启日志以便观察输出
Start-Process -FilePath $ExePath -ArgumentList "--icon-search", "user", "--style", "line" -Wait
Write-Host "测试 2 完成" -ForegroundColor Green

# 测试用例 3: 指定保存路径
Write-Host "`n[测试 3] 搜索 'file'，指定保存路径 './test_output'" -ForegroundColor Yellow
# 创建输出目录
New-Item -ItemType Directory -Force -Path "./test_output" | Out-Null
Start-Process -FilePath $ExePath -ArgumentList "--icon-search", "file", "--save-path", "./test_output" -Wait
Write-Host "测试 3 完成。请检查 ./test_output 目录。" -ForegroundColor Green

Write-Host "`n=== 所有测试已触发 ===" -ForegroundColor Cyan
Write-Host "注意：如果弹窗正常弹出并能交互，说明CLI参数解析和前端路由正常。"
