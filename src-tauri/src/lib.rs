// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::{Command, Child, Stdio};
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};


use std::io::{BufRead, BufReader};
use tauri::{Manager, Emitter, tray::{TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState}, menu::{Menu, MenuItem}};
use reqwest;
use tokio::task;

// 当前应用版本
const CURRENT_VERSION: &str = "1.2";

// 远程版本信息结构体
#[derive(Serialize, Deserialize, Debug)]
struct RemoteVersion {
    version: String,
}

// 版本检查结果
#[derive(Serialize, Deserialize, Debug)]
struct VersionCheckResult {
    current_version: String,
    latest_version: String,
    has_update: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UserInfo {
    group: Option<String>,
    token: Option<String>,
    username: Option<String>,
}

// 隧道进程信息
#[derive(Debug, Clone)]
struct TunnelProcess {
    proxy_id: i32,
    child: Arc<Mutex<Option<Child>>>,
    logs: Arc<Mutex<Vec<String>>>,
}

// 全局进程管理器
type ProcessManager = Arc<Mutex<HashMap<i32, TunnelProcess>>>;

// 删除隧道请求结构体
#[derive(Serialize, Deserialize, Debug)]
struct DeleteTunnelRequest {
    #[serde(rename = "proxyId")]
    proxy_id: i32,
}

// 编辑隧道请求结构体
#[derive(Serialize, Deserialize, Debug)]
struct UpdateTunnelRequest {
    #[serde(rename = "proxyId")]
    proxy_id: i32,
    #[serde(rename = "proxyName")]
    proxy_name: String,
    #[serde(rename = "localIp")]
    local_ip: String,
    #[serde(rename = "localPort")]
    local_port: i32,
    #[serde(rename = "remotePort")]
    remote_port: Option<i32>,
    domain: String,
    location: String,
    #[serde(rename = "accessKey")]
    access_key: String,
    #[serde(rename = "hostHeaderRewrite")]
    host_header_rewrite: String,
    #[serde(rename = "headerXFromWhere")]
    header_x_from_where: String,
    #[serde(rename = "useEncryption")]
    use_encryption: bool,
    #[serde(rename = "useCompression")]
    use_compression: bool,
    #[serde(rename = "proxyProtocolVersion")]
    proxy_protocol_version: String,
    #[serde(rename = "proxyType")]
    proxy_type: String,
    #[serde(rename = "nodeId")]
    node_id: i32,
}

// 强制下线隧道请求结构体
#[derive(Serialize, Deserialize, Debug)]
struct KickTunnelRequest {
    #[serde(rename = "proxyId")]
    proxy_id: i32,
}

// 启用/禁用隧道请求结构体
#[derive(Serialize, Deserialize, Debug)]
struct ToggleTunnelRequest {
    #[serde(rename = "proxyId")]
    proxy_id: i32,
    #[serde(rename = "isDisabled")]
    is_disabled: bool,
}

// API响应结构体
#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse<T> {
    code: i32,
    data: Option<T>,
    message: String,
}

// 登录请求结构体
#[derive(Serialize, Deserialize, Debug)]
struct LoginRequest {
    username: String,
    password: String,
}

// 登录响应数据结构体
#[derive(Serialize, Deserialize, Debug)]
struct LoginData {
    token: String,
    username: String,
    group: String,
}

// 用户详细信息结构体
#[derive(Serialize, Deserialize, Debug)]
struct UserDetailInfo {
    email: String,
    #[serde(rename = "friendlyGroup")]
    friendly_group: String,
    group: String,
    #[serde(rename = "inBound")]
    in_bound: i64,
    #[serde(rename = "isRealname")]
    is_realname: bool,
    #[serde(rename = "maxProxies")]
    max_proxies: i32,
    #[serde(rename = "outBound")]
    out_bound: i64,
    #[serde(rename = "regTime")]
    reg_time: i64,
    status: i32,
    #[serde(rename = "todaySigned")]
    today_signed: bool,
    traffic: i64,
    #[serde(rename = "usedProxies")]
    used_proxies: i32,
    #[serde(rename = "userId")]
    user_id: i32,
    username: String,
}

// 获取空闲端口请求结构体
#[derive(Serialize, Deserialize, Debug)]
struct FreePortRequest {
    #[serde(rename = "nodeId")]
    node_id: i32,
    protocol: String,
}

// 创建隧道请求结构体
#[derive(Serialize, Deserialize, Debug)]
struct CreateTunnelRequest {
    #[serde(rename = "nodeId")]
    node_id: i32,
    #[serde(rename = "proxyName")]
    proxy_name: String,
    #[serde(rename = "localIp")]
    local_ip: String,
    #[serde(rename = "localPort")]
    local_port: i32,
    #[serde(rename = "remotePort")]
    remote_port: Option<i32>,
    domain: String,
    #[serde(rename = "proxyType")]
    proxy_type: String,
    #[serde(rename = "accessKey")]
    access_key: String,
    #[serde(rename = "hostHeaderRewrite")]
    host_header_rewrite: String,
    #[serde(rename = "headerXFromWhere")]
    header_x_from_where: String,
    #[serde(rename = "proxyProtocolVersion")]
    proxy_protocol_version: String,
    #[serde(rename = "useEncryption")]
    use_encryption: bool,
    #[serde(rename = "useCompression")]
    use_compression: bool,
}

