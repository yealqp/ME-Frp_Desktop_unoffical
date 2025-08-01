<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { darkTheme, NDialogProvider, createDiscreteApi } from 'naive-ui'
import Sidebar from './components/Sidebar.vue'
import Dashboard from './components/Dashboard.vue'
import CreateTunnel from './components/CreateTunnel.vue'
import TunnelConfig from './components/TunnelConfig.vue'
import TunnelManagement from './components/TunnelManagement.vue'
import Settings from './components/Settings.vue'
import About from './components/About.vue'
import HelpCenter from './components/HelpCenter.vue'
import Login from './components/Login.vue'

interface Tunnel {
  id: number;
  name: string;
  type: string;
  status: string;
  port: number;
}

interface TunnelForm {
  name: string;
  type: string;
  localPort: number | null;
  remotePort: number | null;
}

interface Node {
  nodeId: number;
  name: string;
  hostname: string;
  description: string;
  token: string;
  servicePort: number;
  adminPort: number;
  adminPass: string;
  allowGroup: string;
  allowPort: string;
  allowType: string;
  region: string;
  bandwidth: string;
  isOnline: boolean;
  isDisabled: boolean;
  totalTrafficIn: number;
  totalTrafficOut: number;
  upTime: number;
  version: string;
}

interface UpdateCheckResult {
  has_update: boolean;
  latest_version: string;
  current_version: string;
}

// 自定义主题配置
const customTheme = {
  ...darkTheme,
  common: {
    ...darkTheme.common,
    bodyColor: '#101014',
    cardColor: '#18181c',
    modalColor: '#18181c',
    popoverColor: '#18181c',
    tableHeaderColor: '#18181c',
    inputColor: '#303033',
    inputColorDisabled: '#303033',
    primaryColor: '#349ff4',
    primaryColorHover: '#4da8f5',
    primaryColorPressed: '#2891f3',
    borderColor: '#29292c',
    dividerColor: '#29292c'
  }
}

// 登录状态管理
const isLoggedIn = ref(false)
const isCheckingAuth = ref(true)

// 消息和对话框 - 使用 createDiscreteApi
const { message, dialog } = createDiscreteApi(['message', 'dialog'], {
  configProviderProps: {
    theme: customTheme
  }
})

// 当前激活的导航项
const activeNav = ref('dashboard');

// 页面状态管理
const currentPage = ref('node-selection'); // 'node-selection' | 'tunnel-config'
const selectedNode = ref<Node | null>(null);



// 隧道数据
const tunnelData = ref<Tunnel[]>([
  { id: 1, name: 'Web服务', type: 'HTTP', status: '运行中', port: 8080 },
  { id: 2, name: 'SSH隧道', type: 'TCP', status: '已停止', port: 22 }
]);



// 切换导航
function handleNavChange(navId: string) {
  activeNav.value = navId;
  // 重置页面状态
  if (navId === 'create-tunnel') {
    currentPage.value = 'node-selection';
    selectedNode.value = null;
  }
}

// 节点选择完成，进入隧道配置页面
function handleNodeSelected(node: Node) {
  console.log('App.vue: 接收到节点选择事件', node);
  console.log('App.vue: 当前页面状态', currentPage.value);
  selectedNode.value = node;
  currentPage.value = 'tunnel-config';
  console.log('App.vue: 切换到隧道配置页面', currentPage.value);
}

// 返回节点选择页面
function handleGoBackToNodeSelection() {
  currentPage.value = 'node-selection';
  selectedNode.value = null;
}

// 创建隧道
function handleTunnelCreated(tunnel: TunnelForm) {
  const newTunnel: Tunnel = {
    id: Date.now(),
    name: tunnel.name,
    type: tunnel.type.toUpperCase(),
    status: '已停止',
    port: tunnel.localPort || 0
  };
  tunnelData.value.push(newTunnel);
}

// 启动隧道
function handleTunnelStart(id: number) {
  const tunnel = tunnelData.value.find(t => t.id === id);
  if (tunnel) {
    tunnel.status = '运行中';
  }
}

// 停止隧道
function handleTunnelStop(id: number) {
  const tunnel = tunnelData.value.find(t => t.id === id);
  if (tunnel) {
    tunnel.status = '已停止';
  }
}

