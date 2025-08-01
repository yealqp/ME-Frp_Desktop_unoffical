<template>
  <div class="settings">
    <div class="settings-content">
      <n-card :bordered="true" class="settings-section">
        <template #header>
          <div class="section-header">
            <i class="fas fa-cog"></i>
            <span>应用设置</span>
          </div>
        </template>
        
        <n-space vertical :size="24">
          <!-- 开机自启动 -->
          <div class="setting-item">
            <div class="setting-info">
              <h4>开机自启动</h4>
              <p>开启后，系统启动时会自动运行ME-Frp客户端</p>
            </div>
            <n-switch v-model:value="settings.autoStart" @update:value="handleAutoStartChange" />
          </div>

          <!-- 窗口最前 -->
          <div class="setting-item">
            <div class="setting-info">
              <h4>窗口置顶</h4>
              <p>开启后，应用窗口将始终保持在最前面</p>
            </div>
            <n-switch v-model:value="settings.alwaysOnTop" @update:value="handleAlwaysOnTopChange" />
          </div>

          <!-- 检查更新 -->
          <div class="setting-item">
            <div class="setting-info">
              <h4>自动检查更新</h4>
              <p>开启后，应用启动时会自动检查是否有新版本</p>
            </div>
            <n-switch v-model:value="settings.autoUpdate" @update:value="handleAutoUpdateChange" />
          </div>

          <!-- 手动检查更新按钮 -->
          <div class="setting-item">
            <div class="setting-info">
              <h4>检查更新</h4>
              <p>立即检查是否有可用的更新</p>
            </div>
            <n-button type="primary" @click="() => checkForUpdates(true)" :loading="updateChecking">
              {{ updateChecking ? '检查中...' : '检查更新' }}
            </n-button>
          </div>
        </n-space>
      </n-card>


      <n-card :bordered="true" class="settings-section">
        <template #header>
          <div class="section-header">
            <i class="fas fa-rocket"></i>
            <span>隧道设置</span>
          </div>
        </template>
        
        <n-space vertical :size="24">
          <!-- 自启动隧道选择 -->
          <div class="setting-item tunnel-selection">
            <div class="setting-info">
              <h4>自启动隧道</h4>
              <p>选择应用启动时自动启动的隧道</p>
            </div>
            <div class="tunnel-controls" v-if="tunnels.length > 0">
              <n-space>
                <n-button size="small" @click="selectAllTunnels" :disabled="tunnelLoading">
                  全选
                </n-button>
                <n-button size="small" @click="clearAllTunnels" :disabled="tunnelLoading">
                  清空
                </n-button>
                <n-button size="small" @click="refreshTunnels" :loading="tunnelLoading">
                  刷新
                </n-button>
              </n-space>
            </div>
            <div class="tunnel-list">
              <div v-if="tunnelLoading" class="loading-text">加载隧道列表中...</div>
              <div v-else-if="tunnels.length === 0" class="empty-text">
                暂无隧道
                <n-button text type="primary" @click="refreshTunnels" style="margin-left: 8px;">
                  点击刷新
                </n-button>
              </div>
              <div v-else class="tunnel-items">
                <!-- 现有隧道 -->
                <div 
                  v-for="tunnel in tunnels" 
                  :key="tunnel.proxyId" 
                  class="tunnel-item"
                  :class="{ 'tunnel-disabled': tunnel.isDisabled }"
                >
                  <n-checkbox 
                     :checked="settings.autoStartTunnels.includes(tunnel.proxyId)"
                     @update:checked="(checked) => handleTunnelAutoStartChange(tunnel.proxyId, checked)"
                     :disabled="tunnel.isDisabled"
                   >
                    <div class="tunnel-info">
                      <div class="tunnel-header">
                        <span class="tunnel-name">{{ tunnel.proxyName }}</span>
                        <n-tag 
                          :type="tunnel.isDisabled ? 'default' : 'success'" 
                          size="small"
                          :bordered="false"
                        >
                          {{ tunnel.isDisabled ? '已禁用' : '正常' }}
                        </n-tag>
                      </div>
                      <div class="tunnel-details">
                        <span class="tunnel-id">ID: {{ tunnel.proxyId }}</span>
                        <span class="tunnel-type">{{ tunnel.proxyType.toUpperCase() }}</span>
                        <span class="tunnel-port">{{ tunnel.localPort }} → {{ tunnel.remotePort || '自动分配' }}</span>
                      </div>
                    </div>
                  </n-checkbox>
                </div>
                
                <!-- 已删除的隧道 -->
                <div 
                  v-for="deletedTunnelId in deletedTunnels" 
                  :key="`deleted-${deletedTunnelId}`" 
                  class="tunnel-item tunnel-deleted"
                >
                  <div class="deleted-tunnel-content">
                    <div class="tunnel-info">
                      <div class="tunnel-header">
                        <span class="tunnel-name">隧道 ID: {{ deletedTunnelId }}</span>
                        <n-tag 
                          type="error" 
                          size="small"
                          :bordered="false"
                        >
                          隧道已删除
                        </n-tag>
                      </div>
                      <div class="tunnel-details">
                        <span class="tunnel-id">此隧道在服务器上已不存在</span>
                      </div>
                    </div>
                    <n-button 
                      type="error" 
                      size="small" 
                      @click="removeDeletedTunnelConfig(deletedTunnelId)"
                      class="delete-config-btn"
                    >
                      点击删除配置
                    </n-button>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- 启动延迟 -->
          <div class="setting-item">
            <div class="setting-info">
              <h4>启动延迟</h4>
              <p>自启动隧道的延迟时间（秒）</p>
            </div>
            <n-input-number
              v-model:value="settings.startupDelay"
              :min="0"
              :max="60"
              :step="1"
              @update:value="handleStartupDelayChange"
            />
          </div>

          <!-- 启动顺序管理 -->
          <div class="setting-item tunnel-order" v-if="selectedTunnels.length > 1">
            <div class="setting-info">
              <h4>启动顺序</h4>
              <p>拖拽调整隧道的启动顺序，从上到下依次启动</p>
            </div>
            <div class="order-list">
              <div 
                v-for="(tunnelId, index) in settings.autoStartTunnels" 
                :key="tunnelId"
                class="order-item"
              >
                <div class="order-number">{{ index + 1 }}</div>
                <div class="order-tunnel-info">
                  <span class="order-tunnel-name">{{ getTunnelName(tunnelId) }}</span>
                  <span class="order-tunnel-id">ID: {{ tunnelId }}</span>
                </div>
                <div class="order-actions">
                  <n-button 
                    size="small" 
                    quaternary 
                    @click="moveTunnelUp(index)"
                    :disabled="index === 0"
                  >
                    <i class="fas fa-arrow-up"></i>
                  </n-button>
                  <n-button 
                    size="small" 
                    quaternary 
                    @click="moveTunnelDown(index)"
                    :disabled="index === settings.autoStartTunnels.length - 1"
                  >
                    <i class="fas fa-arrow-down"></i>
                  </n-button>
                </div>
              </div>
            </div>
          </div>
        </n-space>
      </n-card>

      <n-card :bordered="true" class="settings-section">
        <template #header>
          <div class="section-header">
            <i class="fas fa-palette"></i>
            <span>界面设置</span>
          </div>
        </template>
        
        <n-space vertical :size="24">
          <!-- 最小化到托盘 -->
          <div class="setting-item">
            <div class="setting-info">
              <h4>最小化到系统托盘</h4>
              <p>关闭窗口时最小化到系统托盘而不是退出应用</p>
            </div>
            <n-switch v-model:value="settings.minimizeToTray" @update:value="handleMinimizeToTrayChange" />
          </div>
        </n-space>
      </n-card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useMessage, useDialog, NCard, NSwitch, NButton, NInputNumber, NSpace, NCheckbox, NTag } from 'naive-ui';