// FRP Token响应结构体
#[derive(Serialize, Deserialize, Debug)]
struct FrpTokenData {
    token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    api_status: String,
    login_time: String,
    user_token: String,
    frp_token: String,
    username: String,
    user_info: UserInfo,
}

// 应用设置结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
struct AppSettings {
    #[serde(rename = "autoStart")]
    auto_start: bool,
    #[serde(rename = "alwaysOnTop")]
    always_on_top: bool,
    #[serde(rename = "autoUpdate")]
    auto_update: bool,
    #[serde(rename = "autoStartTunnels")]
    auto_start_tunnels: Vec<i32>,
    #[serde(rename = "startupDelay")]
    startup_delay: i32,
    theme: String,
    #[serde(rename = "minimizeToTray")]
    minimize_to_tray: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            auto_start: false,
            always_on_top: false,
            auto_update: true,
            auto_start_tunnels: Vec::new(),
            startup_delay: 5,
            theme: "dark".to_string(),
            minimize_to_tray: true,
        }
    }
}

#[tauri::command]
async fn save_config(app_handle: tauri::AppHandle, config: Config) -> Result<String, String> {
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    
    // 确保目录存在
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir)
            .map_err(|e| format!("Failed to create app data dir: {}", e))?;
    }
    
    let config_path = app_data_dir.join("config.yaml");
    
    // 将配置转换为YAML格式
    let yaml_content = format!(
        "api_status: {}\nlogin_time: {}\nuser_token: {}\nfrp_token: {}\nusername: {}\nuser_info:\n  group: {}\n  token: {}\n  username: {}",
        config.api_status,
        config.login_time,
        config.user_token,
        config.frp_token,
        config.username,
        config.user_info.group.unwrap_or_default(),
        config.user_info.token.unwrap_or_default(),
        config.user_info.username.unwrap_or_default()
    );
    
    fs::write(&config_path, yaml_content)
        .map_err(|e| format!("Failed to write config file: {}", e))?;
    
    Ok(format!("Config saved to: {}", config_path.display()))
}

#[tauri::command]
async fn read_config(app_handle: tauri::AppHandle) -> Result<Option<Config>, String> {
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    
    let config_path = app_data_dir.join("config.yaml");
    
    if !config_path.exists() {
        return Ok(None);
    }
    
    let content = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;
    
    // 简单的YAML解析（这里可以使用更完善的YAML库）
    let lines: Vec<&str> = content.lines().collect();
    let mut config = Config {
        api_status: String::new(),
        login_time: String::new(),
        user_token: String::new(),
        frp_token: String::new(),
        username: String::new(),
        user_info: UserInfo {
            group: None,
            token: None,
            username: None,
        },
    };
    
    for line in lines {
        let line = line.trim();
        if line.starts_with("api_status:") {
            config.api_status = line.split(':').nth(1).unwrap_or("").trim().to_string();
        } else if line.starts_with("login_time:") {
            config.login_time = line.split(':').nth(1).unwrap_or("").trim().to_string();
        } else if line.starts_with("user_token:") {
            config.user_token = line.split(':').nth(1).unwrap_or("").trim().to_string();
        } else if line.starts_with("frp_token:") {
            config.frp_token = line.split(':').nth(1).unwrap_or("").trim().to_string();
        } else if line.starts_with("username:") && !line.contains("user_info:") {
            config.username = line.split(':').nth(1).unwrap_or("").trim().to_string();
        } else if line.starts_with("group:") {
            config.user_info.group = Some(line.split(':').nth(1).unwrap_or("").trim().to_string());
        } else if line.starts_with("token:") {
            config.user_info.token = Some(line.split(':').nth(1).unwrap_or("").trim().to_string());
        }
    }
    
    Ok(Some(config))
}

#[tauri::command]
async fn clear_config(app_handle: tauri::AppHandle) -> Result<String, String> {
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    
    let config_path = app_data_dir.join("config.yaml");
    
    if config_path.exists() {
        fs::remove_file(&config_path)
            .map_err(|e| format!("Failed to remove config file: {}", e))?;
        Ok("Config file cleared".to_string())
    } else {
        Ok("Config file does not exist".to_string())
    }
}