// 编辑隧道
function handleTunnelEdit(id: number) {
  alert(`编辑隧道 ID: ${id}`);
}

// 删除隧道
function handleTunnelDelete(id: number) {
  const index = tunnelData.value.findIndex(t => t.id === id);
  if (index > -1) {
    tunnelData.value.splice(index, 1);
  }
}

// 刷新隧道列表
function handleRefreshTunnels() {
  // TODO: 实现从API获取隧道列表的逻辑
  console.log('刷新隧道列表');
}

// 跳转到创建隧道页面
function handleGoToCreateTunnel() {
  activeNav.value = 'create-tunnel';
  currentPage.value = 'node-selection';
  selectedNode.value = null;
}

// 配置相关函数
const checkAuthStatus = async (): Promise<void> => {
  try {
    // 首先尝试从Rust后端读取配置
    let config = null
    try {
      const result = await invoke('read_config')
      if (result) {
        config = result
        console.log('从config.yaml读取配置:', config)
      }
    } catch (error) {
      console.log('读取config.yaml失败，尝试从localStorage读取:', error)
    }
    
    // 如果后端读取失败，尝试从localStorage读取
    if (!config) {
      const configStr = localStorage.getItem('mefrp_config')
      if (configStr) {
        config = JSON.parse(configStr)
        console.log('从localStorage读取配置:', config)
      }
    }
    
    if (config) {
      // 检查是否有API连接状态或有效的user_token
      if (config.api_status === 'connected' || config.user_token) {
        isLoggedIn.value = true
      }
    }
  } catch (error) {
    console.error('检查登录状态失败:', error)
  } finally {
    isCheckingAuth.value = false
  }
}

const handleLoginSuccess = (): void => {
  console.log('收到登录成功事件，设置登录状态为true')
  isLoggedIn.value = true
  console.log('当前登录状态:', isLoggedIn.value)
}

const handleLogout = async (): Promise<void> => {
  isLoggedIn.value = false
  // 清除本地存储的配置
  localStorage.removeItem('mefrp_config')
  
  // 同时清除Rust后端的配置文件
  try {
    await invoke('clear_config')
    console.log('已清除后端配置文件')
  } catch (error) {
    console.error('清除后端配置文件失败:', error)
  }
}

// 启动时自动检查更新
const autoCheckForUpdates = async () => {
  try {
    // 延迟5秒后检查更新，确保应用完全启动
    setTimeout(async () => {
      try {
        const result = await invoke('check_for_updates') as UpdateCheckResult;
        if (result.has_update) {
          // 弹窗询问用户是否要更新
          dialog.warning({
            title: '发现新版本',
            content: `发现新版本 ${result.latest_version}，当前版本 ${result.current_version}。是否要立即更新？`,
            positiveText: '立即更新',
            negativeText: '稍后提醒',
            onPositiveClick: () => {
              message.info('正在准备更新...');
              // 可以打开下载页面或执行更新程序
              window.open('https://github.com/your-repo/releases', '_blank');
            },
            onNegativeClick: () => {
              message.info('已取消更新，下次启动时会再次检查');
            }
          });
        }
      } catch (error) {
        console.log('自动检查更新失败:', error);
        // 静默失败，不显示错误消息
      }
    }, 5000);
  } catch (error) {
    console.log('启动自动更新检查失败:', error);
  }
};

// 组件挂载时检查登录状态
onMounted(async () => {

console.log(`     __  _________   ______                  ___          __  __           __          
    /  |/  / ____/  / ____/________         ( _ )         \\ \\/ /__  ____ _/ /___ _____ 
   / /|_/ / __/    / /_  / ___/ __ \\       / __ \\/|        \\  / _ \/ __ \`\/ / __\` / __ \\ 
  / /  / / /___   / __/ / /  / /_/ /      / /_/  <         / /  __/ /_/ / / /_/ / /_/ / 
 /_/  /_/_____/  /_/   /_/  / .___/       \\____/\\/        /_/\\___/\\__,_/_/\\__, / .___/ 
                           /_/                                              /_/_/      `);

    // 监听系统托盘退出事件
    await listen('quit-app', async () => {
      try {
        await invoke('quit_app');
      } catch (error) {
        console.error('退出应用失败:', error);
      }
    });

    checkAuthStatus();
    autoCheckForUpdates();
  })
