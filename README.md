# ME-Frp Desktop Client (Unofficial) | ME-Frp 桌面客户端（非官方）

[English](#english) | [中文](#中文)

---

## English

### 📖 Description

An unofficial desktop client for ME-Frp service, built with modern web technologies. This application provides a user-friendly interface for managing ME-Frp tunnels, monitoring node status, and configuring network forwarding rules.

### ✨ Features

- 🖥️ **Cross-platform Desktop App** - Built with Tauri for Windows, macOS, and Linux
- 🎯 **Node Management** - Browse and select available ME-Frp nodes
- 📊 **Real-time Monitoring** - Monitor node status, load, and bandwidth
- 🔧 **Tunnel Configuration** - Create and manage network tunnels
- 📈 **Dashboard** - View user information and system announcements
- 🎨 **Modern UI** - Clean and intuitive interface with Naive UI components
- 🌐 **API Integration** - Direct integration with ME-Frp official API

### 🛠️ Tech Stack

- **Frontend Framework**: Vue 3 with Composition API
- **Desktop Framework**: Tauri 2.x
- **Language**: TypeScript
- **UI Library**: Naive UI
- **Build Tool**: Vite
- **Backend**: Rust (Tauri)

### 📋 Prerequisites

- Node.js (v16 or higher)
- Rust (latest stable)
- npm or yarn

### 🚀 Installation & Development

1. **Clone the repository**
   ```bash
   git clone https://github.com/yealqp/ME-Frp_Desktop_unoffical
   cd ME-Frp_Desktop_unoffical
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Run development server**
   ```bash
   npm run tauri dev
   ```

4. **Build for production**
   ```bash
   npm run tauri build
   ```

### 📁 Project Structure

```
ME-Frp_Desktop_unoffical/
├── src/                    # Vue.js source code
│   ├── components/         # Vue components
│   ├── assets/            # Static assets
│   └── main.ts            # Application entry point
├── src-tauri/             # Tauri backend (Rust)
│   ├── src/               # Rust source code
│   └── tauri.conf.json    # Tauri configuration
├── package.json           # Node.js dependencies
└── README.md             # This file
```

### 🔧 Available Scripts

- `npm run dev` - Start Vite development server
- `npm run build` - Build Vue.js application
- `npm run preview` - Preview built application
- `npm run tauri dev` - Start Tauri development mode
- `npm run tauri build` - Build Tauri application

### 📝 License

This project is licensed under the GPL 3.0 License.

### ⚠️ Disclaimer

This is an unofficial client for ME-Frp service. It is not affiliated with or endorsed by the official ME-Frp team.

---

## 中文

### 📖 项目简介

ME-Frp 服务的非官方桌面客户端，采用现代化 Web 技术构建。本应用提供友好的用户界面，用于管理 ME-Frp 隧道、监控节点状态和配置网络转发规则。

### ✨ 功能特性

- 🖥️ **跨平台桌面应用** - 基于 Tauri 构建，支持 Windows、macOS 和 Linux
- 🎯 **节点管理** - 浏览和选择可用的 ME-Frp 节点
- 📊 **实时监控** - 监控节点状态、负载和带宽
- 🔧 **隧道配置** - 创建和管理网络隧道
- 📈 **仪表板** - 查看用户信息和系统公告
- 🎨 **现代化界面** - 基于 Naive UI 的简洁直观界面
- 🌐 **API 集成** - 直接集成 ME-Frp 官方 API

### 🛠️ 技术栈

- **前端框架**: Vue 3 + Composition API
- **桌面框架**: Tauri 2.x
- **开发语言**: TypeScript
- **UI 组件库**: Naive UI
- **构建工具**: Vite
- **后端**: Rust (Tauri)

### 📋 环境要求

- Node.js (v16 或更高版本)
- Rust (最新稳定版)
- npm 或 yarn

### 🚀 安装与开发

1. **克隆仓库**
   ```bash
   git clone https://github.com/yealqp/ME-Frp_Desktop_unoffical
   cd ME-Frp_Desktop_unoffical
   ```

2. **安装依赖**
   ```bash
   npm install
   ```

3. **启动开发服务器**
   ```bash
   npm run tauri dev
   ```

4. **构建生产版本**
   ```bash
   npm run tauri build
   ```

### 📁 项目结构

```
ME-Frp_Desktop_unoffical/
├── src/                    # Vue.js 源代码
│   ├── components/         # Vue 组件
│   ├── assets/            # 静态资源
│   └── main.ts            # 应用入口文件
├── src-tauri/             # Tauri 后端 (Rust)
│   ├── src/               # Rust 源代码
│   └── tauri.conf.json    # Tauri 配置文件
├── package.json           # Node.js 依赖配置
└── README.md             # 说明文档
```

### 🔧 可用脚本

- `npm run dev` - 启动 Vite 开发服务器
- `npm run build` - 构建 Vue.js 应用
- `npm run preview` - 预览构建后的应用
- `npm run tauri dev` - 启动 Tauri 开发模式
- `npm run tauri build` - 构建 Tauri 应用

### 📝 开源协议

本项目采用 GPL 3.0 协议开源。

### ⚠️ 免责声明

这是 ME-Frp 服务的非官方客户端，与 ME-Frp 官方团队无关，未获得官方认可或支持。