import { invoke } from '@tauri-apps/api/core';

interface Settings {
  autoStart: boolean;
  alwaysOnTop: boolean;
  autoUpdate: boolean;
  autoStartTunnels: number[];
  startupDelay: number;
  theme: string;
  minimizeToTray: boolean;
}

interface TunnelOption {
  label: string;
  value: number;
}

interface Tunnel {
  proxyId: number;
  proxyName: string;
  proxyType: string;
  localIp: string;
  localPort: number;
  remotePort: number;
  nodeId: number;
  isDisabled: boolean;
}

interface UpdateCheckResult {
  has_update: boolean;
  latest_version: string;
  current_version: string;
}

const message = useMessage();
const dialog = useDialog();
const updateChecking = ref(false);

// 设置数据
const settings = ref<Settings>({
  autoStart: false,
  alwaysOnTop: false,
  autoUpdate: true,
  autoStartTunnels: [],
  startupDelay: 5,
  theme: 'dark',
  minimizeToTray: true
});

// 隧道数据
const tunnels = ref<Tunnel[]>([]);
const tunnelLoading = ref(false);

// 隧道选项（保留兼容性）
const tunnelOptions = ref<TunnelOption[]>([]);

// 计算属性：已选择的隧道
const selectedTunnels = computed(() => {
  return tunnels.value.filter(tunnel => 
    settings.value.autoStartTunnels.includes(tunnel.proxyId)
  );
});