// 登录API命令
#[tauri::command]
async fn api_login(app_handle: tauri::AppHandle, username: String, password: String) -> Result<Config, String> {
    let client = reqwest::Client::new();
    
    let login_request = LoginRequest {
        username: username.clone(),
        password,
    };
    
    // 调用登录API
    let response = client
        .post("https://api.mefrp.com/api/public/login")
        .header("Content-Type", "application/json")
        .json(&login_request)
        .send()
        .await
        .map_err(|e| format!("登录请求失败: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("登录请求失败，状态码: {}", response.status()));
    }
    
    let api_response: ApiResponse<LoginData> = response
        .json()
        .await
        .map_err(|e| format!("解析登录响应失败: {}", e))?;
    
    if api_response.code != 200 {
        return Err(format!("登录失败: {}", api_response.message));
    }
    
    let login_data = api_response.data.ok_or("登录响应数据为空")?;
    let user_token = login_data.token;
    
    // 获取frp_token
    let frp_response = client
        .get("https://api.mefrp.com/api/auth/user/frpToken")
        .header("authorization", format!("Bearer {}", user_token))
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| format!("获取frp_token失败: {}", e))?;
    
    let mut frp_token = String::new();
    if frp_response.status().is_success() {
        let frp_api_response: ApiResponse<FrpTokenData> = frp_response
            .json()
            .await
            .map_err(|e| format!("解析frp_token响应失败: {}", e))?;
        
        if frp_api_response.code == 200 {
            if let Some(frp_data) = frp_api_response.data {
                frp_token = frp_data.token;
            }
        }
    }
    
    // 创建配置
    let config = Config {
        api_status: "connected".to_string(),
        login_time: chrono::Utc::now().to_rfc3339(),
        user_token: user_token.clone(),
        frp_token,
        username: username.clone(),
        user_info: UserInfo {
            group: Some(login_data.group),
            token: Some(user_token),
            username: Some(login_data.username),
        },
    };
    
    // 保存配置
    save_config(app_handle, config.clone()).await?;
    
    Ok(config)
}

// 获取用户信息API命令
#[tauri::command]
async fn api_get_user_info(app_handle: tauri::AppHandle) -> Result<UserDetailInfo, String> {
    // 读取配置获取token
    let config = read_config(app_handle).await?
        .ok_or("未找到配置文件")?;
    
    if config.user_token.is_empty() {
        return Err("未找到有效的token".to_string());
    }
    
    let client = reqwest::Client::new();
    
    let response = client
        .get("https://api.mefrp.com/api/auth/user/info")
        .header("authorization", format!("Bearer {}", config.user_token))
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| format!("获取用户信息请求失败: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("获取用户信息失败，状态码: {}", response.status()));
    }
    
    let api_response: ApiResponse<UserDetailInfo> = response
        .json()
        .await
        .map_err(|e| format!("解析用户信息响应失败: {}", e))?;
    
    if api_response.code != 200 {
        return Err(format!("获取用户信息失败: {}", api_response.message));
    }
    
    api_response.data.ok_or("用户信息响应数据为空".to_string())
}

// 获取系统公告API命令
#[tauri::command]
async fn api_get_announcements(app_handle: tauri::AppHandle) -> Result<String, String> {
    // 读取配置获取token
    let config = read_config(app_handle).await?
        .ok_or("未找到配置文件")?;
    
    if config.user_token.is_empty() {
        return Err("未找到有效的token".to_string());
    }
    
    let client = reqwest::Client::new();
    
    let response = client
        .get("https://api.mefrp.com/api/auth/notice")
        .header("authorization", format!("Bearer {}", config.user_token))
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| format!("获取系统公告请求失败: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("获取系统公告失败，状态码: {}", response.status()));
    }
    
    // 尝试解析为字符串响应
    let api_response: ApiResponse<String> = response
        .json()
        .await
        .map_err(|e| format!("解析系统公告响应失败: {}", e))?;
    
    if api_response.code != 200 {
        return Err(format!("获取系统公告失败: {}", api_response.message));
    }
    
    Ok(api_response.data.unwrap_or_default())
}

// 获取节点列表API命令
#[tauri::command]
async fn api_get_node_list(app_handle: tauri::AppHandle) -> Result<String, String> {
    // 读取配置获取token
    let config = read_config(app_handle).await?
        .ok_or("未找到配置文件")?;
    
    if config.user_token.is_empty() {
        return Err("未找到有效的token".to_string());
    }
    
    let client = reqwest::Client::new();
    
    let response = client
        .get("https://api.mefrp.com/api/auth/node/list")
        .header("authorization", format!("Bearer {}", config.user_token))
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| format!("获取节点列表请求失败: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("获取节点列表失败，状态码: {}", response.status()));
    }
    
    let response_text = response
        .text()
        .await
        .map_err(|e| format!("解析节点列表响应失败: {}", e))?;
    
    Ok(response_text)
}

