// 代理配置相关的 Tauri 命令
use tauri::{AppHandle, State};
use crate::config::{AppState, ProxyConfig, save_config};
use super::{ProxyDetector, ProxyInfo, proxy::ProxyType};

/// 获取代理配置
#[tauri::command]
pub async fn get_proxy_config(state: State<'_, AppState>) -> Result<ProxyConfig, String> {
    let config = state
        .config
        .lock()
        .map_err(|e| format!("获取配置失败: {}", e))?;
    
    Ok(config.proxy_config.clone())
}

/// 设置代理配置
#[tauri::command]
pub async fn set_proxy_config(
    proxy_config: ProxyConfig,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<(), String> {
    {
        let mut config = state
            .config
            .lock()
            .map_err(|e| format!("获取配置失败: {}", e))?;
        config.proxy_config = proxy_config;
    }

    // 保存配置到文件
    save_config(&state, &app)
        .await
        .map_err(|e| format!("保存配置失败: {}", e))?;

    Ok(())
}

/// 测试代理连接
#[tauri::command]
pub async fn test_proxy_connection(
    proxy_type: String,
    host: String,
    port: u16,
) -> Result<bool, String> {
    log::info!("🔍 测试代理连接: {}://{}:{}", proxy_type, host, port);
    
    let proxy_type_enum = match proxy_type.as_str() {
        "socks5" => ProxyType::Socks5,
        _ => ProxyType::Http,
    };
    
    let proxy_info = ProxyInfo::new(proxy_type_enum, host, port);
    
    let is_available = ProxyDetector::check_proxy(&proxy_info).await;
    
    if is_available {
        log::info!("✅ 代理连接测试成功");
    } else {
        log::warn!("❌ 代理连接测试失败");
    }
    
    Ok(is_available)
}

/// 自动检测可用代理
#[tauri::command]
pub async fn detect_available_proxy() -> Result<Option<ProxyInfo>, String> {
    log::info!("🔍 开始自动检测可用代理");
    
    let proxy_info = ProxyDetector::detect_available_proxy().await;
    
    if let Some(ref info) = proxy_info {
        log::info!("✅ 检测到可用代理: {}:{} ({})", info.host, info.port, info.proxy_type);
    } else {
        log::info!("ℹ️ 未检测到可用代理");
    }
    
    Ok(proxy_info)
}

