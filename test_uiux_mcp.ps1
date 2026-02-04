# UIUX MCP 一键测试脚本（PowerShell）
# - 默认运行：UIUX MCP 集成测试（tests/uiux_mcp.rs）
# - 可选：只跑 UIUX engine 单测、跑 doctest、或跑全量测试
#
# 用法示例：
#   powershell -ExecutionPolicy Bypass -File .\\test_uiux_mcp.ps1
#   .\\test_uiux_mcp.ps1 -Mode uiux_engine
#   .\\test_uiux_mcp.ps1 -Mode all
#   .\\test_uiux_mcp.ps1 -Release
#   .\\test_uiux_mcp.ps1 -NoCapture

param(
    [ValidateSet("uiux_mcp", "uiux_engine", "doc", "all")]
    [string]$Mode = "uiux_mcp",
    [switch]$Release,
    [switch]$NoCapture
)

# 确保脚本在项目根目录执行，避免相对路径导致资源找不到
Set-Location -Path $PSScriptRoot

$ErrorActionPreference = "Stop"

function Assert-Command {
    param([string]$CommandName)
    if (-not (Get-Command $CommandName -ErrorAction SilentlyContinue)) {
        throw "未找到命令：$CommandName。请先安装 Rust（cargo）并配置到 PATH。"
    }
}

Assert-Command "cargo"

$profileArgs = @()
if ($Release) {
    $profileArgs += "--release"
}

$cargoArgs = @()
switch ($Mode) {
    "uiux_mcp" {
        Write-Host "[uiux-test] 运行 UIUX MCP 集成测试（tests/uiux_mcp.rs）..." -ForegroundColor Cyan
        $cargoArgs = @("test", "--test", "uiux_mcp")
    }
    "uiux_engine" {
        Write-Host "[uiux-test] 运行 UIUX engine 相关单测（过滤）..." -ForegroundColor Cyan
        # 说明：这里使用测试名过滤，避免跑全量单测
        $cargoArgs = @("test", "--lib", "mcp::tools::uiux::engine::tests")
    }
    "doc" {
        Write-Host "[uiux-test] 运行 doctest ..." -ForegroundColor Cyan
        $cargoArgs = @("test", "--doc")
    }
    "all" {
        Write-Host "[uiux-test] 运行全量测试（含 doctest）..." -ForegroundColor Cyan
        $cargoArgs = @("test")
    }
}

if ($NoCapture) {
    $cargoArgs += "--"
    $cargoArgs += "--nocapture"
}

Write-Host ("[uiux-test] ProjectRoot: {0}" -f $PSScriptRoot) -ForegroundColor DarkGray
Write-Host ("[uiux-test] Command: cargo {0} {1}" -f ($cargoArgs -join " "), ($profileArgs -join " ")) -ForegroundColor DarkGray

& cargo @cargoArgs @profileArgs
exit $LASTEXITCODE