// 编辑隧道API命令
#[tauri::command]
async fn api_update_tunnel(app_handle: tauri::AppHandle, data: String) -> Result<String, String> {
    let config = read_config(app_handle).await?
        .ok_or("未找到配置文件")?;

    if config.user_token.is_empty() {
        return Err("未找到有效的token".to_string());
    }

    let request_data: UpdateTunnelRequest = serde_json::from_str(&data)
        .map_err(|e| format!("解析请求数据失败: {}", e))?;

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.mefrp.com/api/auth/proxy/update")
        .header("authorization", format!("Bearer {}", config.user_token))
        .header("Content-Type", "application/json")
        .json(&request_data)
        .send()
        .await
        .map_err(|e| format!("编辑隧道请求失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("编辑隧道失败，状态码: {}", response.status()));
    }

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("解析编辑隧道响应失败: {}", e))?;

    Ok(response_text)
}

// 强制下线隧道API命令
#[tauri::command]
async fn api_kick_tunnel(app_handle: tauri::AppHandle, proxy_id: i32) -> Result<String, String> {
    let config = read_config(app_handle).await?
        .ok_or("未找到配置文件")?;

    if config.user_token.is_empty() {
        return Err("未找到有效的token".to_string());
    }

    let kick_request = KickTunnelRequest {
        proxy_id,
    };

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.mefrp.com/api/auth/proxy/kick")
        .header("authorization", format!("Bearer {}", config.user_token))
        .header("Content-Type", "application/json")
        .json(&kick_request)
        .send()
        .await
        .map_err(|e| format!("强制下线隧道请求失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("强制下线隧道失败，状态码: {}", response.status()));
    }

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("解析强制下线隧道响应失败: {}", e))?;

    Ok(response_text)
}

// 启用/禁用隧道API命令
#[tauri::command]
async fn api_toggle_tunnel(app_handle: tauri::AppHandle, proxy_id: i32, is_disabled: bool) -> Result<String, String> {
    let config = read_config(app_handle).await?
        .ok_or("未找到配置文件")?;

    if config.user_token.is_empty() {
        return Err("未找到有效的token".to_string());
    }

    let toggle_request = ToggleTunnelRequest {
        proxy_id,
        is_disabled,
    };

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.mefrp.com/api/auth/proxy/toggle")
        .header("authorization", format!("Bearer {}", config.user_token))
        .header("Content-Type", "application/json")
        .json(&toggle_request)
        .send()
        .await
        .map_err(|e| format!("切换隧道状态请求失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("切换隧道状态失败，状态码: {}", response.status()));
    }

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("解析切换隧道状态响应失败: {}", e))?;

    Ok(response_text)
}

// 获取节点简要信息API命令
#[tauri::command]
async fn api_get_node_name_list(app_handle: tauri::AppHandle) -> Result<String, String> {
    let config = read_config(app_handle).await?
        .ok_or("未找到配置文件")?;

    if config.user_token.is_empty() {
        return Err("未找到有效的token".to_string());
    }

    let client = reqwest::Client::new();
    let response = client
        .get("https://api.mefrp.com/api/auth/node/nameList")
        .header("authorization", format!("Bearer {}", config.user_token))
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| format!("获取节点简要信息请求失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("获取节点简要信息失败，状态码: {}", response.status()));
    }

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("解析节点简要信息响应失败: {}", e))?;

    Ok(response_text)
}

// 获取隧道日志
#[tauri::command]
async fn api_get_tunnel_logs(proxy_id: i32, process_manager: tauri::State<'_, ProcessManager>) -> Result<Vec<String>, String> {
    let manager = process_manager.lock().map_err(|e| format!("获取进程管理器锁失败: {}", e))?;
    
    match manager.get(&proxy_id) {
        Some(process) => {
            let logs = process.logs.lock().map_err(|e| format!("获取日志锁失败: {}", e))?;
            Ok(logs.clone())
        }
        None => Err("未找到运行中的隧道进程".to_string())
    }
}

// 获取所有运行中的隧道
#[tauri::command]
async fn api_get_running_tunnels(process_manager: tauri::State<'_, ProcessManager>) -> Result<Vec<i32>, String> {
    let manager = process_manager.lock().map_err(|e| format!("获取进程管理器锁失败: {}", e))?;
    Ok(manager.keys().cloned().collect())
}

// 获取节点状态API命令
#[tauri::command]
async fn api_get_node_status(app_handle: tauri::AppHandle) -> Result<String, String> {
    // 读取配置获取token
    let config = read_config(app_handle).await?
        .ok_or("未找到配置文件")?;
    
    if config.user_token.is_empty() {
        return Err("未找到有效的token".to_string());
    }
    
    let client = reqwest::Client::new();
    
    let response = client
        .get("https://api.mefrp.com/api/auth/node/status")
        .header("authorization", format!("Bearer {}", config.user_token))
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| format!("获取节点状态请求失败: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("获取节点状态失败，状态码: {}", response.status()));
    }
    
    let response_text = response
        .text()
        .await
        .map_err(|e| format!("解析节点状态响应失败: {}", e))?;
    
    Ok(response_text)
}

