<template>
  <div class="sidebar">
    <n-config-provider :theme="customTheme">
      <div class="sidebar-header">
        <h2 class="app-title"> 
          <img src="../assets/icon.png" alt="logo" class="logo">
          ME-Frp
        </h2>
      </div>

      <div class="nav-content">
        <n-menu :options="menuOptions" :value="props.activeNav" @update:value="handleMenuSelect" />
      </div>

      <div class="sidebar-footer">
        <div class="logout-item" @click="handleNavClick('logout')">
          <i class="logout-icon fas fa-sign-out-alt"></i>
          <span class="logout-text">退出登录</span>
        </div>
      </div>
    </n-config-provider>
  </div>
</template>

<script setup lang="ts">
import { h } from 'vue'
import { darkTheme } from 'naive-ui'
import type { MenuOption } from 'naive-ui'

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

interface NavItem {
  id: string;
  name: string;
  icon: string;
}

interface Props {
  activeNav: string;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  'nav-change': [navId: string]
  'logout': []
}>();

// 导航项配置
const navItems: NavItem[] = [
  { id: 'dashboard', name: '面板首页', icon: 'fas fa-home' },
  { id: 'create-tunnel', name: '创建隧道', icon: 'fas fa-plus-circle' },
  { id: 'tunnel-management', name: '隧道管理', icon: 'fas fa-cogs' },
  { id: 'settings', name: '设置', icon: 'fas fa-cog' },
  { id: 'help-center', name: '帮助中心', icon: 'fas fa-question-circle' },
  { id: 'about', name: '关于面板', icon: 'fas fa-info-circle' }
];

// 创建菜单选项
const menuOptions: MenuOption[] = navItems.map(item => ({
  label: item.name,
  key: item.id,
  icon: () => h('i', { class: item.icon })
}))

function handleMenuSelect(key: string) {
  emit('nav-change', key)
}

function handleNavClick(navId: string) {
  if (navId === 'logout') {
    emit('logout');
  } else {
    emit('nav-change', navId);
  }
}
</script>

<style scoped>
.sidebar {
  width: 250px;
  background-color: #18181c;
  color: white;
  display: flex;
  flex-direction: column;
  height: 100vh;
  border-right: 1px solid #29292c;
  position: fixed;
  top: 0;
  left: 0;
  z-index: 1000;
}

.sidebar-header {
  padding: 20px;
  border-bottom: 1px solid #29292c;
  background-color: #18181c;
}

.app-title {
  font-size: 24px;
  font-weight: 600;
  margin: 0;
  color: #349ff4;
}

.nav-content {
  flex: 1;
  padding: 10px 0;
  background-color: #18181c;
}

.sidebar-footer {
  margin-top: auto;
  padding: 20px;
  border-top: 1px solid #29292c;
  background-color: #18181c;
}

.logo {
  width: 25px;
}

.logout-item {
  display: flex;
  align-items: center;
  padding: 12px 20px;
  cursor: pointer;
  transition: all 0.3s ease;
  border-radius: 6px;
  color: #e74c3c;
}

.logout-item:hover {
  background-color: rgba(231, 76, 60, 0.1);
}

.logout-icon {
  font-size: 16px;
  margin-right: 12px;
  width: 20px;
  text-align: center;
}

.logout-text {
  font-size: 14px;
  font-weight: 500;
}

/* 自定义Naive UI Menu样式 */
:deep(.n-menu) {
  background-color: transparent !important;
}

:deep(.n-menu .n-menu-item) {
  margin: 4px 12px;
  border-radius: 6px;
}

:deep(.n-menu .n-menu-item--selected) {
  background-color: #349ff4 !important;
  color: white !important;
}

:deep(.n-menu .n-menu-item:hover) {
  background-color: rgba(52, 159, 244, 0.1) !important;
}

:deep(.n-menu .n-menu-item-content) {
  padding: 12px 16px !important;
}

:deep(.n-menu .n-menu-item-content__icon) {
  margin-right: 12px !important;
  font-size: 16px !important;
}
</style>