// 计算属性：已删除的隧道（配置中存在但API中不存在）
const deletedTunnels = computed(() => {
  const existingTunnelIds = tunnels.value.map(tunnel => tunnel.proxyId);
  return settings.value.autoStartTunnels.filter(tunnelId => 
    !existingTunnelIds.includes(tunnelId)
  );
});
// 处理开机自启动变化
const handleAutoStartChange = async (value: boolean) => {
  try {
    await invoke('set_auto_start', { enable: value });
    message.success(value ? '已开启开机自启动' : '已关闭开机自启动');
    saveSettings();
  } catch (error) {
    message.error('设置开机自启动失败');
    settings.value.autoStart = !value; // 回滚
  }
};

// 处理窗口置顶变化
const handleAlwaysOnTopChange = async (value: boolean) => {
  try {
    await invoke('set_always_on_top', { alwaysOnTop: value });
    message.success(value ? '已开启窗口置顶' : '已关闭窗口置顶');
    saveSettings();
  } catch (error) {
    message.error('设置窗口置顶失败');
    settings.value.alwaysOnTop = !value; // 回滚
  }
};

// 处理自动更新变化
const handleAutoUpdateChange = (value: boolean) => {
  message.success(value ? '已开启自动检查更新' : '已关闭自动检查更新');
  saveSettings();
};



// 处理单个隧道自启动变化
const handleTunnelAutoStartChange = (proxyId: number, checked: boolean) => {
  if (checked) {
    if (!settings.value.autoStartTunnels.includes(proxyId)) {
      settings.value.autoStartTunnels.push(proxyId);
    }
  } else {
    const index = settings.value.autoStartTunnels.indexOf(proxyId);
    if (index > -1) {
      settings.value.autoStartTunnels.splice(index, 1);
    }
  }
  message.success('自启动隧道设置已更新');
  saveSettings();
};

// 处理启动延迟变化
const handleStartupDelayChange = (value: number | null) => {
  if (value !== null) {
    saveSettings();
  }
};



// 处理最小化到托盘变化
const handleMinimizeToTrayChange = async (value: boolean) => {
  try {
    await invoke('set_minimize_to_tray', { minimizeToTray: value });
    message.success(value ? '已开启最小化到托盘' : '已关闭最小化到托盘');
    saveSettings();
  } catch (error) {
    message.error('设置最小化到托盘失败');
    settings.value.minimizeToTray = !value; // 回滚
  }
};

// 检查更新
const checkForUpdates = async (showNoUpdateMessage = true) => {
  updateChecking.value = true;
  try {
    const result = await invoke('check_for_updates') as UpdateCheckResult;
    if (result.has_update) {
      // 弹窗询问用户是否要更新
      dialog.warning({
        title: '发现新版本',
        content: `发现新版本 ${result.latest_version}，当前版本 ${result.current_version}。更新时手动替换可执行文件即可，是否要立即更新？`,
        positiveText: '立即更新',
        negativeText: '稍后提醒',
        onPositiveClick: () => {
          // 这里可以添加实际的更新逻辑
          message.info('正在准备更新...');
          // 可以打开下载页面或执行更新程序
          window.open('https://alist.yealqp.cn/mefrp-desktop', '_blank');
        },
        onNegativeClick: () => {
          message.info('已取消更新，下次启动时会再次检查');
        }
      });
    } else {
      if (showNoUpdateMessage) {
        message.success(`当前已是最新版本 ${result.current_version}`);
      }
    }
  } catch (error) {
    if (showNoUpdateMessage) {
      message.error(`检查更新失败: ${error}`);
    }
  } finally {
    updateChecking.value = false;
  }
};