// 获取空闲端口API命令
#[tauri::command]
async fn api_get_free_port(app_handle: tauri::AppHandle, data: String) -> Result<String, String> {
    let config = read_config(app_handle).await?
        .ok_or("未找到配置文件")?;

    if config.user_token.is_empty() {
        return Err("未找到有效的token".to_string());
    }

    let request_data: FreePortRequest = serde_json::from_str(&data)
        .map_err(|e| format!("解析请求数据失败: {}", e))?;

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.mefrp.com/api/auth/node/freePort")
        .header("authorization", format!("Bearer {}", config.user_token))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "nodeId": request_data.node_id,
            "protocol": request_data.protocol
        }))
        .send()
        .await
        .map_err(|e| format!("获取空闲端口请求失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("获取空闲端口失败，状态码: {}", response.status()));
    }

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("解析空闲端口响应失败: {}", e))?;

    Ok(response_text)
}

// 创建隧道API命令
#[tauri::command]
async fn api_create_tunnel(app_handle: tauri::AppHandle, data: String) -> Result<String, String> {
    let config = read_config(app_handle).await?
        .ok_or("未找到配置文件")?;

    if config.user_token.is_empty() {
        return Err("未找到有效的token".to_string());
    }

    let request_data: CreateTunnelRequest = serde_json::from_str(&data)
        .map_err(|e| format!("解析请求数据失败: {}", e))?;

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.mefrp.com/api/auth/proxy/create")
        .header("authorization", format!("Bearer {}", config.user_token))
        .header("Content-Type", "application/json")
        .json(&request_data)
        .send()
        .await
        .map_err(|e| format!("创建隧道请求失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("创建隧道失败，状态码: {}", response.status()));
    }

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("解析创建隧道响应失败: {}", e))?;

    Ok(response_text)
}

// 获取隧道列表API命令
#[tauri::command]
async fn api_get_tunnel_list(app_handle: tauri::AppHandle) -> Result<String, String> {
    let config = read_config(app_handle).await?
        .ok_or("未找到配置文件")?;

    if config.user_token.is_empty() {
        return Err("未找到有效的token".to_string());
    }

    let client = reqwest::Client::new();
    let response = client
        .get("https://api.mefrp.com/api/auth/proxy/list")
        .header("authorization", format!("Bearer {}", config.user_token))
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| format!("获取隧道列表请求失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("获取隧道列表失败，状态码: {}", response.status()));
    }

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("解析隧道列表响应失败: {}", e))?;

    Ok(response_text)
}

// 启动隧道命令（使用mefrpc.exe）
#[tauri::command]
async fn api_start_tunnel(app_handle: tauri::AppHandle, proxy_id: i32, process_manager: tauri::State<'_, ProcessManager>) -> Result<String, String> {
    let config = read_config(app_handle.clone()).await?
        .ok_or("未找到配置文件")?;

    if config.frp_token.is_empty() {
        return Err("未找到有效的frp_token".to_string());
    }

    // 检查是否已经有进程在运行
    {
        let manager = process_manager.lock().map_err(|e| format!("获取进程管理器锁失败: {}", e))?;
        if manager.contains_key(&proxy_id) {
            return Err("隧道已经在运行中".to_string());
        }
    }

    // 获取可执行文件同目录下bin文件夹中的mefrpc可执行文件路径
    let exe_path = std::env::current_exe()
        .map_err(|e| format!("获取当前可执行文件路径失败: {}", e))?;
    let exe_dir = exe_path.parent()
        .ok_or("无法获取可执行文件目录")?;
    
    let mefrpc_path = exe_dir.join("bin").join("mefrpc-x86_64-pc-windows-msvc.exe");

    if !mefrpc_path.exists() {
        return Err(format!("mefrpc.exe 不存在: {}", mefrpc_path.display()));
    }

    // 启动mefrpc进程
    let mut command = Command::new(&mefrpc_path);
    command
        .arg("-t")
        .arg(&config.frp_token)
        .arg("-p")
        .arg(proxy_id.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    
    // 在Windows上隐藏命令行窗口
    #[cfg(windows)]
    command.creation_flags(0x08000000); // CREATE_NO_WINDOW
    
    let mut child = command
        .spawn()
        .map_err(|e| format!("启动mefrpc进程失败: {}", e))?;

    // 创建日志存储
    let logs = Arc::new(Mutex::new(Vec::new()));
    let logs_clone = logs.clone();

    // 获取进程的 stdout 和 stderr
    let stdout = child.stdout.take().ok_or("无法获取进程stdout")?;
    let stderr = child.stderr.take().ok_or("无法获取进程stderr")?;

    // 启动异步任务读取 stdout
    let logs_stdout = logs.clone();
    task::spawn(async move {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                if let Ok(mut logs) = logs_stdout.lock() {
                    logs.push(format!("[OUT] {}", line));
                    // 限制日志数量，避免内存溢出
                    if logs.len() > 1000 {
                        logs.remove(0);
                    }
                }
            }
        }
    });

    // 启动异步任务读取 stderr
    let logs_stderr = logs.clone();
    task::spawn(async move {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(line) = line {
                if let Ok(mut logs) = logs_stderr.lock() {
                    logs.push(format!("[ERR] {}", line));
                    // 限制日志数量，避免内存溢出
                    if logs.len() > 1000 {
                        logs.remove(0);
                    }
                }
            }
        }
    });

    // 创建隧道进程信息
    let tunnel_process = TunnelProcess {
        proxy_id,
        child: Arc::new(Mutex::new(Some(child))),
        logs: logs_clone,
    };

    // 将进程添加到管理器
    {
        let mut manager = process_manager.lock().map_err(|e| format!("获取进程管理器锁失败: {}", e))?;
        manager.insert(proxy_id, tunnel_process);
    }

    Ok(format!("{{\"code\": 200, \"message\": \"隧道启动成功\", \"data\": {{\"proxyId\": {}}}}}", proxy_id))
}

