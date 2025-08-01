<template>
  <div class="login-container">
    <n-config-provider :theme="customTheme">
      <n-card class="login-card" :bordered="true">
        <!-- 登录标题 -->
        <h1 class="login-title">登录到ME-Frp</h1>

        <!-- 登录状态提示已移除，使用message组件 -->

        <!-- 登录表单 -->
        <n-form ref="formRef" :model="loginForm" :rules="rules" @submit.prevent="handleLogin">
          <n-form-item path="username">
            <n-input v-model:value="loginForm.username" placeholder="用户名/邮箱" size="large" :disabled="isLogging" />
          </n-form-item>

          <n-form-item path="password">
            <n-input v-model:value="loginForm.password" type="password" placeholder="密码" size="large"
              :disabled="isLogging" show-password-on="mousedown" />
          </n-form-item>



          <n-button type="primary" size="large" block :loading="isLogging"
            :disabled="!loginForm.username || !loginForm.password" @click="handleLogin" class="login-btn">
            {{ isLogging ? '登录中...' : '登录' }}
          </n-button>
        </n-form>
      </n-card>
    </n-config-provider>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { darkTheme, useMessage } from 'naive-ui'

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

const emit = defineEmits(['login-success'])
const message = useMessage()

// 登录表单数据
const loginForm = ref({
  username: '',
  password: ''
})



// 表单引用
const formRef = ref(null)

// 表单验证规则
const rules = {
  username: {
    required: true,
    message: '请输入用户名',
    trigger: 'blur'
  },
  password: {
    required: true,
    message: '请输入密码',
    trigger: 'blur'
  }
}

// 登录状态
const isLogging = ref(false)

// 处理登录
async function handleLogin() {
  if (isLogging.value) return

  isLogging.value = true
  message.loading('正在登录中，请稍候...')

  try {
    console.log('开始登录:', loginForm.value.username)

    // 调用后端登录API命令
    const config = await invoke('api_login', {
      username: loginForm.value.username,
      password: loginForm.value.password
    })

    console.log('登录成功，配置已保存:', config)

    message.success('登录成功，正在进入主界面...')

    // 延迟1秒后跳转
    setTimeout(() => {
      console.log('触发login-success事件')
      emit('login-success')
    }, 1000)

  } catch (error) {
    console.error('登录失败:', error)
    // 显示完整的错误信息
    const errorMessage = error && typeof error === 'string' ? error :
      error && typeof error === 'object' && 'message' in error ?
        (error as any).message : '登录失败，请检查用户名和密码';
    message.error(errorMessage)
  } finally {
    isLogging.value = false
  }
}

onMounted(async () => {
  // 清除之前的登录状态，确保重新登录
  localStorage.removeItem('mefrp_config')

  // 同时清除config.yaml文件（使用Rust后端）
  try {
    const result = await invoke('clear_config')
    console.log('已清除config.yaml文件:', result)
  } catch (error) {
    console.log('清除config.yaml文件时出错:', error)
  }

  console.log('已清除登录状态，准备重新登录')
})
</script>

<style scoped>
.login-container {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #101014;
  padding: 20px;
}

.login-card {
  width: 100%;
  max-width: 400px;
  padding: 40px;
  animation: slideUp 0.6s ease-out;
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(30px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.login-title {
  text-align: center;
  color: #349ff4;
  margin-bottom: 30px;
  font-size: 24px;
  font-weight: 600;
}

.login-btn {
  background-color: #101014 !important;
  color: #349ff4 !important;
  border: none !important;
}

.login-btn:hover {
  background-color: #1a1a1e !important;
  color: #4da8f5 !important;
}

.login-btn:focus {
  background-color: #101014 !important;
  color: #349ff4 !important;
}

@media (max-width: 480px) {
  .login-card {
    padding: 30px 20px;
    margin: 10px;
  }

  .login-title {
    font-size: 20px;
    margin-bottom: 20px;
  }
}
</style>