// 启动时自动检查更新
const autoCheckForUpdates = async () => {
  if (settings.value.autoUpdate) {
    // 延迟3秒后检查更新，避免启动时过于繁忙
    setTimeout(() => {
      checkForUpdates(false); // 不显示"已是最新版本"的消息
    }, 3000);
  }
};

// 保存设置
const saveSettings = async () => {
  try {
    await invoke('save_settings', { settings: settings.value });
  } catch (error) {
    console.error('保存设置失败:', error);
  }
};

// 加载设置
const loadSettings = async () => {
  try {
    const savedSettings = await invoke('load_settings');
    if (savedSettings) {
      settings.value = { ...settings.value, ...savedSettings };
    }
    
    // 同步最小化到托盘设置到后端
    try {
      await invoke('set_minimize_to_tray', { minimizeToTray: settings.value.minimizeToTray });
    } catch (error) {
      console.error('同步最小化到托盘设置失败:', error);
    }
  } catch (error) {
    console.error('加载设置失败:', error);
  }
};

// 加载隧道列表
const loadTunnels = async () => {
  tunnelLoading.value = true;
  try {
    const responseText = await invoke('api_get_tunnel_list');
    const result = JSON.parse(responseText as string);
    
    if (result.code === 200 && Array.isArray(result.data)) {
      tunnels.value = result.data;
      // 更新隧道选项（保留兼容性）
       tunnelOptions.value = result.data.map((tunnel: Tunnel) => ({
         label: `${tunnel.proxyName} (ID: ${tunnel.proxyId})`,
         value: tunnel.proxyId
       }));
      console.log(`成功加载 ${result.data.length} 个隧道`);
    } else {
      console.error('获取隧道列表失败:', result.message);
      message.error(result.message || '获取隧道列表失败');
    }
  } catch (error) {
    console.error('加载隧道列表失败:', error);
    message.error('加载隧道列表失败，请检查网络连接');
  } finally {
    tunnelLoading.value = false;
  }
};

// 刷新隧道列表
const refreshTunnels = async () => {
  await loadTunnels();
  message.success('隧道列表已刷新');
};

// 全选隧道
const selectAllTunnels = () => {
  const enabledTunnels = tunnels.value.filter(tunnel => !tunnel.isDisabled);
  const allEnabledIds = enabledTunnels.map(tunnel => tunnel.proxyId);
  
  // 合并现有的已选择隧道和所有可用隧道
  const newSelection = [...new Set([...settings.value.autoStartTunnels, ...allEnabledIds])];
  settings.value.autoStartTunnels = newSelection;
  
  message.success(`已选择 ${enabledTunnels.length} 个可用隧道`);
  saveSettings();
};

// 清空所有选择
const clearAllTunnels = () => {
  settings.value.autoStartTunnels = [];
  message.success('已清空所有自启动隧道选择');
  saveSettings();
};

// 获取隧道名称
const getTunnelName = (tunnelId: number): string => {
  const tunnel = tunnels.value.find(t => t.proxyId === tunnelId);
  return tunnel ? tunnel.proxyName : `隧道 ${tunnelId}`;
};

// 向上移动隧道
const moveTunnelUp = (index: number) => {
  if (index > 0) {
    const tunnels = [...settings.value.autoStartTunnels];
    [tunnels[index], tunnels[index - 1]] = [tunnels[index - 1], tunnels[index]];
    settings.value.autoStartTunnels = tunnels;
    message.success('启动顺序已调整');
    saveSettings();
  }
};

// 向下移动隧道
const moveTunnelDown = (index: number) => {
  if (index < settings.value.autoStartTunnels.length - 1) {
    const tunnels = [...settings.value.autoStartTunnels];
    [tunnels[index], tunnels[index + 1]] = [tunnels[index + 1], tunnels[index]];
    settings.value.autoStartTunnels = tunnels;
    message.success('启动顺序已调整');
    saveSettings();
  }
};

