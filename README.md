# DeskHive

一个占用极低的桌面TODO插件，随时随地不忘事！

![image-20250915124406296](other/pic/deskhive.png)

交流与建议：

<img src="other/pic/wx.png" alt="image-20250907030349657" style="zoom:25%;" />

[中文](#中文) | [English](#english)

## 中文

DeskHive 是一个基于 Vue 3 和 Tauri 构建的轻量级、跨平台的桌面待办事项管理工具，旨在提供无需云端服务、数据本地存储、界面简洁高效的待办事项管理体验。

### 技术栈

- **前端**: Vue 3 + TypeScript + Vite
- **后端**: Rust (Tauri 框架)
- **构建工具**: Cargo (Rust) + npm (Node.js)
- **UI框架**: 自定义CSS响应式设计
- **数据存储**: 本地文件系统

### 功能特性

- 本地数据存储，无网络依赖
- 跨平台桌面应用（支持 Windows）
- 响应式 UI，支持拖拽、弹窗提示等交互
- 待办事项管理（添加、删除、标记完成任务）
- 任务分类与状态管理
- 截止时间跟踪与可视化指示器
- 系统托盘集成与快捷操作
- 窗口控制（透明度、位置管理）
- 设置管理（自动启动、窗口位置等）

### 使用方式

- **添加任务**：在底部输入框中输入任务内容，按回车或点击"+"按钮添加
- **完成任务**：点击任务前面的圆形复选框将任务标记为完成
- **删除任务**：双击任务项可直接删除任务
- **设置截止时间**：右键点击任务项，选择"📅 设置截止时间"，在弹出的对话框中选择日期和时间
- **移除截止时间**：对于已有截止时间的任务，右键点击任务项，选择"🗑️ 移除截止时间"
- **查看任务详情**：右键点击任务项可查看任务状态、创建时间和截止时间等详细信息

### 环境要求

- Node.js（版本 ^20.19.0 或 >=22.12.0）
- npm 或 pnpm
- Rust 工具链（rustup, cargo）

### 开发环境搭建

1. 克隆仓库：
   ```bash
   git clone <repository-url>
   cd DeskHive
   ```

2. 安装依赖：
   ```bash
   npm install
   ```

3. 运行开发环境：
   ```bash
   npm run dev
   ```

### 生产环境构建

1. 构建应用程序：
   ```bash
   npm run tauri build
   ```

2. 构建产物位置：
   - Windows: `src-tauri/target/release/bundle/nsis/` (`.exe`)
   - macOS: `src-tauri/target/release/bundle/macos/` (`.app`)
   - Linux: `src-tauri/target/release/bundle/appimage/` (`.AppImage`)

## English

DeskHive is a lightweight, cross-platform desktop to-do management tool built with Vue 3 and Tauri, designed to provide a local task management experience without cloud dependencies.

### Technology Stack

- **Frontend**: Vue 3 + TypeScript + Vite
- **Backend**: Rust (Tauri Framework)
- **Build Tool**: Cargo (Rust) + npm (Node.js)
- **UI Framework**: Custom CSS with responsive design
- **Data Storage**: Local file system

### Features

- Local data storage with no network dependencies
- Cross-platform support (Windows)
- Responsive UI with drag-and-drop and popup notifications
- Task management (add, delete, mark as complete)
- Task categorization and status management
- Deadline tracking with visual indicators
- System tray integration and quick actions
- Window control (transparency, position management)
- Settings management (auto-start, window position, etc.)

### Usage

- **Add tasks**: Enter task content in the bottom input box, press Enter or click the "+" button to add
- **Complete tasks**: Click the circular checkbox in front of the task to mark it as completed
- **Delete tasks**: Double-click on a task item to delete it directly
- **Set deadline**: Right-click on a task item, select "📅 Set Deadline", and choose date and time in the popup dialog
- **Remove deadline**: For tasks with existing deadlines, right-click on the task item and select "🗑️ Remove Deadline"
- **View task details**: Right-click on a task item to view detailed information such as task status, creation time, and deadline

### Prerequisites

- Node.js (version ^20.19.0 or >=22.12.0)
- npm or pnpm
- Rust toolchain (rustup, cargo)

### Development Setup

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd DeskHive
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Run in development mode:
   ```bash
   npm run dev
   ```

### Building for Production

1. Build the application:
   ```bash
   npm run tauri build
   ```

2. The built application will be located in:
   - Windows: `src-tauri/target/release/bundle/nsis/` (`.exe`)
   - macOS: `src-tauri/target/release/bundle/macos/` (`.app`)
   - Linux: `src-tauri/target/release/bundle/appimage/` (`.AppImage`)