// 停止隧道命令（kill mefrpc进程）
#[tauri::command]
async fn api_stop_tunnel(_app_handle: tauri::AppHandle, proxy_id: i32, process_manager: tauri::State<'_, ProcessManager>) -> Result<String, String> {
    // 从进程管理器中获取并移除进程
    let tunnel_process = {
        let mut manager = process_manager.lock().map_err(|e| format!("获取进程管理器锁失败: {}", e))?;
        manager.remove(&proxy_id)
    };

    match tunnel_process {
        Some(process) => {
            // 尝试终止进程
            let mut child_guard = process.child.lock().map_err(|e| format!("获取进程锁失败: {}", e))?;
            if let Some(mut child) = child_guard.take() {
                match child.kill() {
                    Ok(_) => {
                        // 等待进程结束
                        let _ = child.wait();
                        Ok(format!("{{\"code\": 200, \"message\": \"隧道停止成功\", \"data\": {{\"proxyId\": {}}}}}", proxy_id))
                    }
                    Err(e) => Err(format!("终止进程失败: {}", e))
                }
            } else {
                Err("进程已经被终止".to_string())
            }
        }
        None => Err("未找到运行中的隧道进程".to_string())
    }
}

// 删除隧道API命令
#[tauri::command]
async fn api_delete_tunnel(app_handle: tauri::AppHandle, proxy_id: i32) -> Result<String, String> {
    let config = read_config(app_handle).await?
        .ok_or("未找到配置文件")?;

    if config.user_token.is_empty() {
        return Err("未找到有效的token".to_string());
    }

    let delete_request = DeleteTunnelRequest {
        proxy_id,
    };

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.mefrp.com/api/auth/proxy/delete")
        .header("authorization", format!("Bearer {}", config.user_token))
        .header("Content-Type", "application/json")
        .json(&delete_request)
        .send()
        .await
        .map_err(|e| format!("删除隧道请求失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("删除隧道失败，状态码: {}", response.status()));
    }

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("解析删除隧道响应失败: {}", e))?;

    Ok(response_text)
}

// 保存应用设置
#[tauri::command]
async fn save_settings(app_handle: tauri::AppHandle, settings: AppSettings) -> Result<String, String> {
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("获取应用数据目录失败: {}", e))?;
    
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir)
            .map_err(|e| format!("创建应用数据目录失败: {}", e))?;
    }
    
    let settings_path = app_data_dir.join("settings.json");
    let settings_json = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("序列化设置失败: {}", e))?;
    
    fs::write(&settings_path, settings_json)
        .map_err(|e| format!("写入设置文件失败: {}", e))?;
    
    Ok("设置保存成功".to_string())
}

// 加载应用设置
#[tauri::command]
async fn load_settings(app_handle: tauri::AppHandle) -> Result<AppSettings, String> {
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("获取应用数据目录失败: {}", e))?;
    
    let settings_path = app_data_dir.join("settings.json");
    
    if !settings_path.exists() {
        return Ok(AppSettings::default());
    }
    
    let settings_content = fs::read_to_string(&settings_path)
        .map_err(|e| format!("读取设置文件失败: {}", e))?;
    
    let settings: AppSettings = serde_json::from_str(&settings_content)
        .unwrap_or_else(|_| AppSettings::default());
    
    Ok(settings)
}

// 设置开机自启动
#[tauri::command]
async fn set_auto_start(enable: bool) -> Result<String, String> {
    // 这里需要根据操作系统实现开机自启动
    // Windows: 注册表或启动文件夹
    // macOS: LaunchAgents
    // Linux: systemd 或 autostart
    
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        
        if enable {
            // 添加到注册表启动项
            let output = Command::new("reg")
                .args([
                    "add",
                    "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                    "/v",
                    "ME-Frp",
                    "/t",
                    "REG_SZ",
                    "/d",
                    &format!("\"{}\"", std::env::current_exe().unwrap().display()),
                    "/f"
                ])
                .output()
                .map_err(|e| format!("执行注册表命令失败: {}", e))?;
            
            if !output.status.success() {
                return Err("设置开机自启动失败".to_string());
            }
        } else {
            // 从注册表删除启动项
            let _output = Command::new("reg")
                .args([
                    "delete",
                    "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                    "/v",
                    "ME-Frp",
                    "/f"
                ])
                .output()
                .map_err(|e| format!("执行注册表命令失败: {}", e))?;
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        // 其他操作系统的实现
        return Err("当前操作系统暂不支持开机自启动设置".to_string());
    }
    
    Ok(if enable { "开机自启动已开启" } else { "开机自启动已关闭" }.to_string())
}