// 删除已删除隧道的配置
const removeDeletedTunnelConfig = (tunnelId: number) => {
  const index = settings.value.autoStartTunnels.indexOf(tunnelId);
  if (index > -1) {
    settings.value.autoStartTunnels.splice(index, 1);
    message.success(`已删除隧道 ${tunnelId} 的自启动配置`);
    saveSettings();
  }
};

onMounted(() => {
  loadSettings();
  loadTunnels();
  autoCheckForUpdates();
});
</script>

<style scoped>
.settings {
  padding: 20px;
  max-width: 800px;
  margin: 0 auto;
}

.settings-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.settings-section {
  background: #18181c;
  border: 1px solid #29292c;
}

.section-header {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 16px;
  font-weight: 600;
  color: #ffffff;
}

.section-header i {
  color: #349ff4;
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 0;
  border-bottom: 1px solid #29292c;
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-info {
  flex: 1;
}

.setting-info h4 {
  margin: 0 0 4px 0;
  font-size: 14px;
  font-weight: 500;
  color: #ffffff;
}

.setting-info p {
  margin: 0;
  font-size: 12px;
  color: #a0a0a0;
  line-height: 1.4;
}

/* 隧道选择相关样式 */
.tunnel-selection {
  flex-direction: column;
  align-items: flex-start;
  gap: 12px;
}

.tunnel-controls {
  width: 100%;
  padding: 8px 0;
  border-bottom: 1px solid #29292c;
}

.tunnel-list {
  width: 100%;
  max-height: 400px;
  overflow-y: auto;
  border: 1px solid #29292c;
  border-radius: 6px;
  background: #1a1a1e;
}

.loading-text, .empty-text {
  color: #a0a0a0;
  font-size: 14px;
  padding: 20px;
  text-align: center;
}

.tunnel-items {
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.tunnel-item {
  padding: 12px 16px;
  border-bottom: 1px solid #29292c;
  background: #1a1a1e;
  transition: all 0.2s ease;
}

.tunnel-item:last-child {
  border-bottom: none;
}

.tunnel-item:hover {
  background: #1e1e22;
}

.tunnel-item.tunnel-disabled {
  opacity: 0.6;
  background: #161619;
}

.tunnel-item.tunnel-disabled:hover {
  background: #191919;
}

.tunnel-item.tunnel-deleted {
  background: #2a1a1a;
  border: 1px solid #5c2626;
  opacity: 0.9;
}

.tunnel-item.tunnel-deleted:hover {
  background: #2d1d1d;
}

.deleted-tunnel-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}

.delete-config-btn {
  flex-shrink: 0;
  margin-left: 12px;
}

.tunnel-info {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-left: 8px;
  width: 100%;
}

.tunnel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.tunnel-name {
  color: #ffffff;
  font-size: 14px;
  font-weight: 500;
}

.tunnel-details {
  display: flex;
  gap: 12px;
  align-items: center;
}

.tunnel-id, .tunnel-type, .tunnel-port {
  color: #a0a0a0;
  font-size: 12px;
}

.tunnel-type {
  background: #29292c;
  padding: 2px 6px;
  border-radius: 3px;
  font-weight: 500;
}

.tunnel-port {
  font-family: 'Courier New', monospace;
}

/* 启动顺序管理样式 */
.tunnel-order {
  flex-direction: column;
  align-items: flex-start;
  gap: 12px;
}

.order-list {
  width: 100%;
  border: 1px solid #29292c;
  border-radius: 6px;
  background: #1a1a1e;
  overflow: hidden;
}

.order-item {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid #29292c;
  background: #1a1a1e;
  transition: all 0.2s ease;
}

.order-item:last-child {
  border-bottom: none;
}

.order-item:hover {
  background: #1e1e22;
}

.order-number {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: #349ff4;
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  font-size: 14px;
  margin-right: 12px;
  flex-shrink: 0;
}

.order-tunnel-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.order-tunnel-name {
  color: #ffffff;
  font-size: 14px;
  font-weight: 500;
}

.order-tunnel-id {
  color: #a0a0a0;
  font-size: 12px;
}

.order-actions {
  display: flex;
  gap: 4px;
  flex-shrink: 0;
}

.order-actions .n-button {
  width: 32px;
  height: 32px;
}

@media (max-width: 768px) {
  .settings {
    padding: 10px;
  }
  
  .setting-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }
  
  .tunnel-list {
    max-height: 200px;
  }
}
</style>