</script>

<template>
  <div class="app-container">
    <n-config-provider :theme="customTheme">
      <n-message-provider>
        <n-dialog-provider>
        <!-- 加载状态 -->
        <div v-if="isCheckingAuth" class="loading-container">
          <div class="loading-spinner">
            <i class="fas fa-spinner fa-spin"></i>
            <p>正在检查登录状态...</p>
          </div>
        </div>
        
        <!-- 登录页面 -->
        <div v-else-if="!isLoggedIn" class="login-fullscreen">
          <Login 
            @login-success="handleLoginSuccess" 
          />
        </div>
        
        <!-- 主应用界面 -->
        <template v-else>
      <!-- 左侧导航栏组件 -->
      <Sidebar 
        :active-nav="activeNav" 
        @nav-change="handleNavChange"
        @logout="handleLogout" 
      />

          <!-- 左侧导航栏组件 -->
          <Sidebar 
            :active-nav="activeNav" 
            @nav-change="handleNavChange"
            @logout="handleLogout" 
          />

          <!-- 右侧内容区域 -->
          <main class="main-content">
            <div class="content-body">
            <!-- 面板首页 -->
            <Dashboard v-if="activeNav === 'dashboard'" :tunnel-data="tunnelData" />

            <!-- 创建隧道 -->
            <template v-else-if="activeNav === 'create-tunnel'">
              <!-- 节点选择页面 -->
              <CreateTunnel v-if="currentPage === 'node-selection'" 
                @tunnel-created="handleTunnelCreated" 
                @node-selected="handleNodeSelected" />
              
              <!-- 隧道配置页面 -->
              <TunnelConfig v-else-if="currentPage === 'tunnel-config' && selectedNode" 
                :selected-node="selectedNode"
                @go-back="handleGoBackToNodeSelection"
                @tunnel-created="handleTunnelCreated" />
            </template>

            <!-- 隧道管理 -->
            <TunnelManagement v-else-if="activeNav === 'tunnel-management'" 
                :tunnel-data="tunnelData"
                @tunnel-start="handleTunnelStart"
                @tunnel-stop="handleTunnelStop"
                @tunnel-edit="handleTunnelEdit"
                @tunnel-delete="handleTunnelDelete"
                @refresh-tunnels="handleRefreshTunnels"
                @go-to-create="handleGoToCreateTunnel"
              />



            <!-- 设置 -->
            <Settings v-else-if="activeNav === 'settings'" />

            <!-- 帮助中心 -->
            <HelpCenter v-else-if="activeNav === 'help-center'" />

            <!-- 关于面板 -->
            <About v-else-if="activeNav === 'about'" />
            </div>
          </main>
        </template>
        </n-dialog-provider>
      </n-message-provider>
    </n-config-provider>
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body {
  height: 100%;
  overflow: hidden;
}

:root {
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  font-size: 14px;
  line-height: 1.5;
  color: #333;
  background-color: #f5f5f5;
}

.app-container {
  display: flex;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}

.loading-container {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100vh;
  width: 100vw;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.loading-spinner {
  text-align: center;
  color: white;
}

.loading-spinner i {
  font-size: 48px;
  margin-bottom: 20px;
  display: block;
}

.loading-spinner p {
  font-size: 16px;
  margin: 0;
  opacity: 0.9;
}

.login-fullscreen {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  z-index: 9999;
}

.main-content {
  width: calc(100vw - 250px);
  height: 100vh;
  display: flex;
  flex-direction: column;
  background-color: #101014;
  margin-left: 250px;
  overflow-y: auto;
}

.content-body {
  flex: 1;
  padding: 30px;
  background-color: #101014;
  min-height: calc(100vh - 60px);
}

.fa-spinner {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

/* 响应式设计 */
@media (max-width: 768px) {
  .content-body {
    padding: 20px;
  }
  
  .loading-spinner i {
    font-size: 36px;
  }
  
  .loading-spinner p {
    font-size: 14px;
  }
}
</style>