// 设置窗口置顶
#[tauri::command]
async fn set_always_on_top(app_handle: tauri::AppHandle, always_on_top: bool) -> Result<String, String> {
    let window = app_handle.get_webview_window("main")
        .ok_or("未找到主窗口")?;
    
    window.set_always_on_top(always_on_top)
        .map_err(|e| format!("设置窗口置顶失败: {}", e))?;
    
    Ok(if always_on_top { "窗口置顶已开启" } else { "窗口置顶已关闭" }.to_string())
}

// 显示窗口
#[tauri::command]
async fn show_window(app_handle: tauri::AppHandle) -> Result<String, String> {
    let window = app_handle.get_webview_window("main")
        .ok_or("未找到主窗口")?;
    
    window.show().map_err(|e| format!("显示窗口失败: {}", e))?;
    window.set_focus().map_err(|e| format!("设置窗口焦点失败: {}", e))?;
    
    Ok("窗口已显示".to_string())
}

// 隐藏窗口
#[tauri::command]
async fn hide_window(app_handle: tauri::AppHandle) -> Result<String, String> {
    let window = app_handle.get_webview_window("main")
        .ok_or("未找到主窗口")?;
    
    window.hide().map_err(|e| format!("隐藏窗口失败: {}", e))?;
    
    Ok("窗口已隐藏".to_string())
}

// 设置最小化到托盘行为
#[tauri::command]
async fn set_minimize_to_tray(app_handle: tauri::AppHandle, minimize_to_tray: bool) -> Result<String, String> {
    // 保存设置到应用状态中
    app_handle.state::<Arc<Mutex<bool>>>().lock().map_err(|e| format!("获取状态锁失败: {}", e))?
        .clone_from(&minimize_to_tray);
    
    Ok(if minimize_to_tray { "最小化到托盘已开启" } else { "最小化到托盘已关闭" }.to_string())
}

// 退出应用
#[tauri::command]
async fn quit_app(app_handle: tauri::AppHandle, process_manager: tauri::State<'_, ProcessManager>) -> Result<String, String> {
    // 先停止所有正在运行的隧道
    let manager = process_manager.lock().map_err(|e| format!("获取进程管理器锁失败: {}", e))?;
    
    // 收集所有正在运行的隧道ID
    let running_tunnels: Vec<i32> = manager.keys().cloned().collect();
    drop(manager); // 释放锁
    
    // 逐个停止隧道
    for proxy_id in running_tunnels {
        if let Ok(mut manager) = process_manager.lock() {
            if let Some(tunnel_process) = manager.get_mut(&proxy_id) {
                if let Ok(mut child_opt) = tunnel_process.child.lock() {
                    if let Some(ref mut child) = *child_opt {
                        let _ = child.kill(); // 强制终止进程
                        let _ = child.wait(); // 等待进程结束
                    }
                }
            }
            manager.remove(&proxy_id); // 从管理器中移除
        }
    }
    
    app_handle.exit(0);
    Ok("应用已退出".to_string())
}

// 检查版本更新
#[tauri::command]
async fn check_for_updates() -> Result<VersionCheckResult, String> {
    let client = reqwest::Client::new();
    
    // 请求远程版本信息
    let response = client
        .get("https://alist.yealqp.cn/d/mefrp-desktop/latest/version.json?sign=F3lfDhDXLsJUshAXmzFuY7QfQD0T3dCJY4Jxr1Y9eOg=:0")
        .send()
        .await
        .map_err(|e| format!("请求版本信息失败: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("获取版本信息失败，状态码: {}", response.status()));
    }
    
    let remote_version: RemoteVersion = response
        .json()
        .await
        .map_err(|e| format!("解析版本信息失败: {}", e))?;
    
    let current_version = CURRENT_VERSION.to_string();
    let latest_version = remote_version.version;
    
    // 比较版本号
    let has_update = compare_versions(&current_version, &latest_version);
    
    Ok(VersionCheckResult {
        current_version,
        latest_version,
        has_update,
    })
}

// 版本号比较函数
fn compare_versions(current: &str, latest: &str) -> bool {
    // 简单的版本号比较，假设格式为 x.y.z
    let current_parts: Vec<u32> = current
        .split('.')
        .filter_map(|s| s.parse().ok())
        .collect();
    
    let latest_parts: Vec<u32> = latest
        .split('.')
        .filter_map(|s| s.parse().ok())
        .collect();
    
    // 确保两个版本号都有至少3个部分
    if current_parts.len() < 3 || latest_parts.len() < 3 {
        return current != latest;
    }
    
    // 比较主版本号
    if latest_parts[0] > current_parts[0] {
        return true;
    } else if latest_parts[0] < current_parts[0] {
        return false;
    }
    
    // 比较次版本号
    if latest_parts[1] > current_parts[1] {
        return true;
    } else if latest_parts[1] < current_parts[1] {
        return false;
    }
    
    // 比较修订版本号
    latest_parts[2] > current_parts[2]
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let process_manager: ProcessManager = Arc::new(Mutex::new(HashMap::new()));
    let minimize_to_tray_state: Arc<Mutex<bool>> = Arc::new(Mutex::new(true)); // 默认开启最小化到托盘
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // 当检测到已有实例运行时，显示现有窗口
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
                let _ = window.unminimize();
            }
        }))
        .setup(|app| {
            // 创建系统托盘菜单
            let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit])?;
            
            // 创建系统托盘
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("ME-Frp 非官方客户端")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(move |app, event| {
                    match event.id().as_ref() {
                        "quit" => {
                            // 调用quit_app命令来确保停止所有隧道进程
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.emit("quit-app", ());
                            }
                            // 延迟退出以确保隧道进程有时间停止
                            std::thread::spawn(move || {
                                std::thread::sleep(std::time::Duration::from_millis(500));
                                std::process::exit(0);
                            });
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    match event {
                        TrayIconEvent::Click {
                            button: MouseButton::Left,
                            button_state: MouseButtonState::Up,
                            ..
                        } => {
                            if let Some(app) = tray.app_handle().get_webview_window("main") {
                                if app.is_visible().unwrap_or(false) {
                                    let _ = app.hide();
                                } else {
                                    let _ = app.show();
                                    let _ = app.set_focus();
                                }
                            }
                        }
                        TrayIconEvent::DoubleClick { .. } => {
                            if let Some(app) = tray.app_handle().get_webview_window("main") {
                                let _ = app.show();
                                let _ = app.set_focus();
                            }
                        }
                        _ => {}
                    }
                })
                .build(app)?;
            
            Ok(())
        })
        .on_window_event(|window, event| {
            match event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    // 获取最小化到托盘设置
                    let minimize_to_tray = window.app_handle()
                        .state::<Arc<Mutex<bool>>>()
                        .lock()
                        .map(|state| *state)
                        .unwrap_or(true);
                    
                    if minimize_to_tray {
                        // 阻止默认的关闭行为并隐藏窗口到系统托盘
                        api.prevent_close();
                        let _ = window.hide();
                    } else {
                        // 阻止默认关闭行为，先停止所有隧道进程
                        api.prevent_close();
                        
                        // 获取进程管理器并停止所有隧道
                        let app_handle = window.app_handle();
                        let process_manager = app_handle.state::<ProcessManager>();
                        
                        // 克隆必要的数据以避免生命周期问题
                        let app_handle_clone = app_handle.clone();
                        let process_manager_clone = process_manager.inner().clone();
                        
                        // 在新线程中处理隧道停止和应用退出
                        std::thread::spawn(move || {
                            // 停止所有正在运行的隧道
                            if let Ok(manager) = process_manager_clone.lock() {
                                let running_tunnels: Vec<i32> = manager.keys().cloned().collect();
                                drop(manager); // 释放锁
                                
                                // 逐个停止隧道
                                for proxy_id in running_tunnels {
                                    if let Ok(mut manager) = process_manager_clone.lock() {
                                        if let Some(tunnel_process) = manager.get_mut(&proxy_id) {
                                            if let Ok(mut child_opt) = tunnel_process.child.lock() {
                                                if let Some(ref mut child) = *child_opt {
                                                    let _ = child.kill(); // 强制终止进程
                                                    let _ = child.wait(); // 等待进程结束
                                                }
                                            }
                                        }
                                        manager.remove(&proxy_id); // 从管理器中移除
                                    }
                                }
                            }
                            
                            // 等待一小段时间确保所有进程都已停止
                            std::thread::sleep(std::time::Duration::from_millis(200));
                            
                            // 退出应用
                            app_handle_clone.exit(0);
                        });
                    }
                }
                _ => {}
            }
        })
        .manage(process_manager)
        .manage(minimize_to_tray_state)
        .invoke_handler(tauri::generate_handler![
            save_config, 
            read_config, 
            clear_config,
            api_login,
            api_get_user_info,
            api_get_announcements,
            api_get_node_list,
            api_get_node_status,
            api_get_free_port,
            api_create_tunnel,
            api_get_tunnel_list,
            api_start_tunnel,
            api_stop_tunnel,
            api_delete_tunnel,
            api_update_tunnel,
            api_kick_tunnel,
            api_toggle_tunnel,
            api_get_node_name_list,
            api_get_tunnel_logs,
            api_get_running_tunnels,
            save_settings,
            load_settings,
            set_auto_start,
            set_always_on_top,
            show_window,
            hide_window,
            set_minimize_to_tray,
            quit_app,
            check_for_updates
         ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
